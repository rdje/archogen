#![allow(dead_code)]

mod controller;
mod json;

mod catalog_build {
    use std::{fs, io::Write, path::Path};

    include!("base_environment.rs");

    pub fn environment(
        pin: &str,
        cargo_home: &Path,
        target: &Path,
    ) -> Result<Vec<(String, String)>, String> {
        let mut selected = selected_environment(pin, cargo_home, target)?;
        let bin = std::env::var("ARCHOGEN_TEST_COMPILER_BIN").map_err(|error| error.to_string())?;
        let (_, path) = selected.iter_mut().find(|(key, _)| key == "PATH").unwrap();
        *path = format!("{bin}:{path}");
        let mut receipt = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(target.join("selected-environment.bin"))
            .map_err(|error| error.to_string())?;
        for (key, value) in &selected {
            receipt.write_all(key.as_bytes()).unwrap();
            receipt.write_all(b"=").unwrap();
            receipt.write_all(value.as_bytes()).unwrap();
            receipt.write_all(b"\0").unwrap();
        }
        Ok(selected)
    }
}

fn main() {
    use std::{path::PathBuf, process::Command};
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    assert_eq!(arguments.len(), 2);
    let out = PathBuf::from(&arguments[0]);
    let home = PathBuf::from(&arguments[1]);
    match controller::Compiler::prepare(&out, "1.95.0", &home) {
        Err(reason) => println!("refused:{reason}"),
        Ok(compiler) => {
            println!("accepted");
            if out.file_name().unwrap() != "success" {
                return;
            }
            let cwd = std::env::current_dir().unwrap();
            let base = catalog_build::selected_environment("1.95.0", &home, &out.join("roundtrip"))
                .unwrap();
            let expected = Command::new("rustc")
                .arg("-vV")
                .env_clear()
                .envs(base.iter().cloned())
                .output()
                .unwrap();
            assert!(expected.status.success());
            assert!(expected.stderr.is_empty());
            assert!(expected.stdout.starts_with(b"rustc 1.95.0 "));
            let session = compiler
                .session("controlled-positive", &base, &cwd)
                .unwrap();
            let environment = session.environment(&base).unwrap();
            let executable = &environment
                .iter()
                .find(|(key, _)| key == "RUSTC_WRAPPER")
                .unwrap()
                .1;
            let result = Command::new(executable)
                .args(["rustc", "-vV"])
                .env_clear()
                .envs(environment)
                .output()
                .unwrap();
            assert!(result.status.success());
            assert!(result.stderr.is_empty());
            assert_eq!(result.stdout, expected.stdout);
            assert!(session.invocations().unwrap().is_empty());
            println!("query-ok");
        }
    }
}
