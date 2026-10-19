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

    fn native_fixture(
        out: &std::path::Path,
        name: &str,
        source: &str,
        base: &[(String, String)],
    ) -> PathBuf {
        let file = out.join(format!("{name}.c"));
        let binary = out.join(name);
        fs::write(&file, source).unwrap();
        let built = Command::new("cc")
            .args(["-Wall", "-Wextra", "-Werror"])
            .arg(file)
            .arg("-o")
            .arg(&binary)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        binary
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn actual_installation_closes_its_writer_before_readback_allocation() {
        crate::trust_test_process::bounded!(
            actual_installation_closes_its_writer_before_readback_allocation
        );
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        fn directories(path: &std::path::Path) -> Vec<PathBuf> {
            let mut result = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            result.sort();
            result
        }
        let (out, _, session, base) = setup("install-close");
        let source = out.join("install_close.c");
        fs::write(
            &source,
            include_str!("../../scripts/fixtures/compiler_install_close.c"),
        )
        .unwrap();
        let library = out.join("install_close.dylib");
        let built = Command::new("cc")
            .args([
                "-Wall",
                "-Wextra",
                "-Werror",
                "-dynamiclib",
                "-Wl,-install_name,@rpath/install_close.dylib",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&library)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        let canonical_library = fs::canonicalize(&library).unwrap();
        let preload = canonical_library.strip_prefix(&session.cwd).unwrap();
        let sources = out.join("component");
        fs::create_dir(&sources).unwrap();
        fs::write(
            sources.join("controller.rs"),
            include_str!("trust_compiler.rs"),
        )
        .unwrap();
        fs::write(sources.join("json.rs"), include_str!("json.rs")).unwrap();
        fs::write(sources.join("trust_compiler_observer.rs"), super::SOURCE).unwrap();
        fs::write(
            sources.join("probe.rs"),
            include_str!("../../scripts/fixtures/compiler_install_close_probe.rs"),
        )
        .unwrap();
        let catalog = include_str!("catalog_build.rs");
        let start = catalog.find("pub fn environment(").unwrap();
        let tail = &catalog[start..];
        let end = tail.find("\nfn command(").unwrap();
        fs::write(sources.join("base_environment.rs"), &tail[..end]).unwrap();
        let helper = sources.join("probe");
        let built = Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "install_close_component"])
            .arg(sources.join("probe.rs"))
            .arg("-o")
            .arg(&helper)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        fn fields(line: &str) -> std::collections::BTreeMap<&str, u64> {
            let mut result = std::collections::BTreeMap::new();
            for item in line.split_ascii_whitespace().skip(1) {
                let (key, value) = item.split_once('=').unwrap();
                assert!(result.insert(key, value.parse().unwrap()).is_none());
            }
            result
        }
        for (mode, inject, scarce) in [
            ("baseline", false, false),
            ("scarce", true, true),
            ("pass", true, false),
        ] {
            let case = out.join(mode);
            let target = case.join("target");
            fs::create_dir_all(&target).unwrap();
            let trace = case.join("trace");
            let mut command = Command::new(&helper);
            command
                .arg(&case)
                .arg(out.join("cargo-home"))
                .arg(if scarce { "scarce" } else { "pass" })
                .current_dir(&session.cwd)
                .env_clear()
                .envs(base.iter().cloned())
                .env(
                    "ARCHOGEN_TEST_CLOSE_ROOT",
                    fs::canonicalize(&target).unwrap(),
                )
                .env("ARCHOGEN_TEST_CLOSE_TRACE", &trace)
                .env("ARCHOGEN_TEST_CLOSE_SCARCE", if scarce { "1" } else { "0" });
            if inject {
                command.env("DYLD_INSERT_LIBRARIES", preload);
            }
            let result = command.output().unwrap();
            assert!(result.status.success(), "{mode}: {result:?}");
            assert!(result.stderr.is_empty(), "{mode}: {result:?}");
            let prepared = directories(&case.join("compiler-provenance"));
            assert_eq!(prepared.len(), 1);
            let prepared = prepared[0].join("observer");
            let image = fs::read(&prepared).unwrap();
            assert!(!image.is_empty());
            let sessions = directories(&target.join(".archogen-provenance"));
            assert_eq!(sessions.len(), 2);
            for directory in &sessions {
                let installed = directory.join("observer");
                assert_eq!(fs::read(&installed).unwrap(), image);
                let metadata = fs::symlink_metadata(&installed).unwrap();
                assert!(metadata.is_file());
                assert_eq!(
                    metadata.permissions().mode(),
                    fs::metadata(&prepared).unwrap().permissions().mode()
                );
            }
            assert_eq!(
                fs::read(target.join("release")).unwrap(),
                b"owned fixture release"
            );
            let events = if inject {
                fs::read_to_string(&trace).unwrap()
            } else {
                assert!(!trace.exists());
                String::new()
            };
            if inject {
                assert_eq!(events.matches("loaded\n").count(), 1);
                assert_eq!(events.matches("permissions\n").count(), 2);
            }
            let mut boundary = None;
            if scarce {
                let filled = events
                    .lines()
                    .find(|line| line.starts_with("filled "))
                    .unwrap();
                let released = events
                    .lines()
                    .find(|line| line.starts_with("released "))
                    .unwrap();
                assert_eq!(
                    events
                        .lines()
                        .filter(|line| line.starts_with("filled "))
                        .count(),
                    1
                );
                assert_eq!(
                    events
                        .lines()
                        .filter(|line| line.starts_with("released "))
                        .count(),
                    1
                );
                let fill = fields(filled);
                let restore = fields(released);
                assert_eq!(fill.len(), 7);
                assert_eq!(restore.len(), 3);
                assert!((16..=64).contains(&fill["soft"]));
                assert_eq!(fill["open"], fill["soft"]);
                assert!(fill["prior"] >= fill["soft"]);
                assert!(fill["writer"] < fill["soft"]);
                assert!(fill["fillers"] > 0 && fill["fillers"] < fill["soft"]);
                assert_eq!(fill["errno"], 24);
                assert_eq!(restore["closed"], fill["fillers"]);
                assert_eq!(restore["soft"], fill["prior"]);
                assert_eq!(restore["hard"], fill["hard"]);
                boundary = Some((
                    fill["writer"],
                    events.find(filled).unwrap(),
                    events.find(released).unwrap(),
                ));
            } else if inject {
                assert!(!events.contains("filled "));
                assert_eq!(events.matches("release-pass\n").count(), 1);
            }
            // Judge allocation only after proving the actual resource state and cleanup.
            assert_eq!(
                result.stdout, b"accepted\nquery-ok\nrecovery-ok\n",
                "{mode}"
            );
            for directory in &sessions {
                assert_eq!(directories(&directory.join("records")).len(), 1);
            }
            if let Some((writer, filled, released)) = boundary {
                let inode = fs::metadata(sessions[0].join("observer")).unwrap().ino();
                let read = format!("readback fd={writer} inode={inode}\n");
                let read = events.find(&read).unwrap();
                assert!(filled < read && read < released);
                let recovery = fs::metadata(sessions[1].join("observer")).unwrap().ino();
                let recovered = events
                    .lines()
                    .find(|line| {
                        line.starts_with("readback ")
                            && line.ends_with(&format!(" inode={recovery}"))
                    })
                    .unwrap();
                assert!(released < events.find(recovered).unwrap());
            }
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn actual_installed_observer_corruption_is_refused_before_use_and_recovers() {
        crate::trust_test_process::bounded!(
            actual_installed_observer_corruption_is_refused_before_use_and_recovers
        );
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        fn directories(path: &std::path::Path) -> Vec<PathBuf> {
            let mut result = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            result.sort();
            result
        }
        let (out, _, session, base) = setup("install-readback");
        let source = out.join("install_fault.c");
        fs::write(
            &source,
            include_str!("../../scripts/fixtures/compiler_install_readback.c"),
        )
        .unwrap();
        let library = out.join("install_fault.dylib");
        let built = Command::new("cc")
            .args([
                "-Wall",
                "-Wextra",
                "-Werror",
                "-dynamiclib",
                "-Wl,-install_name,@rpath/install_fault.dylib",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&library)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        let canonical_library = fs::canonicalize(&library).unwrap();
        let preload = canonical_library.strip_prefix(&session.cwd).unwrap();
        let sources = out.join("component");
        fs::create_dir(&sources).unwrap();
        fs::write(
            sources.join("controller.rs"),
            include_str!("trust_compiler.rs"),
        )
        .unwrap();
        fs::write(sources.join("json.rs"), include_str!("json.rs")).unwrap();
        fs::write(sources.join("trust_compiler_observer.rs"), super::SOURCE).unwrap();
        fs::write(
            sources.join("probe.rs"),
            include_str!("../../scripts/fixtures/compiler_install_readback_probe.rs"),
        )
        .unwrap();
        let catalog = include_str!("catalog_build.rs");
        let start = catalog.find("pub fn environment(").unwrap();
        let tail = &catalog[start..];
        let end = tail.find("\nfn command(").unwrap();
        fs::write(sources.join("base_environment.rs"), &tail[..end]).unwrap();
        let helper = sources.join("probe");
        let built = Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "install_component"])
            .arg(sources.join("probe.rs"))
            .arg("-o")
            .arg(&helper)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        for (mode, inject, corrupt) in [
            ("baseline", false, false),
            ("corrupt", true, true),
            ("pass", true, false),
        ] {
            let case = out.join(mode);
            let target = case.join("target");
            fs::create_dir_all(&target).unwrap();
            let trace = case.join("trace");
            let mut command = Command::new(&helper);
            command
                .arg(&case)
                .arg(out.join("cargo-home"))
                .arg(if corrupt { "corrupt" } else { "pass" })
                .current_dir(&session.cwd)
                .env_clear()
                .envs(base.iter().cloned())
                .env(
                    "ARCHOGEN_TEST_INSTALL_ROOT",
                    fs::canonicalize(&target).unwrap(),
                )
                .env("ARCHOGEN_TEST_INSTALL_TRACE", &trace)
                .env(
                    "ARCHOGEN_TEST_INSTALL_CORRUPT",
                    if corrupt { "1" } else { "0" },
                );
            if inject {
                command.env("DYLD_INSERT_LIBRARIES", preload);
            }
            let result = command.output().unwrap();
            assert!(result.status.success(), "{mode}: {result:?}");
            assert!(result.stderr.is_empty(), "{mode}: {result:?}");
            let prepared = directories(&case.join("compiler-provenance"));
            assert_eq!(prepared.len(), 1);
            let prepared = prepared[0].join("observer");
            let image = fs::read(&prepared).unwrap();
            assert!(!image.is_empty());
            let sessions = directories(&target.join(".archogen-provenance"));
            assert!(!sessions.is_empty());
            let first = sessions[0].join("observer");
            let mut expected = image.clone();
            if corrupt {
                expected[0] ^= 1;
            }
            // Establish the same real byte effect before judging acceptance or recovery.
            assert_eq!(fs::read(&first).unwrap(), expected, "{mode}");
            let events = if inject {
                fs::read_to_string(&trace).unwrap()
            } else {
                assert!(!trace.exists());
                String::new()
            };
            if inject {
                assert_eq!(events.matches("loaded inode=0\n").count(), 1);
                assert_eq!(
                    events.matches("corrupt inode=").count(),
                    usize::from(corrupt)
                );
                if corrupt {
                    assert!(events.contains(&format!(
                        "corrupt inode={}\n",
                        fs::metadata(&first).unwrap().ino()
                    )));
                }
            }
            let stdout = if corrupt {
                "refused:installed compiler provenance observer differs\nrecovery-ok\n"
            } else {
                "accepted\nquery-ok\n"
            };
            assert_eq!(result.stdout, stdout.as_bytes(), "{mode}");
            assert_eq!(sessions.len(), if corrupt { 2 } else { 1 });
            for (index, directory) in sessions.iter().enumerate() {
                let file = directory.join("observer");
                let metadata = fs::symlink_metadata(&file).unwrap();
                assert!(metadata.is_file());
                assert_eq!(
                    metadata.permissions().mode(),
                    fs::metadata(&prepared).unwrap().permissions().mode()
                );
                let mut expected = image.clone();
                if corrupt && index == 0 {
                    expected[0] ^= 1;
                }
                assert_eq!(fs::read(&file).unwrap(), expected, "{mode}/{index}");
                if inject {
                    let permissions = format!("permissions inode={}\n", metadata.ino());
                    let readback = format!("readback inode={}\n", metadata.ino());
                    assert!(
                        events.contains(&permissions) && events.contains(&readback),
                        "{mode}/{index}: {events}"
                    );
                    assert!(events.find(&permissions).unwrap() < events.find(&readback).unwrap());
                }
                let records = directory.join("records");
                if corrupt && index == 0 {
                    assert!(!records.exists());
                } else {
                    assert_eq!(directories(&records).len(), 1);
                }
            }
        }
    }

    #[test]
    fn actual_preparation_holds_status_output_and_selected_environment() {
        crate::trust_test_process::bounded!(
            actual_preparation_holds_status_output_and_selected_environment
        );
        use std::collections::{BTreeMap, BTreeSet};
        fn environment_record(bytes: &[u8]) -> BTreeMap<Vec<u8>, Vec<u8>> {
            assert_eq!(bytes.last(), Some(&0));
            let mut result = BTreeMap::new();
            for field in bytes[..bytes.len() - 1].split(|byte| *byte == 0) {
                let equals = field.iter().position(|byte| *byte == b'=').unwrap();
                assert!(result
                    .insert(field[..equals].to_vec(), field[equals + 1..].to_vec())
                    .is_none());
            }
            result
        }
        let (out, compiler, session, base) = setup("prepare-component");
        let bin = out.join("callee");
        fs::create_dir(&bin).unwrap();
        native_fixture(
            &bin,
            "rustc",
            include_str!("../../scripts/fixtures/compiler_preparation_callee.c"),
            &base,
        );
        fs::copy(&compiler.executable, bin.join("seed-observer")).unwrap();
        let canonical_bin = fs::canonicalize(&bin).unwrap();
        // The relative PATH component also works when the repository root contains a colon.
        let relative_bin = canonical_bin.strip_prefix(&session.cwd).unwrap();
        let sources = out.join("component");
        fs::create_dir(&sources).unwrap();
        fs::write(
            sources.join("controller.rs"),
            include_str!("trust_compiler.rs"),
        )
        .unwrap();
        fs::write(sources.join("json.rs"), include_str!("json.rs")).unwrap();
        fs::write(sources.join("trust_compiler_observer.rs"), super::SOURCE).unwrap();
        fs::write(
            sources.join("probe.rs"),
            include_str!("../../scripts/fixtures/compiler_preparation_probe.rs"),
        )
        .unwrap();
        // Keep the real allowlist constructor; the helper changes only executable selection.
        let catalog = include_str!("catalog_build.rs");
        let start = catalog.find("pub fn environment(").unwrap();
        let tail = &catalog[start..];
        let end = tail.find("\nfn command(").unwrap();
        let selected =
            tail[..end].replacen("pub fn environment(", "pub fn selected_environment(", 1);
        fs::write(sources.join("base_environment.rs"), selected).unwrap();
        let helper = sources.join("probe");
        let built = Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "prepare_component"])
            .arg(sources.join("probe.rs"))
            .arg("-o")
            .arg(&helper)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        let expected_keys: BTreeSet<_> = [
            "PATH",
            "HOME",
            "RUSTUP_HOME",
            "RUSTUP_TOOLCHAIN",
            "CARGO_HOME",
            "CARGO_TARGET_DIR",
            "TMPDIR",
            "TMP",
            "TEMP",
        ]
        .into_iter()
        .map(|key| key.as_bytes().to_vec())
        .collect();
        for (mode, expected) in [
            (
                "failed",
                "refused:compiler provenance observer build failed: controlled compiler failure\n",
            ),
            ("empty", "refused:compiler provenance observer is empty\n"),
            ("success", "accepted\nquery-ok\n"),
        ] {
            let case = out.join(mode);
            let result = Command::new(&helper)
                .arg(&case)
                .arg(out.join("cargo-home"))
                .current_dir(&session.cwd)
                .env_clear()
                .envs(base.iter().cloned())
                .env("ARCHOGEN_TEST_COMPILER_BIN", relative_bin)
                .env("ARCHOGEN_TEST_UNDECLARED", "must-not-reach-callee")
                .output()
                .unwrap();
            assert!(result.status.success(), "{mode}: {result:?}");
            assert!(result.stderr.is_empty(), "{mode}: {result:?}");
            let directories: Vec<_> = fs::read_dir(case.join("compiler-provenance"))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect();
            assert_eq!(directories.len(), 1, "{mode}");
            let directory = &directories[0];
            let selected =
                environment_record(&fs::read(directory.join("selected-environment.bin")).unwrap());
            assert_eq!(
                selected.keys().cloned().collect::<BTreeSet<_>>(),
                expected_keys
            );
            let received =
                environment_record(&fs::read(directory.join("environment.bin")).unwrap());
            assert_eq!(
                received, selected,
                "{mode}: selected environment was not forwarded"
            );
            assert_eq!(result.stdout, expected.as_bytes(), "{mode}");
            let output = fs::read(directory.join("observer")).unwrap();
            if mode == "empty" {
                assert!(output.is_empty());
            } else {
                assert_eq!(output, compiler.bytes, "{mode}");
            }
        }
    }

    #[test]
    fn actual_observer_sync_failure_retains_partial_evidence_and_recovers() {
        crate::trust_test_process::bounded!(
            actual_observer_sync_failure_retains_partial_evidence_and_recovers
        );
        use std::process::Stdio;
        let (out, compiler, session, base) = setup("writer-sync");
        let source = out.join("io_fault.c");
        fs::write(
            &source,
            include_str!("../../scripts/fixtures/compiler_provenance_io_fault.c"),
        )
        .unwrap();
        let library = out.join(if cfg!(target_os = "macos") {
            "io_fault.dylib"
        } else {
            "io_fault.so"
        });
        let mut build = Command::new("cc");
        build.args(["-Wall", "-Wextra", "-Werror"]);
        if cfg!(target_os = "macos") {
            build.args(["-dynamiclib", "-Wl,-install_name,@rpath/io_fault.dylib"]);
        } else {
            build.args(["-shared", "-fPIC"]);
        }
        build.arg(source).arg("-o").arg(&library);
        if cfg!(target_os = "linux") {
            build.arg("-ldl");
        }
        let built = build
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(built.status.success(), "{built:?}");
        let canonical_library = fs::canonicalize(&library).unwrap();
        // A relative preload path avoids loader-list delimiters in a relocated repository root.
        let preload = canonical_library.strip_prefix(&session.cwd).unwrap();
        let expected = Command::new("rustc")
            .arg("-vV")
            .current_dir(&session.cwd)
            .env_clear()
            .envs(base.iter().cloned())
            .output()
            .unwrap();
        assert!(expected.status.success(), "{expected:?}");
        assert!(expected.stderr.is_empty());
        assert!(expected.stdout.starts_with(b"rustc 1.95.0"));
        let mut retained_failure = None;
        for (role, inject, fail) in [
            ("sync-no-library", false, false),
            ("sync-fail", true, true),
            ("sync-pass", true, false),
        ] {
            let current = compiler.session(role, &base, &session.cwd).unwrap();
            let trace = out.join(format!("{role}.trace"));
            let mut command = Command::new(&current.executable);
            command
                .args(["rustc", "-vV"])
                .current_dir(&current.cwd)
                .env_clear()
                .envs(current.environment(&base).unwrap())
                .env_remove("DYLD_INSERT_LIBRARIES")
                .env_remove("LD_PRELOAD")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            if inject {
                command
                    .env(
                        if cfg!(target_os = "macos") {
                            "DYLD_INSERT_LIBRARIES"
                        } else {
                            "LD_PRELOAD"
                        },
                        preload,
                    )
                    .env("ARCHOGEN_TEST_IO_TRACE", &trace)
                    .env("ARCHOGEN_TEST_IO_ERROR", if fail { "1" } else { "0" });
            }
            let child = command.spawn().unwrap();
            let pid = child.id();
            let result = child.wait_with_output().unwrap();
            assert_eq!(
                result.status.code(),
                Some(if fail { 2 } else { 0 }),
                "{role}: {result:?}"
            );
            assert_eq!(result.stdout, expected.stdout, "{role}");
            if fail {
                assert!(String::from_utf8_lossy(&result.stderr)
                    .contains("archogen compiler provenance:"));
            } else {
                assert!(result.stderr.is_empty(), "{role}: {result:?}");
            }
            let filename = if fail {
                format!(".{pid}.part")
            } else {
                format!("{pid}.json")
            };
            let files: Vec<_> = fs::read_dir(&current.records)
                .unwrap()
                .map(|entry| entry.unwrap().file_name())
                .collect();
            assert_eq!(files, [std::ffi::OsString::from(&filename)], "{role}");
            let path = current.records.join(filename);
            assert!(fs::symlink_metadata(&path).unwrap().file_type().is_file());
            let bytes = fs::read(&path).unwrap();
            let value = json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
            assert_eq!(value.get("pid"), Some(&Json::Number(pid.to_string())));
            assert_eq!(value.get("exit"), Some(&Json::Number("0".into())));
            assert_eq!(value.get("signal"), Some(&Json::Null));
            assert_eq!(
                value.get("schema"),
                Some(&Json::Str("archogen-compiler-provenance/1".into()))
            );
            assert_eq!(
                value.get("argv_hex"),
                Some(&Json::Array(vec![hex("rustc"), hex("-vV")]))
            );
            assert_eq!(value.get("session_hex"), Some(&hex(&current.identity)));
            assert_eq!(value.get("role_hex"), Some(&hex(role)));
            assert_eq!(
                value.get("cwd_hex"),
                Some(&hex(current.cwd.to_str().unwrap()))
            );
            if inject {
                let text = fs::read_to_string(&trace).unwrap();
                let identity = format!(" pid={pid} ");
                let own: Vec<_> = text.lines().collect();
                assert!(own.iter().all(|line| line.contains(&identity)), "{text}");
                assert_eq!(own.len(), 2, "{role}: {text}");
                assert_eq!(own[0], format!("loaded pid={pid} fd=-1"));
                let sync = if cfg!(target_os = "macos") {
                    "full-sync"
                } else {
                    "sync"
                };
                assert!(
                    own[1].starts_with(&format!("{sync} pid={pid} fd=")),
                    "{text}"
                );
                assert!(
                    own[1]
                        .rsplit_once(" fd=")
                        .unwrap()
                        .1
                        .parse::<i32>()
                        .unwrap()
                        >= 0
                );
            } else {
                assert!(!trace.exists());
            }
            if fail {
                assert!(current.invocations().is_err());
                assert_eq!(fs::read(&path).unwrap(), bytes);
                retained_failure = Some((path, bytes));
            } else {
                assert!(current.invocations().unwrap().is_empty());
            }
        }
        let recovered = compiler
            .session("sync-recovery", &base, &session.cwd)
            .unwrap();
        let result = run(&recovered, &base, &["rustc", "-vV"]);
        assert!(result.status.success(), "{result:?}");
        assert!(recovered.invocations().unwrap().is_empty());
        assert_eq!(fs::read_dir(&recovered.records).unwrap().count(), 1);
        let (path, bytes) = retained_failure.unwrap();
        assert_eq!(fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn actual_observer_setup_and_installation_fail_before_child_execution() {
        crate::trust_test_process::bounded!(
            actual_observer_setup_and_installation_fail_before_child_execution
        );
        use std::os::unix::fs::PermissionsExt;
        let (out, compiler, session, base) = setup("writer-setup");
        let collision = native_fixture(
            &out,
            "collision",
            include_str!("../../scripts/fixtures/compiler_provenance_collision.c"),
            &base,
        );
        let blocked = out.join("not-a-directory");
        fs::write(&blocked, b"held file").unwrap();
        assert!(Compiler::prepare(&blocked, "1.95.0", &out.join("cargo-home")).is_err());
        let missing_target: Vec<_> = base
            .iter()
            .filter(|(key, _)| key != "CARGO_TARGET_DIR")
            .cloned()
            .collect();
        assert!(compiler
            .session("no-target", &missing_target, &session.cwd)
            .err()
            .unwrap()
            .contains("CARGO_TARGET_DIR"));
        let mut bad_target = base.clone();
        bad_target
            .iter_mut()
            .find(|(key, _)| key == "CARGO_TARGET_DIR")
            .unwrap()
            .1 = blocked.to_str().unwrap().to_owned();
        assert!(compiler
            .session("blocked-target", &bad_target, &session.cwd)
            .is_err());
        let mut relative_target = base.clone();
        relative_target
            .iter_mut()
            .find(|(key, _)| key == "CARGO_TARGET_DIR")
            .unwrap()
            .1 = "relative-target".into();
        let relative = compiler
            .session("relative-target", &relative_target, &out)
            .unwrap();
        assert!(relative.records.starts_with(out.join("relative-target")));
        assert_eq!(relative.cwd, out.canonicalize().unwrap());
        assert_eq!(fs::read(&relative.executable).unwrap(), compiler.bytes);
        let mode = fs::metadata(&relative.executable)
            .unwrap()
            .permissions()
            .mode()
            & 0o111;
        assert_ne!(mode, 0);
        assert_eq!(
            mode,
            fs::metadata(&compiler.executable)
                .unwrap()
                .permissions()
                .mode()
                & 0o111
        );
        let held = out.join("held-observer");
        fs::rename(&compiler.executable, &held).unwrap();
        let missing_source = compiler.session("missing-install-source", &base, &session.cwd);
        fs::rename(&held, &compiler.executable).unwrap();
        assert!(missing_source.is_err());
        let missing_argument = run(&session, &base, &[]);
        assert_eq!(missing_argument.status.code(), Some(2));
        assert!(String::from_utf8(missing_argument.stderr)
            .unwrap()
            .contains("missing compiler argument"));
        assert_eq!(fs::read_dir(&session.records).unwrap().count(), 0);
        for (key, reason) in [
            ("ARCHOGEN_COMPILER_RECORDS", "missing records"),
            ("ARCHOGEN_COMPILER_SESSION", "missing session"),
            ("ARCHOGEN_COMPILER_ROLE", "missing role"),
            ("records-is-file", "archogen compiler provenance:"),
        ] {
            let fresh = compiler.session(key, &base, &session.cwd).unwrap();
            let mut command = Command::new(&fresh.executable);
            command
                .arg(&collision)
                .arg(&fresh.records)
                .current_dir(&fresh.cwd)
                .env_clear()
                .envs(fresh.environment(&base).unwrap());
            if key == "records-is-file" {
                command.env("ARCHOGEN_COMPILER_RECORDS", &blocked);
            } else {
                command.env_remove(key);
            }
            let result = command.output().unwrap();
            assert_eq!(result.status.code(), Some(2), "{key}: {result:?}");
            assert!(result.stdout.is_empty(), "the native child ran for {key}");
            assert!(String::from_utf8(result.stderr).unwrap().contains(reason));
            assert_eq!(fs::read_dir(&fresh.records).unwrap().count(), 0, "{key}");
            assert_eq!(fs::read(&blocked).unwrap(), b"held file");
        }
        let recovered = compiler
            .session("setup-recovery", &base, &session.cwd)
            .unwrap();
        assert!(run(&recovered, &base, &["rustc", "-vV"]).status.success());
        assert!(recovered.invocations().unwrap().is_empty());
    }

    #[test]
    fn actual_observer_publication_collision_preserves_completed_and_partial_evidence() {
        crate::trust_test_process::bounded!(
            actual_observer_publication_collision_preserves_completed_and_partial_evidence
        );
        let (out, compiler, session, base) = setup("writer-publication");
        let collision = native_fixture(
            &out,
            "collision",
            include_str!("../../scripts/fixtures/compiler_provenance_collision.c"),
            &base,
        );
        let result = run(
            &session,
            &base,
            &[
                collision.to_str().unwrap(),
                session.records.to_str().unwrap(),
            ],
        );
        assert_eq!(result.status.code(), Some(2));
        assert_eq!(result.stdout, b"native compiler completed\n");
        assert!(String::from_utf8(result.stderr)
            .unwrap()
            .contains("archogen compiler provenance:"));
        let files: std::collections::BTreeMap<_, _> = fs::read_dir(&session.records)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (
                    path.file_name().unwrap().to_owned(),
                    fs::read(path).unwrap(),
                )
            })
            .collect();
        assert_eq!(files.len(), 2);
        let partial = files
            .iter()
            .find(|(name, _)| name.to_str().unwrap().ends_with(".part"))
            .unwrap();
        let value = json::parse(std::str::from_utf8(partial.1).unwrap()).unwrap();
        let Some(Json::Number(pid)) = value.get("pid") else {
            panic!("native PID")
        };
        assert_eq!(partial.0.to_str().unwrap(), format!(".{pid}.part"));
        assert_eq!(value.get("exit"), Some(&Json::Number("0".into())));
        assert_eq!(value.get("signal"), Some(&Json::Null));
        assert_eq!(
            files
                .get(std::ffi::OsStr::new(&format!("{pid}.json")))
                .unwrap(),
            b"retained native evidence\n"
        );
        assert!(session.invocations().is_err());
        for (name, bytes) in files {
            assert_eq!(fs::read(session.records.join(name)).unwrap(), bytes);
        }
        let recovered = compiler
            .session("publication-recovery", &base, &session.cwd)
            .unwrap();
        assert!(run(&recovered, &base, &["rustc", "-vV"]).status.success());
        assert!(recovered.invocations().unwrap().is_empty());
    }

    #[test]
    fn actual_observer_records_native_signal_and_recovers() {
        crate::trust_test_process::bounded!(actual_observer_records_native_signal_and_recovers);
        let (out, compiler, session, base) = setup("writer-signal");
        let source = include_str!("../../scripts/fixtures/compiler_provenance_receiver.c")
            .replace("ARCHOGEN_PROTO_TRACE_DIR", "ARCHOGEN_COMPILER_RECORDS")
            .replace("ARCHOGEN_PROTO_ROLE", "ARCHOGEN_COMPILER_ROLE")
            .replace("ARCHOGEN_PROTO_SESSION", "ARCHOGEN_COMPILER_SESSION");
        let receiver = native_fixture(&out, "receiver", &source, &base);
        let sink = out.join("signal.argv");
        let result = Command::new(&session.executable)
            .arg(&receiver)
            .current_dir(&session.cwd)
            .env_clear()
            .envs(session.environment(&base).unwrap())
            .env("ARGV_SINK", &sink)
            .env("ARGV_SIGNAL", "1")
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(143));
        assert!(result.stdout.is_empty() && result.stderr.is_empty());
        use std::os::unix::ffi::OsStrExt;
        assert_eq!(
            fs::read(sink).unwrap(),
            [receiver.as_os_str().as_bytes(), &[0]].concat()
        );
        let records: Vec<_> = fs::read_dir(&session.records)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        assert_eq!(records.len(), 1);
        let record = json::parse(&fs::read_to_string(&records[0]).unwrap()).unwrap();
        assert_eq!(record.get("exit"), Some(&Json::Null));
        assert_eq!(record.get("signal"), Some(&Json::Number("15".into())));
        assert!(session.invocations().unwrap_err().contains("unsuccessful"));
        let recovered = compiler
            .session("signal-recovery", &base, &session.cwd)
            .unwrap();
        assert!(run(&recovered, &base, &["rustc", "-vV"]).status.success());
        assert!(recovered.invocations().unwrap().is_empty());
    }

    #[test]
    fn actual_concurrent_observer_writers_publish_independent_complete_records() {
        crate::trust_test_process::bounded!(
            actual_concurrent_observer_writers_publish_independent_complete_records
        );
        use std::{
            collections::BTreeSet,
            io::Write,
            process::Stdio,
            time::{Duration, Instant},
        };
        let (out, compiler, session, base) = setup("concurrent-writers");
        let environment = session.environment(&base).unwrap();
        let mut expected = Vec::new();
        let mut binaries = Vec::new();
        for index in 0..8 {
            let name = format!("concurrent_{index}");
            let directory = out.join(&name);
            fs::create_dir(&directory).unwrap();
            binaries.push(directory.join(&name));
            expected.push(vec![
                "rustc".to_owned(),
                "--crate-name".into(),
                name,
                "-".into(),
                "--emit=dep-info,link".into(),
                "--out-dir".into(),
                directory.to_str().unwrap().to_owned(),
            ]);
        }
        let mut children = Vec::new();
        let mut pids = BTreeSet::new();
        // Do not assert while a child owns an unreleased stdin. Every error below goes through
        // the common close-and-wait path before an assertion can unwind this test.
        let launched = (|| -> Result<(), String> {
            for arguments in &expected {
                let child = Command::new(&session.executable)
                    .args(arguments)
                    .current_dir(&session.cwd)
                    .env_clear()
                    .envs(environment.clone())
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .map_err(|error| error.to_string())?;
                pids.insert(child.id());
                children.push(child);
            }
            let reservations: BTreeSet<_> = pids
                .iter()
                .map(|pid| std::ffi::OsString::from(format!(".{pid}.part")))
                .collect();
            let deadline = Instant::now() + Duration::from_secs(5);
            loop {
                let names: BTreeSet<_> = fs::read_dir(&session.records)
                    .map_err(|error| error.to_string())?
                    .map(|entry| entry.map(|entry| entry.file_name()))
                    .collect::<Result<_, _>>()
                    .map_err(|error| error.to_string())?;
                if names == reservations {
                    for name in &names {
                        let metadata = fs::symlink_metadata(session.records.join(name))
                            .map_err(|error| error.to_string())?;
                        if !metadata.is_file() || metadata.len() != 0 {
                            return Err("a writer completed before source release".into());
                        }
                    }
                    break;
                }
                if Instant::now() >= deadline {
                    return Err(format!("concurrent reservations differ: {names:?}"));
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            for child in &mut children {
                if child
                    .try_wait()
                    .map_err(|error| error.to_string())?
                    .is_some()
                {
                    return Err("a writer exited before source release".into());
                }
            }
            for (index, child) in children.iter_mut().enumerate() {
                let source = format!(
                    r#"
const _: () = assert!(option_env!("RUSTC_WRAPPER").is_none());
const _: () = assert!(option_env!("ARCHOGEN_COMPILER_RECORDS").is_none());
const _: () = assert!(option_env!("ARCHOGEN_COMPILER_SESSION").is_none());
const _: () = assert!(option_env!("ARCHOGEN_COMPILER_ROLE").is_none());
fn main() {{ println!("worker-{index}"); }}
"#
                );
                child
                    .stdin
                    .as_mut()
                    .ok_or("missing writer stdin")?
                    .write_all(source.as_bytes())
                    .map_err(|error| error.to_string())?;
            }
            Ok(())
        })();
        // Release all compiler inputs, even when a spawn/reservation/write failed, and reap
        // every observer before examining results. The bounded outer worker guards a hang.
        for child in &mut children {
            drop(child.stdin.take());
        }
        let outputs: Vec<_> = children
            .into_iter()
            .map(|child| child.wait_with_output())
            .collect();
        assert!(launched.is_ok(), "{launched:?}");
        assert_eq!(pids.len(), 8);
        for output in outputs {
            let output = output.unwrap();
            assert!(output.status.success(), "{output:?}");
            assert!(output.stdout.is_empty() && output.stderr.is_empty());
        }
        let mut observed = session.invocations().unwrap();
        observed.sort();
        expected.sort();
        assert_eq!(observed, expected);
        let files: Vec<_> = fs::read_dir(&session.records)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        assert_eq!(files.len(), 8);
        let mut recorded = BTreeSet::new();
        for file in files {
            let value = json::parse(&fs::read_to_string(&file).unwrap()).unwrap();
            let Some(Json::Number(pid)) = value.get("pid") else {
                panic!("native PID")
            };
            let pid = pid.parse::<u32>().unwrap();
            assert!(pids.contains(&pid) && recorded.insert(pid));
            assert_eq!(
                file.file_name().unwrap(),
                std::ffi::OsStr::new(&format!("{pid}.json"))
            );
            assert_eq!(value.get("exit"), Some(&Json::Number("0".into())));
            assert_eq!(value.get("signal"), Some(&Json::Null));
            assert_eq!(value.get("role_hex"), Some(&hex(&session.role)));
            assert_eq!(value.get("session_hex"), Some(&hex(&session.identity)));
            assert_eq!(
                value.get("cwd_hex"),
                Some(&hex(session.cwd.to_str().unwrap()))
            );
        }
        assert_eq!(recorded, pids);
        for (index, binary) in binaries.iter().enumerate() {
            let output = Command::new(binary).output().unwrap();
            assert!(output.status.success());
            assert_eq!(output.stdout, format!("worker-{index}\n").as_bytes());
        }
        let fresh = compiler
            .session("after-concurrent-role", &base, &session.cwd)
            .unwrap();
        assert_ne!(fresh.records, session.records);
        assert_ne!(fresh.identity, session.identity);
        assert_ne!(fresh.role, session.role);
        assert!(run(&fresh, &base, &["rustc", "-vV"]).status.success());
        assert!(fresh.invocations().unwrap().is_empty());
        assert_eq!(fs::read_dir(&fresh.records).unwrap().count(), 1);
        assert_eq!(fs::read_dir(&session.records).unwrap().count(), 8);
    }

    #[test]
    fn actual_record_validation_applies_to_every_real_record() {
        crate::trust_test_process::bounded!(actual_record_validation_applies_to_every_real_record);
        use std::collections::BTreeMap;
        fn valid(session: &Session, expected: &[Vec<String>]) {
            let mut actual = session.invocations().unwrap();
            actual.sort();
            let mut expected = expected.to_vec();
            expected.sort();
            assert_eq!(actual, expected);
        }
        fn snapshot(session: &Session) -> BTreeMap<PathBuf, Vec<u8>> {
            fs::read_dir(&session.records)
                .unwrap()
                .map(|entry| {
                    let path = entry.unwrap().path();
                    assert!(fs::symlink_metadata(&path).unwrap().file_type().is_file());
                    let bytes = fs::read(&path).unwrap();
                    (path, bytes)
                })
                .collect()
        }
        let (out, _, session, base) = setup("record-positions");
        let mut expected = Vec::new();
        for _ in 0..3 {
            let result = run(&session, &base, &["rustc", "-vV"]);
            assert!(result.status.success(), "{result:?}");
            assert!(result.stderr.is_empty());
            assert!(result.stdout.starts_with(b"rustc 1.95.0 "));
        }
        let source = out.join("position.rs");
        fs::write(&source, "fn main() { println!(\"record-position\"); }\n").unwrap();
        for index in 0..2 {
            let name = format!("position_{index}");
            let compiled = out.join(format!("compiled-{index}"));
            fs::create_dir(&compiled).unwrap();
            let arguments = [
                "rustc",
                "--crate-name",
                &name,
                "--emit=dep-info,link",
                "--out-dir",
                compiled.to_str().unwrap(),
                source.to_str().unwrap(),
            ];
            let result = run(&session, &base, &arguments);
            assert!(result.status.success(), "{result:?}");
            assert!(result.stderr.is_empty());
            expected.push(arguments.map(str::to_owned).to_vec());
            let result = Command::new(compiled.join(&name)).output().unwrap();
            assert!(result.status.success());
            assert_eq!(result.stdout, b"record-position\n");
        }
        valid(&session, &expected);
        let seeded = snapshot(&session);
        assert_eq!(seeded.len(), 5);
        let held_directory = session.records.parent().unwrap().join("held-records");
        fs::create_dir(&held_directory).unwrap();
        for (file, bytes) in &seeded {
            valid(&session, &expected);
            let name = file.file_name().unwrap();
            let held = held_directory.join(name);
            fs::rename(file, &held).unwrap();
            let relative = PathBuf::from("..").join("held-records").join(name);
            std::os::unix::fs::symlink(&relative, file).unwrap();
            assert!(!fs::read_link(file).unwrap().is_absolute());
            assert_eq!(fs::read(file).unwrap(), *bytes, "valid symlink target");
            // Add a valid record after the link, also exposing insertion-order endpoint guards.
            let result = run(&session, &base, &["rustc", "-vV"]);
            assert!(result.status.success(), "{result:?}");
            let error = session
                .invocations()
                .expect_err("symlink in a real record set");
            assert!(error.contains("incomplete"), "{error}");
            assert_eq!(fs::read(&held).unwrap(), *bytes);
            assert_eq!(fs::read_link(file).unwrap(), relative);
            fs::remove_file(file).unwrap();
            fs::rename(&held, file).unwrap();
            valid(&session, &expected);
        }
        let originals = snapshot(&session);
        assert_eq!(originals.len(), 10);
        for (fault, reason) in [
            ("extra-field", "unexpected fields"),
            ("noncanonical", "canonical"),
            ("filename", "filename"),
            ("compiler", "another compiler"),
            ("query", "unknown"),
        ] {
            for (file, bytes) in &originals {
                valid(&session, &expected);
                let mut value = json::parse(std::str::from_utf8(bytes).unwrap()).unwrap();
                let Json::Object(fields) = &mut value else {
                    panic!("producer object")
                };
                let raw = match fault {
                    "extra-field" => {
                        fields.insert("extra".into(), Json::Null);
                        None
                    }
                    "noncanonical" => Some([b" ".as_slice(), bytes].concat()),
                    "filename" => {
                        let Some(Json::Number(pid)) = fields.get("pid") else {
                            panic!("producer pid")
                        };
                        let other = pid.parse::<u32>().unwrap().checked_add(1).unwrap();
                        fields.insert("pid".into(), Json::Number(other.to_string()));
                        None
                    }
                    "compiler" => {
                        let Some(Json::Array(arguments)) = fields.get_mut("argv_hex") else {
                            panic!("producer argv")
                        };
                        arguments[0] = hex("another");
                        None
                    }
                    "query" => {
                        fields.insert(
                            "argv_hex".into(),
                            Json::Array(vec![hex("rustc"), hex("--unknown-query")]),
                        );
                        None
                    }
                    _ => unreachable!(),
                };
                let changed = raw.unwrap_or_else(|| (json::write(&value) + "\n").into_bytes());
                fs::write(file, &changed).unwrap();
                let error = session.invocations().expect_err(fault);
                assert!(
                    error.contains(reason),
                    "{fault}, {}: {error}",
                    file.display()
                );
                assert_eq!(
                    fs::read(file).unwrap(),
                    changed,
                    "refusal preserves evidence"
                );
                fs::write(file, bytes).unwrap();
                valid(&session, &expected);
                assert_eq!(snapshot(&session), originals, "{fault} recovery");
            }
        }
        assert_eq!(snapshot(&session), originals);
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
                    let relative = PathBuf::from("..").join(original.strip_prefix(&out).unwrap());
                    std::os::unix::fs::symlink(&relative, &file).unwrap();
                    assert!(!fs::read_link(&file).unwrap().is_absolute());
                    assert_eq!(fs::read(&file).unwrap(), bytes);
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
    fn actual_compilation_records_refuse_internal_nul_in_argument_values() {
        crate::trust_test_process::bounded!(
            actual_compilation_records_refuse_internal_nul_in_argument_values
        );
        let (out, _, session, base) = setup("internal-nul");
        let source = out.join("probe.rs");
        let compiled = out.join("compiled");
        fs::create_dir(&compiled).unwrap();
        fs::write(&source, "fn main() { println!(\"nul-control\"); }\n").unwrap();
        let arguments = [
            "rustc",
            "--crate-name",
            "nul_control",
            "--emit=dep-info,link",
            "--out-dir",
            compiled.to_str().unwrap(),
            source.to_str().unwrap(),
        ];
        let result = run(&session, &base, &arguments);
        assert!(result.status.success(), "{result:?}");
        assert!(result.stderr.is_empty());
        let expected = vec![arguments.map(str::to_owned).to_vec()];
        assert_eq!(session.invocations().unwrap(), expected);
        let result = Command::new(compiled.join("nul_control")).output().unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, b"nul-control\n");
        let files = fs::read_dir(&session.records)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 1);
        let file = &files[0];
        let bytes = fs::read(file).unwrap();
        for index in [2, 5, 6] {
            assert_eq!(session.invocations().unwrap(), expected);
            let mut value = json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
            let Json::Object(fields) = &mut value else {
                panic!("producer object")
            };
            let Some(Json::Array(encoded)) = fields.get_mut("argv_hex") else {
                panic!("producer argv")
            };
            assert_eq!(encoded[index], hex(arguments[index]));
            let mut argument = arguments[index].to_owned();
            let middle = argument
                .char_indices()
                .nth(argument.chars().count() / 2)
                .unwrap()
                .0;
            assert!(middle > 0 && middle < argument.len());
            argument.insert(middle, '\0');
            assert_ne!(argument.as_bytes().first(), Some(&0));
            assert_ne!(argument.as_bytes().last(), Some(&0));
            encoded[index] = hex(&argument);
            let changed = json::write(&value) + "\n";
            fs::write(file, &changed).unwrap();
            let error = session
                .invocations()
                .expect_err("internal NUL in non-marker value");
            assert!(error.contains("NUL in argument"), "{index}: {error}");
            assert_eq!(fs::read(file).unwrap(), changed.as_bytes());
            fs::write(file, &bytes).unwrap();
            assert_eq!(session.invocations().unwrap(), expected);
            assert_eq!(fs::read(file).unwrap(), bytes);
        }
    }

    #[test]
    fn actual_compilation_records_require_the_exact_emit_marker() {
        crate::trust_test_process::bounded!(
            actual_compilation_records_require_the_exact_emit_marker
        );
        let (out, _, session, base) = setup("emit-marker");
        let source = out.join("probe.rs");
        let compiled = out.join("compiled");
        fs::create_dir(&compiled).unwrap();
        fs::write(&source, "fn main() { println!(\"emit-control\"); }\n").unwrap();
        let arguments = [
            "rustc",
            "--crate-name",
            "emit_control",
            "--emit=dep-info,link",
            "--out-dir",
            compiled.to_str().unwrap(),
            source.to_str().unwrap(),
        ];
        let result = run(&session, &base, &arguments);
        assert!(result.status.success(), "{result:?}");
        assert!(result.stderr.is_empty());
        let expected = vec![arguments.map(str::to_owned).to_vec()];
        assert_eq!(session.invocations().unwrap(), expected);
        let result = Command::new(compiled.join("emit_control"))
            .output()
            .unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, b"emit-control\n");
        let files = fs::read_dir(&session.records)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 1);
        let file = &files[0];
        let bytes = fs::read(file).unwrap();
        for marker in [
            "prefix--emit=dep-info,link",
            "--emit-extra=dep-info,link",
            "--emit",
        ] {
            assert_eq!(session.invocations().unwrap(), expected);
            let mut value = json::parse(std::str::from_utf8(&bytes).unwrap()).unwrap();
            let Json::Object(fields) = &mut value else {
                panic!("producer object")
            };
            let Some(Json::Array(encoded)) = fields.get_mut("argv_hex") else {
                panic!("producer argv")
            };
            assert_eq!(encoded[3], hex("--emit=dep-info,link"));
            encoded[3] = hex(marker);
            let changed = json::write(&value) + "\n";
            fs::write(file, &changed).unwrap();
            let error = session.invocations().expect_err("invalid emit marker");
            assert!(
                error.contains("unknown compiler provenance invocation shape"),
                "{marker}: {error}"
            );
            assert_eq!(fs::read(file).unwrap(), changed.as_bytes());
            fs::write(file, &bytes).unwrap();
            assert_eq!(session.invocations().unwrap(), expected);
            assert_eq!(fs::read(file).unwrap(), bytes);
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
