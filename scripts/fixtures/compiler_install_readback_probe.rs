#![allow(dead_code)]
mod controller;
mod json;
mod catalog_build {
    use std::{fs, path::Path};
    include!("base_environment.rs");
}

fn query(session: &controller::Session, base: &[(String, String)]) {
    use std::process::Command;
    let expected = Command::new("rustc")
        .arg("-vV")
        .env_clear()
        .envs(base.iter().cloned())
        .output()
        .unwrap();
    assert!(expected.status.success() && expected.stderr.is_empty());
    assert!(expected.stdout.starts_with(b"rustc 1.95.0 "));
    let env = session.environment(base).unwrap();
    let executable = &env
        .iter()
        .find(|(key, _)| key == "RUSTC_WRAPPER")
        .unwrap()
        .1;
    let result = Command::new(executable)
        .args(["rustc", "-vV"])
        .env_clear()
        .envs(env)
        .output()
        .unwrap();
    assert!(result.status.success() && result.stderr.is_empty());
    assert_eq!(result.stdout, expected.stdout);
    assert!(session.invocations().unwrap().is_empty());
}

fn main() {
    use std::path::PathBuf;
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 3);
    let out = PathBuf::from(&args[0]);
    let home = PathBuf::from(&args[1]);
    let corrupt = args[2] == "corrupt";
    let compiler = controller::Compiler::prepare(&out, "1.95.0", &home).unwrap();
    let base = catalog_build::environment("1.95.0", &home, &out.join("target")).unwrap();
    let cwd = std::env::current_dir().unwrap();
    match compiler.session("readback", &base, &cwd) {
        Err(reason) => println!("refused:{reason}"),
        Ok(session) => {
            println!("accepted");
            if corrupt {
                return;
            }
            query(&session, &base);
            println!("query-ok");
            return;
        }
    }
    assert!(corrupt);
    let recovered = compiler.session("recovery", &base, &cwd).unwrap();
    query(&recovered, &base);
    println!("recovery-ok");
}
