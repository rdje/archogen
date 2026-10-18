//! Private compiler provenance, independent of Cargo's human diagnostic streams.

use crate::{
    catalog_build::environment,
    json::{self, Json},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const SOURCE: &str = include_str!("trust_compiler_observer.rs");

/// One embedded observer build, shared by the role sessions of an inventory operation.
pub(super) struct Compiler {
    executable: PathBuf,
    bytes: Vec<u8>,
}

/// One fresh role build, whose records cannot be confused with another build's.
pub(super) struct Session {
    executable: PathBuf,
    records: PathBuf,
    identity: String,
    role: String,
    cwd: PathBuf,
}

fn fresh(parent: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    for counter in 0..u64::MAX {
        let path = parent.join(format!("{}-{counter}", std::process::id()));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(format!("{}: {error}", path.display())),
        }
    }
    Err("compiler provenance session identities exhausted".to_owned())
}

fn text_path(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("non-UTF8 compiler provenance path: {}", path.display()))
}

impl Compiler {
    /// Compile trusted embedded source once, under the same pinned and cleared environment as inventory builds.
    pub(super) fn prepare(out: &Path, pin: &str, cargo_home: &Path) -> Result<Self, String> {
        let directory = fresh(&out.join("compiler-provenance"))?;
        let source = directory.join("observer.rs");
        fs::write(&source, SOURCE).map_err(|error| format!("{}: {error}", source.display()))?;
        let executable = directory.join("observer");
        let env = environment(pin, cargo_home, &directory)?;
        let result = Command::new("rustc")
            .args([
                "--edition=2021",
                "--crate-name",
                "archogen_compiler_observer",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&executable)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .stdin(Stdio::null())
            .output()
            .map_err(|error| format!("compiler provenance observer did not build: {error}"))?;
        if !result.status.success() {
            return Err(format!(
                "compiler provenance observer build failed: {}",
                String::from_utf8_lossy(&result.stderr).trim()
            ));
        }
        let bytes =
            fs::read(&executable).map_err(|error| format!("{}: {error}", executable.display()))?;
        if bytes.is_empty() {
            return Err("compiler provenance observer is empty".to_owned());
        }
        Ok(Self { executable, bytes })
    }

    /// Install verified observer bytes in a fresh session under this role's Cargo target directory.
    pub(super) fn session(
        &self,
        role: &str,
        env: &[(String, String)],
        cwd: &Path,
    ) -> Result<Session, String> {
        let target = env
            .iter()
            .rfind(|(key, _)| key == "CARGO_TARGET_DIR")
            .map(|(_, value)| Path::new(value))
            .ok_or("compiler provenance requires CARGO_TARGET_DIR")?;
        let target = if target.is_absolute() {
            target.to_path_buf()
        } else {
            cwd.join(target)
        };
        let directory = fresh(&target.join(".archogen-provenance"))?;
        let executable = directory.join("observer");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&executable)
            .map_err(|error| format!("{}: {error}", executable.display()))?;
        file.write_all(&self.bytes)
            .map_err(|error| format!("{}: {error}", executable.display()))?;
        fs::set_permissions(
            &executable,
            fs::metadata(&self.executable)
                .map_err(|error| error.to_string())?
                .permissions(),
        )
        .map_err(|error| format!("{}: {error}", executable.display()))?;
        drop(file);
        if fs::read(&executable).map_err(|error| error.to_string())? != self.bytes {
            return Err("installed compiler provenance observer differs".to_owned());
        }
        let records = directory.join("records");
        fs::create_dir(&records).map_err(|error| format!("{}: {error}", records.display()))?;
        let identity =
            text_path(&fs::canonicalize(&directory).map_err(|error| error.to_string())?)?;
        let cwd = fs::canonicalize(cwd).map_err(|error| error.to_string())?;
        Ok(Session {
            executable,
            records,
            identity,
            role: role.to_owned(),
            cwd,
        })
    }
}

