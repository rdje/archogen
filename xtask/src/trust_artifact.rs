//! Bind a role artifact to metadata and an observed compilation, independently of Cargo stdout.

use super::{Compilation, Digest, Program, Target, TargetSources};
use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::{Command, Stdio},
};

/// Construct a source-free naming query. Only these observed options can affect output names.
fn naming_arguments(arguments: &[String]) -> Result<(Vec<String>, String), String> {
    let mut query = vec!["--print=file-names".to_owned(), "-".to_owned()];
    let mut seen = BTreeSet::new();
    let mut output = None;
    for (index, argument) in arguments.iter().enumerate().skip(1) {
        match argument.as_str() {
            "--crate-name" | "--crate-type" | "--target" | "--out-dir" => {
                let value = arguments
                    .get(index + 1)
                    .filter(|value| !value.is_empty() && !value.starts_with('-'))
                    .ok_or_else(|| format!("missing observed {argument} value"))?;
                if argument != "--crate-type" && !seen.insert(argument.as_str()) {
                    return Err(format!("duplicate observed {argument}"));
                }
                if argument == "--out-dir" {
                    output = Some(value.clone());
                } else {
                    query.extend([argument.clone(), value.clone()]);
                }
            }
            "--test" => {
                if !seen.insert("--test") {
                    return Err("duplicate observed --test".to_owned());
                }
                query.push(argument.clone());
            }
            "-C" => {
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| "missing observed -C value".to_owned())?;
                if value.starts_with("extra-filename=") {
                    if !seen.insert("extra-filename") {
                        return Err("duplicate observed extra-filename".to_owned());
                    }
                    query.extend([argument.clone(), value.clone()]);
                }
            }
            value
                if [
                    "--crate-name=",
                    "--crate-type=",
                    "--target=",
                    "--out-dir=",
                    "-Cextra-filename=",
                ]
                .iter()
                .any(|prefix| value.starts_with(prefix)) =>
            {
                return Err(format!("unsupported observed naming option {value}"));
            }
            _ => {}
        }
    }
    if !seen.contains("--crate-name") {
        return Err("missing observed --crate-name".to_owned());
    }
    Ok((query, output.ok_or("missing observed --out-dir")?))
}

/// Validate every returned output, then select exactly one executable or rlib.
fn outputs(
    stdout: &[u8],
    directory: &Path,
    target: &Path,
    library: bool,
) -> Result<(String, String), String> {
    let text = std::str::from_utf8(stdout).map_err(|_| "non-UTF8 compiler artifact names")?;
    let text = text
        .strip_suffix('\n')
        .ok_or("incomplete compiler artifact names")?;
    let root = fs::canonicalize(target).map_err(|error| error.to_string())?;
    let out = fs::canonicalize(directory).map_err(|error| error.to_string())?;
    if !out.starts_with(&root) {
        return Err("compiler artifact directory is outside the role target".to_owned());
    }
    let mut names = BTreeSet::new();
    let mut artifacts = Vec::new();
    for name in text.split('\n') {
        if name.is_empty()
            || matches!(name, "." | "..")
            || name.contains(['/', '\\', '\r', '\0'])
            || !names.insert(name)
        {
            return Err("invalid or duplicate compiler artifact basename".to_owned());
        }
        let file = out.join(name);
        let metadata = fs::symlink_metadata(&file).map_err(|error| format!("{name}: {error}"))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "compiler artifact {name} is not a regular non-symlink file"
            ));
        }
        let bytes = fs::read(&file).map_err(|error| format!("{name}: {error}"))?;
        if !library || name.ends_with(".rlib") {
            let relative = file
                .strip_prefix(&root)
                .map_err(|error| error.to_string())?;
            artifacts.push((
                relative
                    .to_str()
                    .ok_or("non-UTF8 compiler artifact path")?
                    .to_owned(),
                Digest::of(&bytes).hex(),
            ));
        }
    }
    match artifacts.as_slice() {
        [one] => Ok(one.clone()),
        _ => Err("compiler artifact selection is missing or ambiguous".to_owned()),
    }
}

