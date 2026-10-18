use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    os::unix::{ffi::OsStrExt, process::ExitStatusExt},
    path::PathBuf,
    process::{self, Command},
};

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        result.push(DIGITS[(byte >> 4) as usize] as char);
        result.push(DIGITS[(byte & 15) as usize] as char);
    }
    result
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let compiler = args.first().ok_or("no compiler argument")?;
    let directory =
        PathBuf::from(env::var_os("ARCHOGEN_PROTO_TRACE_DIR").ok_or("no trace directory")?);
    let session = env::var_os("ARCHOGEN_PROTO_SESSION").ok_or("no session")?;
    let role = env::var_os("ARCHOGEN_PROTO_ROLE").ok_or("no role")?;
    let pid = process::id();
    let temporary = directory.join(format!(".{pid}.part"));
    let destination = directory.join(format!("{pid}.json"));
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;

    let status = Command::new(compiler)
        .args(&args[1..])
        .env_remove("RUSTC_WRAPPER")
        .env_remove("ARCHOGEN_PROTO_TRACE_DIR")
        .env_remove("ARCHOGEN_PROTO_SESSION")
        .env_remove("ARCHOGEN_PROTO_ROLE")
        .status()?;
    let arguments = args
        .iter()
        .map(|arg| format!("\"{}\"", hex(arg.as_bytes())))
        .collect::<Vec<_>>()
        .join(",");
    let exit = status
        .code()
        .map_or("null".to_owned(), |value| value.to_string());
    let signal = status
        .signal()
        .map_or("null".to_owned(), |value| value.to_string());
    let record = format!(
        "{{\"schema\":\"archogen-compiler-provenance-prototype/1\",\"session_hex\":\"{}\",\"role_hex\":\"{}\",\"cwd_hex\":\"{}\",\"pid\":{pid},\"exit\":{exit},\"signal\":{signal},\"argv_hex\":[{arguments}]}}\n",
        hex(session.as_bytes()),
        hex(role.as_bytes()),
        hex(env::current_dir()?.as_os_str().as_bytes()),
    );
    output.write_all(record.as_bytes())?;
    output.sync_all()?;
    drop(output);
    // Publishing a completed record never replaces an earlier invocation.
    fs::hard_link(&temporary, &destination)?;
    fs::remove_file(&temporary)?;
    Ok(status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or(1)))
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(error) => {
            eprintln!("compiler provenance prototype: {error}");
            process::exit(2);
        }
    }
}
