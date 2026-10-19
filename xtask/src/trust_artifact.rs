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
    fn naming_query_preserves_test_and_refuses_duplicate_test_options() {
        let query = |observed: &[&str]| {
            naming_arguments(
                &observed
                    .iter()
                    .map(|value| (*value).to_owned())
                    .collect::<Vec<_>>(),
            )
        };
        for (observed, expected) in [
            (
                &[
                    "rustc",
                    "--test",
                    "--crate-name",
                    "probe",
                    "--out-dir",
                    "outputs",
                ][..],
                &["--print=file-names", "-", "--test", "--crate-name", "probe"][..],
            ),
            (
                &[
                    "rustc",
                    "--crate-name",
                    "probe",
                    "--test",
                    "--out-dir",
                    "outputs",
                ][..],
                &["--print=file-names", "-", "--crate-name", "probe", "--test"][..],
            ),
            (
                &[
                    "rustc",
                    "--crate-name",
                    "probe",
                    "--out-dir",
                    "outputs",
                    "--test",
                ][..],
                &["--print=file-names", "-", "--crate-name", "probe", "--test"][..],
            ),
        ] {
            let (actual, directory) = query(observed).unwrap();
            assert_eq!(actual, expected, "{observed:?}");
            assert_eq!(directory, "outputs");
        }
        for observed in [
            &[
                "rustc",
                "--test",
                "--test",
                "--crate-name",
                "probe",
                "--out-dir",
                "outputs",
            ][..],
            &[
                "rustc",
                "--crate-name",
                "probe",
                "--test",
                "--out-dir",
                "outputs",
                "--test",
                "-C",
                "opt-level=3",
            ][..],
            &[
                "rustc",
                "--crate-name",
                "probe",
                "--out-dir",
                "outputs",
                "--test",
                "--test",
            ][..],
        ] {
            assert_eq!(
                query(observed).unwrap_err(),
                "duplicate observed --test",
                "{observed:?}"
            );
        }
    }

    #[test]
    fn naming_query_requires_nonempty_nonflaglike_option_values() {
        let base = &["rustc", "--crate-name", "probe", "--out-dir", "outputs"][..];
        let basic_query = &["--print=file-names", "-", "--crate-name", "probe"][..];
        for (option, valid, prefix, expected) in [
            (
                "--crate-name",
                "probe",
                &["rustc", "--out-dir", "outputs"][..],
                basic_query,
            ),
            (
                "--crate-type",
                "rlib",
                base,
                &[
                    "--print=file-names",
                    "-",
                    "--crate-name",
                    "probe",
                    "--crate-type",
                    "rlib",
                ][..],
            ),
            (
                "--target",
                "host",
                base,
                &[
                    "--print=file-names",
                    "-",
                    "--crate-name",
                    "probe",
                    "--target",
                    "host",
                ][..],
            ),
            (
                "--out-dir",
                "outputs",
                &["rustc", "--crate-name", "probe"][..],
                basic_query,
            ),
        ] {
            for value in [valid, "", "-flaglike"] {
                let mut observed: Vec<String> =
                    prefix.iter().map(|value| (*value).to_owned()).collect();
                observed.extend([option.to_owned(), value.to_owned()]);
                let result = naming_arguments(&observed);
                if value == valid {
                    let (query, directory) = result.unwrap();
                    assert_eq!(query, expected, "{observed:?}");
                    assert_eq!(directory, "outputs");
                } else {
                    assert_eq!(
                        result.unwrap_err(),
                        format!("missing observed {option} value"),
                        "{observed:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn naming_query_forwards_only_the_exact_extra_filename_option() {
        for (value, suffix) in [
            ("extra-filename=-hash", &["-C", "extra-filename=-hash"][..]),
            ("extra-filename=", &["-C", "extra-filename="][..]),
            ("metadata=extra-filename=-bait", &[][..]),
            ("extra-filename", &[][..]),
            ("extra-filename-other=-hash", &[][..]),
        ] {
            let observed: Vec<String> = [
                "rustc",
                "--crate-name",
                "probe",
                "--out-dir",
                "outputs",
                "-C",
                value,
            ]
            .map(str::to_owned)
            .to_vec();
            let (query, directory) = naming_arguments(&observed).unwrap();
            let mut expected = ["--print=file-names", "-", "--crate-name", "probe"]
                .map(str::to_owned)
                .to_vec();
            expected.extend(suffix.iter().map(|value| (*value).to_owned()));
            assert_eq!(query, expected, "{value:?}");
            assert_eq!(directory, "outputs");
        }
    }

    #[test]
    fn naming_query_refuses_each_joined_option_but_allows_source_lookalikes() {
        let query = |extra| {
            let observed = [
                "rustc",
                "--crate-name",
                "probe",
                "--out-dir",
                "outputs",
                extra,
            ]
            .map(str::to_owned)
            .to_vec();
            naming_arguments(&observed)
        };
        for joined in [
            "--crate-name=joined",
            "--crate-type=rlib",
            "--target=host",
            "--out-dir=outputs",
            "-Cextra-filename=-hash",
        ] {
            assert_eq!(
                query(joined).unwrap_err(),
                format!("unsupported observed naming option {joined}")
            );
        }
        for source in [
            "source.rs",
            "src/look--crate-name=joined.rs",
            "src/look--crate-type=rlib.rs",
            "src/look--target=host.rs",
            "src/look--out-dir=outputs.rs",
            "src/look-Cextra-filename=-hash.rs",
        ] {
            let (actual, directory) = query(source).unwrap();
            assert_eq!(
                actual,
                ["--print=file-names", "-", "--crate-name", "probe"],
                "{source:?}"
            );
            assert_eq!(directory, "outputs");
        }
    }

    #[test]
    fn every_reported_output_is_read_and_the_rlib_can_be_in_each_position() {
        crate::trust_test_process::bounded!(
            every_reported_output_is_read_and_the_rlib_can_be_in_each_position
        );
        use crate::catalog_build::environment;
        use std::{
            collections::BTreeSet,
            process::{Command, Stdio},
        };

        let base = real_root().join("target/trust-tests/artifact-output-positions");
        let _ = fs::remove_dir_all(&base);
        let target = base.join("target");
        let out = target.join("release/deps");
        fs::create_dir_all(&out).unwrap();
        let env = environment("1.95.0", &base.join("cargo-home"), &target).unwrap();
        let printed = Command::new("rustc")
            .args([
                "--print=file-names",
                "-",
                "--crate-name",
                "probe",
                "--crate-type",
                "dylib",
                "--crate-type",
                "rlib",
                "--crate-type",
                "staticlib",
            ])
            .current_dir(&base)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(printed.status.success(), "{:?}", printed.stderr);
        let shared = if cfg!(target_os = "macos") {
            "libprobe.dylib"
        } else {
            "libprobe.so"
        };
        let rlib = "libprobe.rlib";
        let archive = "libprobe.a";
        let names: Vec<_> = std::str::from_utf8(&printed.stdout)
            .unwrap()
            .lines()
            .collect();
        assert_eq!(names.len(), 3);
        assert_eq!(
            names.into_iter().collect::<BTreeSet<_>>(),
            BTreeSet::from([shared, rlib, archive])
        );
        fs::write(base.join("reported-names.txt"), &printed.stdout).unwrap();
        fs::write(out.join(rlib), b"selected library bytes\n").unwrap();
        for name in [shared, archive] {
            fs::write(out.join(name), b"nonselected output bytes\n").unwrap();
        }
        let good = |stdout: &[u8]| {
            let actual = outputs(stdout, &out, &target, true).unwrap();
            assert_eq!(actual.0, "release/deps/libprobe.rlib");
            assert_eq!(
                actual.1,
                "26fff0e84dc91af757184906354cddbb54702d2078c781ce0ad5eb1ff693de8d"
            );
        };
        good(&printed.stdout);
        for position in 0..3 {
            let mut names = vec![shared, archive];
            names.insert(position, rlib);
            let stdout = format!("{}\n", names.join("\n"));
            good(stdout.as_bytes());
            for missing in [shared, archive] {
                fs::remove_file(out.join(missing)).unwrap();
                let result = outputs(stdout.as_bytes(), &out, &target, true);
                fs::write(out.join(missing), b"nonselected output bytes\n").unwrap();
                let error = result.unwrap_err();
                assert!(error.starts_with(&format!("{missing}:")), "{error}");
                good(stdout.as_bytes());
            }
        }
    }

    use std::path::{Path, PathBuf};

    struct OutputFiles {
        base: PathBuf,
        target: PathBuf,
        out: PathBuf,
        names: [&'static str; 3],
    }

    impl OutputFiles {
        fn new(label: &str) -> Self {
            let base = real_root().join("target/trust-tests").join(label);
            let _ = fs::remove_dir_all(&base);
            let target = base.join("target");
            let out = target.join("release/deps");
            fs::create_dir_all(&out).unwrap();
            let shared = if cfg!(target_os = "macos") {
                "libprobe.dylib"
            } else {
                "libprobe.so"
            };
            let fixture = Self {
                base,
                target,
                out,
                names: [shared, "libprobe.rlib", "libprobe.a"],
            };
            for name in fixture.names {
                fixture.restore(name);
            }
            fixture
        }

        fn restore(&self, name: &str) {
            let bytes: &[u8] = if name == "libprobe.rlib" {
                b"selected library bytes\n"
            } else {
                b"nonselected output bytes\n"
            };
            fs::write(self.out.join(name), bytes).unwrap();
        }

        fn response(&self) -> String {
            format!("{}\n", self.names.join("\n"))
        }

        fn good(&self) {
            let actual =
                outputs(self.response().as_bytes(), &self.out, &self.target, true).unwrap();
            assert_eq!(actual.0, "release/deps/libprobe.rlib");
            assert_eq!(
                actual.1,
                "26fff0e84dc91af757184906354cddbb54702d2078c781ce0ad5eb1ff693de8d"
            );
        }
    }

    fn pipe_command(
        fixture: &OutputFiles,
        action: &str,
        pipe: &Path,
        stop: Option<&Path>,
    ) -> std::process::Command {
        use crate::catalog_build::environment;
        use std::process::{Command, Stdio};
        let env = environment("1.95.0", &fixture.base.join("cargo-home"), &fixture.target).unwrap();
        let mut command = Command::new("python3");
        command
            .args(["-I", "-B", "-X", "utf8"])
            .arg(real_root().join("scripts/fixtures/artifact_output_pipe.py"))
            .arg(action)
            .arg("--pipe")
            .arg(pipe);
        if let Some(stop) = stop {
            command.arg("--stop").arg(stop);
        }
        command
            .current_dir(&fixture.base)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command
    }

    fn create_pipe(fixture: &OutputFiles, pipe: &Path) {
        let output = pipe_command(fixture, "create", pipe, None)
            .output()
            .unwrap();
        assert!(output.status.success(), "{:?}", output.stderr);
        assert_eq!(output.stdout, b"created\n");
    }

    fn finish_pipe_writer(child: std::process::Child, stop: &Path) -> std::process::Output {
        let stopped = fs::write(stop, b"stop");
        let output = child.wait_with_output().unwrap();
        let removed = fs::remove_file(stop);
        stopped.unwrap();
        removed.unwrap();
        output
    }

    fn assert_pipe_writer(output: &std::process::Output, expected: &[u8]) {
        assert!(output.status.success(), "{:?}", output.stderr);
        assert_eq!(output.stdout, expected);
    }

    #[test]
    fn linked_outputs_are_refused_at_every_reported_position() {
        crate::trust_test_process::bounded!(linked_outputs_are_refused_at_every_reported_position);
        let fixture = OutputFiles::new("artifact-output-kind-links");
        let foreign = b"foreign library bytes\n";
        fs::write(fixture.base.join("foreign.rlib"), foreign).unwrap();
        fixture.good();
        for name in fixture.names {
            let file = fixture.out.join(name);
            fs::remove_file(&file).unwrap();
            symlink("../../../foreign.rlib", &file).unwrap();
            assert_eq!(
                fs::read_link(&file).unwrap(),
                Path::new("../../../foreign.rlib")
            );
            assert_eq!(fs::read(&file).unwrap(), foreign);
            let result = outputs(
                fixture.response().as_bytes(),
                &fixture.out,
                &fixture.target,
                true,
            );
            fs::remove_file(&file).unwrap();
            fixture.restore(name);
            assert_eq!(
                result.unwrap_err(),
                format!("compiler artifact {name} is not a regular non-symlink file")
            );
            fixture.good();
        }
        assert_eq!(
            fs::read(fixture.base.join("foreign.rlib")).unwrap(),
            foreign
        );
    }

    #[test]
    fn named_pipe_outputs_are_refused_without_reading_at_every_position() {
        crate::trust_test_process::bounded!(
            named_pipe_outputs_are_refused_without_reading_at_every_position
        );
        let fixture = OutputFiles::new("artifact-output-kind-pipes");
        let control = fixture.base.join("writer-control");
        let stop = fixture.base.join("writer-control.stop");
        create_pipe(&fixture, &control);
        let child = pipe_command(&fixture, "write", &control, Some(&stop))
            .spawn()
            .unwrap();
        let bytes = fs::read(&control);
        let output = finish_pipe_writer(child, &stop);
        fs::remove_file(&control).unwrap();
        assert_pipe_writer(&output, b"wrote\n");
        assert_eq!(bytes.unwrap(), b"stream artifact bytes\n");
        fixture.good();
        for (index, name) in ["libprobe.rlib", fixture.names[0], fixture.names[2]]
            .into_iter()
            .enumerate()
        {
            let file = fixture.out.join(name);
            let stop = fixture.base.join(format!("writer-{index}.stop"));
            fs::remove_file(&file).unwrap();
            create_pipe(&fixture, &file);
            let child = pipe_command(&fixture, "write", &file, Some(&stop))
                .spawn()
                .unwrap();
            let result = outputs(
                fixture.response().as_bytes(),
                &fixture.out,
                &fixture.target,
                true,
            );
            let output = finish_pipe_writer(child, &stop);
            fs::remove_file(&file).unwrap();
            fixture.restore(name);
            assert_pipe_writer(&output, b"stopped\n");
            assert_eq!(
                result.unwrap_err(),
                format!("compiler artifact {name} is not a regular non-symlink file")
            );
            fixture.good();
        }
    }

    #[test]
    fn malformed_basenames_are_refused_before_their_output_lookup() {
        crate::trust_test_process::bounded!(
            malformed_basenames_are_refused_before_their_output_lookup
        );
        let fixture = OutputFiles::new("artifact-basename-controls");
        fs::create_dir_all(fixture.out.join("nested")).unwrap();
        let witnesses = [
            fixture.out.join("nested/inside"),
            fixture.base.join("foreign.rlib"),
            fixture.out.join("back\\slash"),
            fixture.out.join("carriage\rname"),
        ];
        let bytes = b"malformed basename bytes\n";
        for file in &witnesses {
            fs::write(file, bytes).unwrap();
        }
        for name in [
            "nested/inside",
            "../../../foreign.rlib",
            "back\\slash",
            "carriage\rname",
            "nul\0name",
            "",
            ".",
            "..",
        ] {
            fixture.good();
            let stdout = format!("{name}\n");
            assert_eq!(
                outputs(stdout.as_bytes(), &fixture.out, &fixture.target, false).unwrap_err(),
                "invalid or duplicate compiler artifact basename",
                "{name:?}"
            );
            fixture.good();
        }
        for file in witnesses {
            assert_eq!(fs::read(file).unwrap(), bytes);
        }
    }

    #[test]
    fn every_output_name_is_unique_including_nonselected_outputs() {
        crate::trust_test_process::bounded!(
            every_output_name_is_unique_including_nonselected_outputs
        );
        let fixture = OutputFiles::new("artifact-output-uniqueness");
        let shared = fixture.names[0];
        let rlib = fixture.names[1];
        for names in [
            [rlib, shared, shared],
            [shared, rlib, shared],
            [shared, shared, rlib],
            [rlib, rlib, shared],
        ] {
            fixture.good();
            let stdout = format!("{}\n", names.join("\n"));
            assert_eq!(
                outputs(stdout.as_bytes(), &fixture.out, &fixture.target, true).unwrap_err(),
                "invalid or duplicate compiler artifact basename",
                "{names:?}"
            );
            fixture.good();
        }
    }

    fn independent_artifact_hash(bytes: &[u8], env: &[(String, String)]) -> String {
        use std::{
            io::Write,
            process::{Command, Stdio},
        };
        let mut child = Command::new("shasum")
            .args(["-a", "256"])
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let wrote = child.stdin.take().unwrap().write_all(bytes);
        let output = child.wait_with_output().unwrap();
        wrote.unwrap();
        assert!(output.status.success(), "{:?}", output.stderr);
        String::from_utf8(output.stdout)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_owned()
    }

    #[test]
    fn source_selection_preserves_ordinary_and_canonical_prefixes_under_a_tree_alias() {
        crate::trust_test_process::bounded!(
            source_selection_preserves_ordinary_and_canonical_prefixes_under_a_tree_alias
        );
        use super::{select, Compilation, Program, Target, TargetSources};
        use crate::catalog_build::environment;
        use std::{
            collections::BTreeSet,
            process::{Command, Stdio},
        };

        let base = real_root().join("target/trust-tests/artifact-source-aliases");
        let _ = fs::remove_dir_all(&base);
        let physical = base.join("physical-tree");
        let alias = base.join("tree-alias");
        fs::create_dir_all(&physical).unwrap();
        symlink("physical-tree", &alias).unwrap();
        assert_eq!(fs::read_link(&alias).unwrap(), Path::new("physical-tree"));
        assert_eq!(fs::canonicalize(&alias).unwrap(), physical);
        let target = base.join("target");
        let out = target.join("release/deps");
        fs::create_dir_all(&out).unwrap();
        let env = environment("1.95.0", &base.join("cargo-home"), &target).unwrap();
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
        for (label, tree, source_path) in [
            ("physical", physical.clone(), physical.join("source.rs")),
            ("ordinary", alias.clone(), alias.join("source.rs")),
            ("canonical", alias.clone(), physical.join("source.rs")),
        ] {
            fs::write(
                physical.join("source.rs"),
                "fn main() { println!(\"alias-artifact\"); }\n",
            )
            .unwrap();
            let arguments: Vec<String> = vec![
                "rustc".into(),
                "--crate-name".into(),
                "probe".into(),
                source_path.to_str().unwrap().into(),
                "--crate-type".into(),
                "bin".into(),
                "--out-dir".into(),
                out.to_str().unwrap().into(),
                "-C".into(),
                "extra-filename=-alias".into(),
            ];
            let compiled = Command::new("rustc")
                .args(&arguments[1..])
                .current_dir(&tree)
                .env_clear()
                .envs(env.iter().map(|(key, value)| (key, value)))
                .stdin(Stdio::null())
                .output()
                .unwrap();
            assert!(compiled.status.success(), "{label}: {:?}", compiled.stderr);
            fs::write(
                base.join(format!("{label}.arguments")),
                arguments.join("\0"),
            )
            .unwrap();
            let artifact = out.join("probe-alias");
            let before = fs::read(&artifact).unwrap();
            let expected = independent_artifact_hash(&before, &env);
            fs::write(base.join(format!("{label}-before.bin")), &before).unwrap();
            let before_names: BTreeSet<_> = fs::read_dir(&out)
                .unwrap()
                .map(|entry| entry.unwrap().file_name())
                .collect();
            fs::write(
                physical.join("source.rs"),
                "compile_error!(\"selection must not recompile the source\");\n",
            )
            .unwrap();
            let invalid_source = fs::read(physical.join("source.rs")).unwrap();
            let compilations = vec![Compilation {
                package: "p".into(),
                arguments,
            }];
            let actual = select(&program, &sources, &compilations, &env, &tree, &target)
                .unwrap_or_else(|error| panic!("{label}: {error}"));
            assert_eq!(actual.0, "release/deps/probe-alias", "{label}");
            assert_eq!(actual.1, expected, "{label}");
            assert_eq!(fs::read(&artifact).unwrap(), before, "{label}");
            assert_eq!(
                fs::read(physical.join("source.rs")).unwrap(),
                invalid_source,
                "{label}"
            );
            assert_eq!(
                fs::read_dir(&out)
                    .unwrap()
                    .map(|entry| entry.unwrap().file_name())
                    .collect::<BTreeSet<_>>(),
                before_names,
                "{label}"
            );
            let ran = Command::new(&artifact)
                .current_dir(&tree)
                .env_clear()
                .envs(env.iter().map(|(key, value)| (key, value)))
                .stdin(Stdio::null())
                .output()
                .unwrap();
            assert!(ran.status.success(), "{label}: {:?}", ran.stderr);
            assert_eq!(ran.stdout, b"alias-artifact\n", "{label}");
        }
    }

    #[test]
    fn naming_query_honors_the_explicit_toolchain_environment() {
        crate::trust_test_process::bounded!(naming_query_honors_the_explicit_toolchain_environment);
        use super::{select, Compilation, Program, Target, TargetSources};
        use crate::catalog_build::environment;
        use std::{
            collections::BTreeSet,
            process::{Command, Stdio},
        };

        let base = real_root().join("target/trust-tests/artifact-query-environment");
        let _ = fs::remove_dir_all(&base);
        let tree = base.join("tree");
        let target = base.join("target");
        let out = target.join("release/deps");
        fs::create_dir_all(&tree).unwrap();
        fs::create_dir_all(&out).unwrap();
        fs::write(
            tree.join("source.rs"),
            "fn main() { println!(\"pinned-artifact\"); }\n",
        )
        .unwrap();
        let env = environment("1.95.0", &base.join("cargo-home"), &target).unwrap();
        let version = Command::new("rustc")
            .arg("--version")
            .current_dir(&tree)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(version.status.success(), "{:?}", version.stderr);
        assert!(String::from_utf8_lossy(&version.stdout).starts_with("rustc 1.95.0 "));
        fs::write(base.join("pinned-version.stdout"), &version.stdout).unwrap();
        let arguments: Vec<String> = vec![
            "rustc".into(),
            "--crate-name".into(),
            "probe".into(),
            "source.rs".into(),
            "--crate-type".into(),
            "bin".into(),
            "--out-dir".into(),
            out.to_str().unwrap().into(),
            "-C".into(),
            "extra-filename=-pinned".into(),
        ];
        let compiled = Command::new("rustc")
            .args(&arguments[1..])
            .current_dir(&tree)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(compiled.status.success(), "{:?}", compiled.stderr);
        fs::write(base.join("compiled.arguments"), arguments.join("\0")).unwrap();
        let artifact = out.join("probe-pinned");
        let before = fs::read(&artifact).unwrap();
        let expected = independent_artifact_hash(&before, &env);
        fs::write(base.join("before.bin"), &before).unwrap();
        let before_names: BTreeSet<_> = fs::read_dir(&out)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        fs::write(
            tree.join("source.rs"),
            "compile_error!(\"naming must preserve the compiled artifact\");\n",
        )
        .unwrap();
        let invalid_source = fs::read(tree.join("source.rs")).unwrap();
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
            arguments,
        }];
        let good = || {
            let actual = select(&program, &sources, &compilations, &env, &tree, &target).unwrap();
            assert_eq!(actual.0, "release/deps/probe-pinned");
            assert_eq!(actual.1, expected);
            assert_eq!(fs::read(&artifact).unwrap(), before);
            assert_eq!(fs::read(tree.join("source.rs")).unwrap(), invalid_source);
            assert_eq!(
                fs::read_dir(&out)
                    .unwrap()
                    .map(|entry| entry.unwrap().file_name())
                    .collect::<BTreeSet<_>>(),
                before_names
            );
        };
        good();
        let ran = Command::new(&artifact)
            .current_dir(&tree)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(ran.status.success(), "{:?}", ran.stderr);
        assert_eq!(ran.stdout, b"pinned-artifact\n");

        let missing_pin = "archogen-artifact-query-missing-toolchain";
        let mut bad_env = env.clone();
        bad_env
            .iter_mut()
            .find(|(key, _)| key == "RUSTUP_TOOLCHAIN")
            .unwrap()
            .1 = missing_pin.into();
        for (label, values) in [("pinned", &env), ("missing-pin", &bad_env)] {
            let fields: Vec<_> = values
                .iter()
                .flat_map(|(key, value)| [key.as_str(), value.as_str()])
                .collect();
            fs::write(base.join(format!("{label}.environment")), fields.join("\0")).unwrap();
        }
        let refused = Command::new("rustc")
            .arg("--version")
            .current_dir(&tree)
            .env_clear()
            .envs(bad_env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(!refused.status.success());
        assert!(String::from_utf8_lossy(&refused.stderr).contains(missing_pin));
        fs::write(base.join("missing-pin.stderr"), &refused.stderr).unwrap();
        let observed = select(&program, &sources, &compilations, &bad_env, &tree, &target);
        fs::write(base.join("missing-pin.result"), format!("{observed:?}")).unwrap();
        good();
        let error = observed.unwrap_err();
        assert!(
            error.starts_with("compiler artifact naming query failed:"),
            "{error}"
        );
        assert!(error.contains(missing_pin), "{error}");
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