fn decode(value: &Json, field: &str) -> Result<String, String> {
    let encoded = value
        .get(field)
        .and_then(Json::as_str)
        .ok_or_else(|| format!("missing {field}"))?;
    decode_hex(encoded, field)
}

fn decode_hex(encoded: &str, field: &str) -> Result<String, String> {
    if !encoded.len().is_multiple_of(2)
        || !encoded
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!("invalid hexadecimal {field}"));
    }
    let bytes = encoded
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| {
                if byte <= b'9' {
                    byte - b'0'
                } else {
                    byte - b'a' + 10
                }
            };
            digit(pair[0]) * 16 + digit(pair[1])
        })
        .collect::<Vec<_>>();
    if bytes.contains(&0) {
        return Err(format!("NUL in {field}"));
    }
    String::from_utf8(bytes).map_err(|_| format!("non-UTF8 {field}"))
}

impl Session {
    /// Only these explicit settings extend the existing base environment, and the observer removes them before rustc.
    pub(super) fn environment(
        &self,
        base: &[(String, String)],
    ) -> Result<Vec<(String, String)>, String> {
        let mut env = base.to_vec();
        env.extend([
            ("RUSTC_WRAPPER".to_owned(), text_path(&self.executable)?),
            (
                "ARCHOGEN_COMPILER_RECORDS".to_owned(),
                text_path(&self.records)?,
            ),
            (
                "ARCHOGEN_COMPILER_SESSION".to_owned(),
                self.identity.clone(),
            ),
            ("ARCHOGEN_COMPILER_ROLE".to_owned(), self.role.clone()),
        ]);
        Ok(env)
    }