pub(super) fn select(
    program: &Program,
    sources: &TargetSources,
    compilations: &[Compilation],
    env: &[(String, String)],
    tree: &Path,
    target: &Path,
) -> Result<(String, String), String> {
    let roots: BTreeSet<&String> = sources
        .iter()
        .filter(|((package, kind, name), _)| {
            package == &program.package
                && match &program.target {
                    Target::Bin(want) => kind == "bin" && name == want,
                    Target::Test(want) => kind == "test" && name == want,
                    Target::Lib => kind.ends_with("lib"),
                }
        })
        .flat_map(|(_, roots)| roots)
        .collect();
    let roots: Vec<_> = roots.into_iter().collect();
    let [source] = roots.as_slice() else {
        return Err(format!(
            "`{}` has missing or ambiguous metadata artifact sources",
            program.name
        ));
    };
    let root = fs::canonicalize(tree).map_err(|error| error.to_string())?;
    let invocations: Vec<_> = compilations
        .iter()
        .filter(|compilation| {
            compilation.package == program.package
                && compilation.arguments.iter().skip(1).any(|argument| {
                    let path = Path::new(argument);
                    path == Path::new(source)
                        || path.strip_prefix(tree).ok() == Some(Path::new(source))
                        || path.strip_prefix(&root).ok() == Some(Path::new(source))
                })
        })
        .collect();
    let [compilation] = invocations.as_slice() else {
        return Err(format!(
            "`{}` has missing or ambiguous observed artifact compilations",
            program.name
        ));
    };
    let (query, directory) = naming_arguments(&compilation.arguments)?;
    let result = Command::new("rustc")
        .args(query)
        .current_dir(tree)
        .env_clear()
        .envs(env.iter().map(|(key, value)| (key, value)))
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("compiler artifact naming query: {error}"))?;
    if !result.status.success() {
        return Err(format!(
            "compiler artifact naming query failed: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }
    let directory = Path::new(&directory);
    let directory = if directory.is_absolute() {
        directory.to_owned()
    } else {
        tree.join(directory)
    };
    outputs(
        &result.stdout,
        &directory,
        target,
        program.target == Target::Lib,
    )
}

#[cfg(test)]
mod tests {
    use super::{naming_arguments, outputs};
    use crate::trust::tests::real_root;
    use std::{fs, os::unix::fs::symlink};

    #[test]
    fn artifact_names_refuse_missing_foreign_linked_and_ambiguous_outputs() {
        crate::trust_test_process::bounded!(
            artifact_names_refuse_missing_foreign_linked_and_ambiguous_outputs
        );
        let base = real_root().join("target/trust-tests/artifact-output-controls");
        let _ = fs::remove_dir_all(&base);
        let target = base.join("target");
        let out = target.join("release/deps");
        let foreign = base.join("foreign");
        fs::create_dir_all(&out).unwrap();
        fs::create_dir_all(&foreign).unwrap();
        fs::write(out.join("root"), b"actual executable bytes").unwrap();
        fs::write(out.join("libroot.rlib"), b"!<arch>\n").unwrap();
        fs::write(foreign.join("root"), b"foreign bytes").unwrap();
        let good = || {
            let executable = outputs(b"root\n", &out, &target, false).unwrap();
            assert_eq!(executable.0, "release/deps/root");
            assert_eq!(
                executable.1,
                "064d7f281f05977e9f8b0f34f693e69bac17a13f1640d4decac712ea3128329f"
            );
            assert_eq!(
                outputs(b"libroot.rlib\n", &out, &target, true).unwrap().0,
                "release/deps/libroot.rlib"
            );
        };
        for names in [
            &b""[..],
            b"root",
            b"\n",
            b".\n",
            b"..\n",
            b"../root\n",
            b"a/root\n",
            b"a\\root\n",
            b"root\r\n",
            b"root\0\n",
            b"\xff\n",
            b"missing\n",
            b"root\nroot\n",
            b"root\nlibroot.rlib\n",
        ] {
            good();
            assert!(outputs(names, &out, &target, false).is_err(), "{names:?}");
        }
        good();
        assert!(outputs(b"root\n", &foreign, &target, false)
            .unwrap_err()
            .contains("outside"));
        symlink("root", out.join("link")).unwrap();
        assert!(outputs(b"link\n", &out, &target, false)
            .unwrap_err()
            .contains("non-symlink"));
        fs::create_dir(out.join("directory")).unwrap();
        assert!(outputs(b"directory\n", &out, &target, false).is_err());
        assert!(outputs(b"root\n", &out, &target, true).is_err());
        fs::write(out.join("libother.rlib"), b"other library").unwrap();
        assert!(outputs(b"libroot.rlib\nlibother.rlib\n", &out, &target, true).is_err());
        good();
    }

    #[test]
    fn naming_query_excludes_source_codegen_and_environment_inputs() {
        let observed: Vec<String> = [
            "rustc",
            "--crate-name",
            "root",
            "source.rs",
            "--crate-type",
            "rlib",
            "--crate-type",
            "dylib",
            "--target",
            "host",
            "--out-dir",
            "outputs",
            "-C",
            "opt-level=3",
            "-C",
            "extra-filename=-hash",
            "--cfg",
            "feature=\"x\"",
            "--extern",
            "other=source.rlib",
            "-L",
            "dependency=somewhere",
        ]
        .map(str::to_owned)
        .to_vec();
        let (query, out) = naming_arguments(&observed).unwrap();
        assert_eq!(out, "outputs");
        assert_eq!(
            query,
            [
                "--print=file-names",
                "-",
                "--crate-name",
                "root",
                "--crate-type",
                "rlib",
                "--crate-type",
                "dylib",
                "--target",
                "host",
                "-C",
                "extra-filename=-hash"
            ]
        );
        for tail in [
            vec!["--crate-name", "other"],
            vec!["--out-dir", "elsewhere"],
            vec!["--target", "other"],
            vec!["-C", "extra-filename=-other"],
            vec!["--crate-name=joined"],
            vec!["--out-dir"],
            vec!["-C"],
        ] {
            let mut bad = observed.clone();
            bad.extend(tail.into_iter().map(str::to_owned));
            assert!(naming_arguments(&bad).is_err(), "{bad:?}");
        }
        assert!(
            naming_arguments(&["rustc".to_owned(), "--out-dir".to_owned(), "out".to_owned()])
                .is_err()
        );
    }
    #[test]
    fn actual_empty_input_query_preserves_build_and_ignores_now_invalid_source() {
        crate::trust_test_process::bounded!(
            actual_empty_input_query_preserves_build_and_ignores_now_invalid_source
        );
        use super::{select, Compilation, Program, Target, TargetSources};
        use crate::catalog_build::environment;
        use std::{collections::BTreeSet, process::Command};
        let base = real_root().join("target/trust-tests/artifact-empty-query");
        let _ = fs::remove_dir_all(&base);
        let tree = base.join("tree");
        let target = base.join("target");
        let out = target.join("release/deps");
        fs::create_dir_all(&tree).unwrap();
        fs::create_dir_all(&out).unwrap();
        fs::write(
            tree.join("source.rs"),
            "fn main() { println!(\"query-artifact\"); }\n",
        )
        .unwrap();
        let env = environment("1.95.0", &base.join("cargo-home"), &target).unwrap();
        let arguments = vec![
            "rustc".to_owned(),
            "--crate-name".into(),
            "probe".into(),
            "source.rs".into(),
            "--crate-type".into(),
            "bin".into(),
            "--out-dir".into(),
            out.to_str().unwrap().to_owned(),
            "-C".into(),
            "extra-filename=-actual".into(),
        ];
        let result = Command::new("rustc")
            .args(&arguments[1..])
            .current_dir(&tree)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .output()
            .unwrap();
        assert!(result.status.success(), "{:?}", result.stderr);
        let artifact = out.join("probe-actual");
        let before = fs::read(&artifact).unwrap();
        let before_names: BTreeSet<_> = fs::read_dir(&out)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        fs::write(
            tree.join("source.rs"),
            "compile_error!(\"must never be evaluated by the naming query\");\n",
        )
        .unwrap();
        let source = fs::read(tree.join("source.rs")).unwrap();
        let program = Program {
            name: "root".into(),
            role: "generator".into(),
            package: "p".into(),
            target: Target::Bin("probe".into()),
            role_packages: BTreeSet::new(),
            pair: None,
            data: Vec::new(),
        };
        let sources = TargetSources::from([(
            ("p".into(), "bin".into(), "probe".into()),
            BTreeSet::from(["source.rs".into()]),
        )]);
        let compilations = vec![Compilation {
            package: "p".into(),
            arguments: arguments.clone(),
        }];
        let good = || {
            let selected = select(&program, &sources, &compilations, &env, &tree, &target).unwrap();
            assert_eq!(selected.0, "release/deps/probe-actual");
            assert_eq!(fs::read(&artifact).unwrap(), before);
            assert_eq!(fs::read(tree.join("source.rs")).unwrap(), source);
            assert_eq!(
                fs::read_dir(&out)
                    .unwrap()
                    .map(|entry| entry.unwrap().file_name())
                    .collect::<BTreeSet<_>>(),
                before_names
            );
            let result = Command::new(&artifact).output().unwrap();
            assert!(result.status.success());
            assert_eq!(result.stdout, b"query-artifact\n");
        };
        good();
        for bad_sources in [
            TargetSources::new(),
            TargetSources::from([(
                ("p".into(), "bin".into(), "probe".into()),
                BTreeSet::from(["other.rs".into()]),
            )]),
            TargetSources::from([(
                ("p".into(), "bin".into(), "probe".into()),
                BTreeSet::from(["source.rs".into(), "other.rs".into()]),
            )]),
        ] {
            good();
            assert!(select(&program, &bad_sources, &compilations, &env, &tree, &target).is_err());
        }
        for bad in [
            vec![],
            vec![Compilation {
                package: "foreign".into(),
                arguments: arguments.clone(),
            }],
            vec![
                Compilation {
                    package: "p".into(),
                    arguments: arguments.clone(),
                },
                Compilation {
                    package: "p".into(),
                    arguments: arguments.clone(),
                },
            ],
        ] {
            good();
            assert!(select(&program, &sources, &bad, &env, &tree, &target).is_err());
        }
        let mut bad = arguments;
        bad.extend(["--target".to_owned(), "archogen-not-a-target".to_owned()]);
        assert!(select(
            &program,
            &sources,
            &[Compilation {
                package: "p".into(),
                arguments: bad
            }],
            &env,
            &tree,
            &target
        )
        .unwrap_err()
        .contains("naming query failed"));
        good();
    }
}
