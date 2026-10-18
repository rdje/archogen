//! Standalone compiler observer, embedded and built by the trust inventory's controller.
//! Cargo's published RUSTC_WRAPPER interface supplies the compiler followed by its native argv.

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
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(DIGITS[(byte >> 4) as usize] as char);
        encoded.push(DIGITS[(byte & 15) as usize] as char);
    }
    encoded
}

fn observe() -> Result<i32, Box<dyn std::error::Error>> {
    let argv: Vec<_> = env::args_os().skip(1).collect();
    let compiler = argv.first().ok_or("missing compiler argument")?;
    let records = PathBuf::from(env::var_os("ARCHOGEN_COMPILER_RECORDS").ok_or("missing records")?);
    let session = env::var_os("ARCHOGEN_COMPILER_SESSION").ok_or("missing session")?;
    let role = env::var_os("ARCHOGEN_COMPILER_ROLE").ok_or("missing role")?;
    let cwd = env::current_dir()?;
    let pid = process::id();
    let partial = records.join(format!(".{pid}.part"));
    let completed = records.join(format!("{pid}.json"));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&partial)?;
    let status = Command::new(compiler)
        .args(&argv[1..])
        .env_remove("RUSTC_WRAPPER")
        .env_remove("ARCHOGEN_COMPILER_RECORDS")
        .env_remove("ARCHOGEN_COMPILER_SESSION")
        .env_remove("ARCHOGEN_COMPILER_ROLE")
        .status()?;
    let arguments = argv
        .iter()
        .map(|arg| format!("\"{}\"", hex(arg.as_bytes())))
        .collect::<Vec<_>>()
        .join(",");
    let exit = status
        .code()
        .map_or("null".to_owned(), |code| code.to_string());
    let signal = status
        .signal()
        .map_or("null".to_owned(), |value| value.to_string());
    // Alphabetical keys and canonical JSON let the consumer reject duplicate fields as well as malformed data.
    let record = format!(
        "{{\"argv_hex\":[{arguments}],\"cwd_hex\":\"{}\",\"exit\":{exit},\"pid\":{pid},\"role_hex\":\"{}\",\"schema\":\"archogen-compiler-provenance/1\",\"session_hex\":\"{}\",\"signal\":{signal}}}\n",
        hex(cwd.as_os_str().as_bytes()), hex(role.as_bytes()), hex(session.as_bytes()),
    );
    file.write_all(record.as_bytes())?;
    file.sync_all()?;
    drop(file);
    fs::hard_link(&partial, &completed)?;
    fs::remove_file(&partial)?;
    Ok(status
        .code()
        .unwrap_or_else(|| 128 + status.signal().unwrap_or(1)))
}

fn main() {
    match observe() {
        Ok(code) => process::exit(code),
        Err(error) => {
            eprintln!("archogen compiler provenance: {error}");
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_argument_bytes_have_literal_hex_encodings() {
        assert_eq!(super::hex(b""), "");
        assert_eq!(super::hex(b"a b'\n\r\\\xff"), "612062270a0d5cff");
        assert_eq!(super::hex("é".as_bytes()), "c3a9");
    }
}