    /// Read completed successful invocations; query-only records never become units.
    pub(super) fn invocations(&self) -> Result<Vec<Vec<String>>, String> {
        let entries = fs::read_dir(&self.records).map_err(|error| error.to_string())?;
        let mut count = 0;
        let mut invocations = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|error| error.to_string())?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).map_err(|error| error.to_string())?;
            if !metadata.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                return Err(format!(
                    "incomplete or unexpected compiler provenance record: {}",
                    path.display()
                ));
            }
            let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            let value = json::parse(&text)?;
            let Json::Object(fields) = &value else {
                return Err("compiler provenance record is not an object".to_owned());
            };
            if fields.len() != 8 || json::write(&value) + "\n" != text {
                return Err(
                    "compiler provenance record is not canonical or has unexpected fields"
                        .to_owned(),
                );
            }
            if value.get("schema").and_then(Json::as_str) != Some("archogen-compiler-provenance/1")
                || decode(&value, "session_hex")? != self.identity
                || decode(&value, "role_hex")? != self.role
                || decode(&value, "cwd_hex")? != text_path(&self.cwd)?
            {
                return Err(
                    "compiler provenance schema, session, role or working directory differs"
                        .to_owned(),
                );
            }
            let pid = match value.get("pid") {
                Some(Json::Number(number)) => number.parse::<u32>().ok().filter(|pid| *pid > 0),
                _ => None,
            }
            .ok_or("invalid compiler provenance invocation identity")?;
            if path.file_name().and_then(|name| name.to_str()) != Some(&format!("{pid}.json")) {
                return Err(
                    "compiler provenance filename differs from invocation identity".to_owned(),
                );
            }
            if value.get("exit") != Some(&Json::Number("0".to_owned()))
                || value.get("signal") != Some(&Json::Null)
            {
                return Err("compiler provenance records an unsuccessful compiler".to_owned());
            }
            let Some(Json::Array(arguments)) = value.get("argv_hex") else {
                return Err("compiler provenance has no argument vector".to_owned());
            };
            let argv = arguments
                .iter()
                .map(|argument| {
                    argument
                        .as_str()
                        .ok_or_else(|| "non-string compiler argument".to_owned())
                        .and_then(|encoded| decode_hex(encoded, "argument"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            if argv.first().map(String::as_str) != Some("rustc") {
                return Err("compiler provenance names another compiler".to_owned());
            }
            let has = |flag| argv.iter().any(|arg| arg == flag);
            if has("--out-dir")
                && has("--crate-name")
                && argv.iter().any(|arg| arg.starts_with("--emit="))
            {
                invocations.push(argv);
            } else if !query(&argv[1..]) {
                return Err("unknown compiler provenance invocation shape".to_owned());
            }
            count += 1;
        }
        if count == 0 {
            return Err("missing compiler provenance records".to_owned());
        }
        Ok(invocations)
    }
}

fn query(argv: &[String]) -> bool {
    argv == ["-vV"]
        || argv
            == [
                "-",
                "--crate-name",
                "___",
                "--print=file-names",
                "--crate-type",
                "bin",
                "--crate-type",
                "rlib",
                "--crate-type",
                "dylib",
                "--crate-type",
                "cdylib",
                "--crate-type",
                "staticlib",
                "--crate-type",
                "proc-macro",
                "--print=sysroot",
                "--print=split-debuginfo",
                "--print=crate-name",
                "--print=cfg",
                "-Wwarnings",
            ]
}

#[cfg(test)]
mod tests {
    use super::{Compiler, Session};
    use crate::json::{self, Json};
    use std::{fs, path::PathBuf, process::Command};

    fn setup(name: &str) -> (PathBuf, Compiler, Session, Vec<(String, String)>) {
        let root = crate::trust::tests::real_root();
        let out = root.join("target/trust-compiler-tests").join(name);
        match fs::remove_dir_all(&out) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("cannot reset {}: {error}", out.display()),
        }
        let home = out.join("cargo-home");
        let compiler = Compiler::prepare(&out, "1.95.0", &home).unwrap();
        let base = crate::catalog_build::environment("1.95.0", &home, &out.join("target")).unwrap();
        let session = compiler.session("fixture", &base, &root).unwrap();
        (out, compiler, session, base)
    }

    fn run(session: &Session, base: &[(String, String)], args: &[&str]) -> std::process::Output {
        Command::new(&session.executable)
            .args(args)
            .current_dir(&session.cwd)
            .env_clear()
            .envs(session.environment(base).unwrap())
            .output()
            .unwrap()
    }

    fn hex(value: &str) -> Json {
        Json::Str(
            value
                .as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
        )
    }

    #[test]
    fn actual_query_records_reject_missing_stale_foreign_and_malformed_inputs() {
        crate::trust_test_process::bounded!(
            actual_query_records_reject_missing_stale_foreign_and_malformed_inputs
        );
        let (out, _, session, base) = setup("record-refusals");
        let result = run(&session, &base, &["rustc", "-vV"]);
        assert!(result.status.success(), "{result:?}");
        assert!(String::from_utf8(result.stdout)
            .unwrap()
            .starts_with("rustc 1.95.0"));
        assert!(session.invocations().unwrap().is_empty());
        let original = fs::read_dir(&session.records)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let bytes = fs::read(&original).unwrap();
        for (fault, reason) in [
            ("missing", "missing"),
            ("partial", "incomplete"),
            ("session", "session"),
            ("role", "role"),
            ("cwd", "working directory"),
            ("extra", "unexpected fields"),
            ("duplicate", "canonical"),
            ("status", "unsuccessful"),
            ("pid", "filename"),
            ("hex", "hexadecimal"),
            ("compiler", "another compiler"),
            ("query", "unknown"),
            ("symlink", "incomplete"),
            ("argument-type", "non-string"),
            ("object", "not an object"),
        ] {
            let records = out.join(fault);
            fs::create_dir(&records).unwrap();
            let file = records.join(original.file_name().unwrap());
            fs::write(&file, &bytes).unwrap();
            let copy = Session {
                records,
                executable: session.executable.clone(),
                identity: session.identity.clone(),
                role: session.role.clone(),
                cwd: session.cwd.clone(),
            };
            assert!(
                copy.invocations().unwrap().is_empty(),
                "valid copy before {fault}"
            );
            let mut value = json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
            let Json::Object(fields) = &mut value else {
                panic!("producer object")
            };
            match fault {
                "missing" => fs::remove_file(&file).unwrap(),
                "partial" => fs::write(copy.records.join(".unfinished.part"), b"partial").unwrap(),
                "session" => {
                    fields.insert("session_hex".to_owned(), hex("old-session"));
                }
                "role" => {
                    fields.insert("role_hex".to_owned(), hex("other-role"));
                }
                "cwd" => {
                    fields.insert("cwd_hex".to_owned(), hex("other-directory"));
                }
                "extra" => {
                    fields.insert("extra".to_owned(), Json::Null);
                }
                "duplicate" => fs::write(
                    &file,
                    std::str::from_utf8(&bytes).unwrap().replacen(
                        '{',
                        "{\"schema\":\"duplicate\",",
                        1,
                    ),
                )
                .unwrap(),
                "status" => {
                    fields.insert("exit".to_owned(), Json::Number("1".to_owned()));
                }
                "pid" => {
                    fields.insert("pid".to_owned(), Json::Number("1".to_owned()));
                }
                "hex" => {
                    fields.insert(
                        "argv_hex".to_owned(),
                        Json::Array(vec![Json::Str("xz".to_owned())]),
                    );
                }
                "compiler" => {
                    fields.insert(
                        "argv_hex".to_owned(),
                        Json::Array(vec![hex("another"), hex("-vV")]),
                    );
                }
                "query" => {
                    fields.insert(
                        "argv_hex".to_owned(),
                        Json::Array(vec![hex("rustc"), hex("--unknown-query")]),
                    );
                }
                "symlink" => {
                    fs::remove_file(&file).unwrap();
                    std::os::unix::fs::symlink(&original, &file).unwrap();
                }
                "argument-type" => {
                    fields.insert("argv_hex".to_owned(), Json::Array(vec![Json::Bool(false)]));
                }
                "object" => fs::write(&file, "[]\n").unwrap(),
                _ => unreachable!(),
            }
            if !["missing", "partial", "duplicate", "symlink", "object"].contains(&fault) {
                fs::write(&file, json::write(&value) + "\n").unwrap();
            }
            let error = copy.invocations().expect_err(fault);
            assert!(
                error.contains(reason),
                "{fault}: expected {reason}, got {error}"
            );
        }
    }

    #[test]
    fn actual_records_refuse_remaining_field_range_byte_and_shape_boundaries() {
        crate::trust_test_process::bounded!(
            actual_records_refuse_remaining_field_range_byte_and_shape_boundaries
        );
        let (out, compiler, session, base) = setup("record-boundaries");
        let source = out.join("probe.rs");
        let compiled = out.join("compiled");
        fs::create_dir(&compiled).unwrap();
        fs::write(&source, "fn main() {}\n").unwrap();
        let arguments = [
            "rustc",
            "--crate-name",
            "boundary_probe",
            "--emit=dep-info,link",
            "--out-dir",
            compiled.to_str().unwrap(),
            source.to_str().unwrap(),
        ];
        assert!(run(&session, &base, &arguments).status.success());
        let expected = vec![arguments.map(str::to_owned).to_vec()];
        assert_eq!(session.invocations().unwrap(), expected);
        let original = fs::read_dir(&session.records)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let bytes = fs::read(&original).unwrap();
        for (fault, reason) in [
            ("schema-version", "schema"),
            ("schema-type", "schema"),
            ("session-type", "missing session_hex"),
            ("role-type", "missing role_hex"),
            ("cwd-type", "missing cwd_hex"),
            ("missing-key", "unexpected fields"),
            (
                "pid-zero",
                "invalid compiler provenance invocation identity",
            ),
            (
                "pid-negative",
                "invalid compiler provenance invocation identity",
            ),
            (
                "pid-overflow",
                "invalid compiler provenance invocation identity",
            ),
            (
                "pid-fraction",
                "invalid compiler provenance invocation identity",
            ),
            (
                "pid-type",
                "invalid compiler provenance invocation identity",
            ),
            ("exit-type", "unsuccessful"),
            ("signal-number", "unsuccessful"),
            ("argv-type", "no argument vector"),
            ("argv-empty", "another compiler"),
            ("hex-odd", "hexadecimal"),
            ("hex-uppercase", "hexadecimal"),
            ("hex-nul", "NUL"),
            ("hex-non-utf8", "non-UTF8"),
            ("query-tail", "unknown"),
            ("compilation-no-output", "unknown"),
            ("compilation-no-name", "unknown"),
            ("compilation-no-emit", "unknown"),
            ("no-newline", "canonical"),
            ("extra-newline", "canonical"),
            ("invalid-json", ""),
            ("invalid-text", ""),
            ("record-directory", "incomplete"),
            ("record-extension", "incomplete"),
        ] {
            let records = out.join(fault);
            fs::create_dir(&records).unwrap();
            let file = records.join(original.file_name().unwrap());
            fs::write(&file, &bytes).unwrap();
            let copy = Session {
                records,
                executable: session.executable.clone(),
                identity: session.identity.clone(),
                role: session.role.clone(),
                cwd: session.cwd.clone(),
            };
            assert_eq!(
                copy.invocations().unwrap(),
                expected,
                "valid before {fault}"
            );
            let mut value = json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
            let Json::Object(fields) = &mut value else {
                panic!("producer object")
            };
            let raw = match fault {
                "schema-version" => {
                    fields.insert(
                        "schema".into(),
                        Json::Str("archogen-compiler-provenance/2".into()),
                    );
                    None
                }
                "schema-type" => {
                    fields.insert("schema".into(), Json::Bool(false));
                    None
                }
                "session-type" | "role-type" | "cwd-type" => {
                    fields.insert(
                        format!("{}_hex", fault.strip_suffix("-type").unwrap()),
                        Json::Null,
                    );
                    None
                }
                "missing-key" => {
                    fields.remove("schema");
                    None
                }
                "pid-zero" | "pid-negative" | "pid-overflow" | "pid-fraction" => {
                    let number = match fault {
                        "pid-zero" => "0",
                        "pid-negative" => "-1",
                        "pid-overflow" => "4294967296",
                        _ => "1.5",
                    };
                    fields.insert("pid".into(), Json::Number(number.into()));
                    None
                }
                "pid-type" => {
                    fields.insert("pid".into(), Json::Str("1".into()));
                    None
                }
                "exit-type" => {
                    fields.insert("exit".into(), Json::Str("0".into()));
                    None
                }
                "signal-number" => {
                    fields.insert("signal".into(), Json::Number("0".into()));
                    None
                }
                "argv-type" => {
                    fields.insert("argv_hex".into(), Json::Null);
                    None
                }
                "argv-empty" => {
                    fields.insert("argv_hex".into(), Json::Array(Vec::new()));
                    None
                }
                "hex-odd" | "hex-uppercase" | "hex-nul" | "hex-non-utf8" => {
                    let encoded = match fault {
                        "hex-odd" => "0",
                        "hex-uppercase" => "FF",
                        "hex-nul" => "00",
                        _ => "ff",
                    };
                    let Some(Json::Array(arguments)) = fields.get_mut("argv_hex") else {
                        panic!("producer argument vector")
                    };
                    arguments.push(Json::Str(encoded.into()));
                    None
                }
                "query-tail"
                | "compilation-no-output"
                | "compilation-no-name"
                | "compilation-no-emit" => {
                    let arguments: &[&str] = match fault {
                        "query-tail" => &["rustc", "-vV", "extra"],
                        "compilation-no-output" => {
                            &["rustc", "--crate-name", "probe", "--emit=dep-info,link"]
                        }
                        "compilation-no-name" => {
                            &["rustc", "--out-dir", "out", "--emit=dep-info,link"]
                        }
                        _ => &["rustc", "--out-dir", "out", "--crate-name", "probe"],
                    };
                    fields.insert(
                        "argv_hex".into(),
                        Json::Array(arguments.iter().map(|argument| hex(argument)).collect()),
                    );
                    None
                }
                "no-newline" => Some(bytes[..bytes.len() - 1].to_vec()),
                "extra-newline" => Some([bytes.as_slice(), b"\n"].concat()),
                "invalid-json" => Some(b"{\n".to_vec()),
                "invalid-text" => Some(vec![0xff]),
                "record-directory" => {
                    fs::remove_file(&file).unwrap();
                    fs::create_dir(&file).unwrap();
                    None
                }
                "record-extension" => {
                    fs::rename(&file, file.with_extension("JSON")).unwrap();
                    None
                }
                _ => unreachable!(),
            };
            if let Some(raw) = raw {
                fs::write(&file, raw).unwrap();
            } else if !["record-directory", "record-extension"].contains(&fault) {
                fs::write(&file, json::write(&value) + "\n").unwrap();
            }
            if fault.starts_with("pid-") && fault != "pid-type" {
                let Some(Json::Number(number)) = value.get("pid") else {
                    panic!("numeric PID fault")
                };
                fs::rename(&file, copy.records.join(format!("{number}.json"))).unwrap();
            }
            let error = copy.invocations().expect_err(fault);
            assert!(
                !error.is_empty() && error.contains(reason),
                "{fault}: {error}"
            );
        }
        let fresh = compiler
            .session("boundary-recovery", &base, &session.cwd)
            .unwrap();
        assert_ne!(fresh.records, session.records);
        assert!(run(&fresh, &base, &["rustc", "-vV"]).status.success());
        assert!(fresh.invocations().unwrap().is_empty());
    }

    #[test]
    fn actual_compiler_removes_private_environment_and_records_failures() {
        crate::trust_test_process::bounded!(
            actual_compiler_removes_private_environment_and_records_failures
        );
        let (out, compiler, session, base) = setup("compiler-forwarding");
        let observer_tests = out.join("observer-tests");
        let built = Command::new("rustc")
            .args(["--edition=2021", "--test"])
            .arg(compiler.executable.parent().unwrap().join("observer.rs"))
            .arg("-o")
            .arg(&observer_tests)
            .env_clear()
            .envs(base.clone())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        let checked = Command::new(&observer_tests).output().unwrap();
        assert!(checked.status.success(), "{checked:?}");
        assert!(String::from_utf8(checked.stdout)
            .unwrap()
            .contains("1 passed; 0 failed"));
        let source = out.join("probe.rs");
        fs::write(
            &source,
            r#"
const _: () = assert!(option_env!("RUSTC_WRAPPER").is_none());
const _: () = assert!(option_env!("ARCHOGEN_COMPILER_RECORDS").is_none());
const _: () = assert!(option_env!("ARCHOGEN_COMPILER_SESSION").is_none());
const _: () = assert!(option_env!("ARCHOGEN_COMPILER_ROLE").is_none());
fn main() {}
"#,
        )
        .unwrap();
        let target = out.join("compiled");
        fs::create_dir(&target).unwrap();
        let arguments = [
            "rustc",
            "--crate-name",
            "probe",
            "--emit=dep-info,link",
            "--out-dir",
            target.to_str().unwrap(),
            source.to_str().unwrap(),
        ];
        let result = run(&session, &base, &arguments);
        assert!(result.status.success(), "{result:?}");
        assert!(target.join("probe").is_file());
        assert_eq!(
            session.invocations().unwrap(),
            [arguments.map(str::to_owned).to_vec()]
        );
        let failed = compiler
            .session("failed-compiler", &base, &session.cwd)
            .unwrap();
        fs::write(&source, "compile_error!(\"actual compiler failure\");").unwrap();
        let result = run(&failed, &base, &arguments);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("actual compiler failure"));
        assert!(failed.invocations().unwrap_err().contains("unsuccessful"));
        let missing = compiler
            .session("missing-compiler", &base, &session.cwd)
            .unwrap();
        let result = run(
            &missing,
            &base,
            &[out.join("no-compiler").to_str().unwrap()],
        );
        assert_eq!(result.status.code(), Some(2));
        assert!(missing.invocations().unwrap_err().contains("incomplete"));
        let fresh = compiler
            .session("fresh-recovery", &base, &session.cwd)
            .unwrap();
        assert!(run(&fresh, &base, &["rustc", "-vV"]).status.success());
        assert!(fresh.invocations().unwrap().is_empty());
    }

    #[test]
    fn actual_native_receiver_observes_every_forwarded_argument_byte() {
        crate::trust_test_process::bounded!(
            actual_native_receiver_observes_every_forwarded_argument_byte
        );
        use std::os::unix::ffi::{OsStrExt, OsStringExt};
        let (out, _, session, base) = setup("native-argv-forwarding");
        let source = include_str!("../../scripts/fixtures/compiler_provenance_receiver.c")
            .replace("ARCHOGEN_PROTO_TRACE_DIR", "ARCHOGEN_COMPILER_RECORDS")
            .replace("ARCHOGEN_PROTO_ROLE", "ARCHOGEN_COMPILER_ROLE")
            .replace("ARCHOGEN_PROTO_SESSION", "ARCHOGEN_COMPILER_SESSION");
        let file = out.join("receiver.c");
        fs::write(&file, source).unwrap();
        let receiver = out.join("receiver");
        let compiled = Command::new("cc")
            .args(["-Wall", "-Wextra", "-Werror"])
            .arg(&file)
            .arg("-o")
            .arg(&receiver)
            .env_clear()
            .envs(base.clone())
            .output()
            .unwrap();
        assert!(compiled.status.success(), "{compiled:?}");
        let mut arguments = vec![receiver.into_os_string()];
        arguments.extend(
            [
                "",
                "a b",
                "a'b",
                "\"quoted\"",
                "line\nreturn\r",
                "back\\slash",
                "é",
                "--cfg",
                "feature=\"held\"",
                "--extern",
                "common=held.rlib",
            ]
            .into_iter()
            .map(std::ffi::OsString::from),
        );
        arguments.push(std::ffi::OsString::from_vec(vec![b'x', 0xff]));
        let sink = out.join("received.argv");
        let result = Command::new(&session.executable)
            .args(&arguments)
            .current_dir(&session.cwd)
            .env_clear()
            .envs(session.environment(&base).unwrap())
            .env("ARGV_SINK", &sink)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(7));
        assert_eq!(result.stdout, b"native receiver stdout\n");
        assert_eq!(result.stderr, b"native receiver stderr\n");
        let received = fs::read(&sink).unwrap();
        let expected = arguments
            .iter()
            .flat_map(|arg| arg.as_bytes().iter().copied().chain([0]))
            .collect::<Vec<_>>();
        assert_eq!(received, expected, "independent native argv");
        let file = fs::read_dir(&session.records)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let record = json::parse(&fs::read_to_string(file).unwrap()).unwrap();
        let expected_hex = arguments
            .iter()
            .map(|arg| {
                Json::Str(
                    arg.as_bytes()
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(record.get("argv_hex"), Some(&Json::Array(expected_hex)));
        assert_eq!(record.get("exit"), Some(&Json::Number("7".to_owned())));
        assert_eq!(record.get("signal"), Some(&Json::Null));
    }
}
