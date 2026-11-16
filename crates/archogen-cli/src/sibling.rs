//! The front's launcher (leaf `M3.5.1.4.1`; `M3.5.1`'s design, *Finding and trusting the sibling*): the one file of the
//! product that starts a process. The `archogen` executable's `main.rs` declares it and hands it to
//! `run_with_input`; `NO-SUBPROCESS` admits this file by its path and refuses `mod sibling` anywhere else, so the
//! library, the MCP server and the browser module cannot reach it.
//!
//! ⭐ **It runs only what the install vouches for.** The install directory is the one the canonical `current_exe()`
//! lies in. Its manifest, `archogen-install/1`, must name that directory and this front's own sha256, so a front and
//! its siblings are one build; the sibling must be one the manifest names, and the bytes read from the file opened to
//! run it must be the sha256 the manifest gives. Then it runs, its environment cleared, its standard input empty, its
//! output relayed.
//!
//! ⚠️ **What that guards, and what it does not** (the review's D2). Whoever can write the install directory can replace
//! the front and the manifest together, so the digests catch a stale or mismatched sibling — an older build left beside
//! a newer front, a program copied in by hand — never tampering. On Linux the sibling runs from the file it was hashed
//! from, through the descriptor this process holds until the sibling ends, named under this process's `/proc` entry;
//! elsewhere it runs by its path, and a file replaced between the hash and the start runs unchecked: a race stated,
//! not closed.

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_digest::Digest;
use archogen_install::{Manifest, FILE};

/// The most of a manifest the front reads: one names the front and a handful of siblings.
const MANIFEST_LIMIT: usize = 1 << 16;

/// A place as a refusal says it: where an install stands is anyone's name, and this says it within a line of its own words.
fn place(path: &Path) -> String {
    eadl_front::fit_name(&path.display().to_string())
}

/// The launcher of an installed front: it runs the siblings its install's manifest names.
#[derive(Debug, Clone, Copy, Default)]
pub struct Installed;

impl Launch for Installed {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        let (dir, front) = install()?;
        let manifest = manifest(&dir, program)?;
        if manifest.directory() != dir.to_string_lossy() {
            return Err(refused(
                format!(
                    "the install manifest in {} names {} as its directory, so it describes another install",
                    place(&dir),
                    eadl_front::fit_name(manifest.directory())
                ),
                "install the toolchain again, into one directory, and run the `archogen` it places there",
            ));
        }
        let mine = std::fs::read(&front).map_err(|error| {
            refused(
                format!(
                    "cannot read the running `archogen`, {}, to check it: {error}",
                    place(&front)
                ),
                "run the `archogen` an install placed",
            )
        })?;
        if Digest::of(&mine) != manifest.front().sha256 {
            return Err(refused(
                format!(
                    "this `archogen` is not the one the install in {} placed, so its siblings are another build's",
                    place(&dir)
                ),
                "install the toolchain again, so the front and its siblings come from one build",
            ));
        }
        let Some(entry) = manifest.sibling(program) else {
            return Err(refused(
                format!("the install in {} has no `{program}`", place(&dir)),
                "install the toolchain again from a commit whose front runs it",
            ));
        };
        let file = dir.join(&entry.name);
        let mut opened = File::open(&file).map_err(|error| {
            refused(
                format!("cannot open `{program}` in {}: {error}", place(&dir)),
                "install the toolchain again",
            )
        })?;
        let mut bytes = Vec::new();
        opened.read_to_end(&mut bytes).map_err(|error| {
            refused(
                format!("cannot read `{program}` in {}: {error}", place(&dir)),
                "install the toolchain again",
            )
        })?;
        if Digest::of(&bytes) != entry.sha256 {
            return Err(refused(
                format!(
                    "`{program}` in {} is not the one the install placed: its sha256 is not the manifest's",
                    place(&dir)
                ),
                "install the toolchain again, so the front and its siblings come from one build",
            ));
        }
        drop(bytes);
        let mut command = Command::new(runnable(&opened, &file));
        command.args(args).env_clear().stdin(Stdio::null());
        // Nothing else of this environment reaches it: this is the instance's, and the same for what it starts.
        if let Some(ceiling) = std::env::var_os(archogen_transport::ceiling::VARIABLE) {
            command.env(archogen_transport::ceiling::VARIABLE, ceiling);
        }
        let output = command
            .output()
            .map_err(|error| {
                refused(
                    format!("cannot start `{program}` in {}: {error}", place(&dir)),
                    "check that the install's programs are executable, and that this user has room for another process and its files; or install the toolchain again",
                )
            })?;
        drop(opened);
        // Each goes on whatever became of the other, and neither decides anything: the program has judged, a lost report is
        // noted where standard output is watched, and a standard error nobody reads loses only what it would have said.
        let _ = out.write_all(&output.stdout);
        let _ = err.write_all(&output.stderr);
        Ok(Launched {
            ended: match output.status.code() {
                Some(code) => Ended::Code(code),
                None => Ended::Abnormally(output.status.to_string()),
            },
            sha256: entry.sha256,
            front: manifest.front().sha256,
        })
    }
}

fn refused(message: String, repair: &str) -> Unlaunched {
    Unlaunched {
        message,
        repair: repair.to_owned(),
    }
}

/// The install directory and the running front, both canonical.
fn install() -> Result<(PathBuf, PathBuf), Unlaunched> {
    let front = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .map_err(|error| {
            refused(
                format!("cannot find the running `archogen`: {error}"),
                "run `archogen` from the directory an install placed it in",
            )
        })?;
    let dir = front.parent().map(Path::to_path_buf).ok_or_else(|| {
        refused(
            format!("{} lies in no directory", place(&front)),
            "run an installed `archogen`",
        )
    })?;
    Ok((dir, front))
}

/// The manifest an install wrote in `dir`, read whole under its bound.
fn manifest(dir: &Path, program: &str) -> Result<Manifest, Unlaunched> {
    let file = dir.join(FILE);
    let mut bytes = Vec::new();
    let read = File::open(&file).and_then(|opened| {
        opened
            .take(u64::try_from(MANIFEST_LIMIT).unwrap_or(u64::MAX) + 1)
            .read_to_end(&mut bytes)
    });
    match read {
        Ok(_) => Manifest::read(&bytes, MANIFEST_LIMIT).map_err(|refusal| {
            refused(
                format!(
                    "the install manifest {}: {}",
                    place(&file),
                    refusal.to_string().replace('\n', "\\u{a}")
                ),
                "install the toolchain again",
            )
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(refused(
            format!(
                "`{program}` runs from an install, and {} has no install manifest, {FILE}",
                place(dir)
            ),
            "place the toolchain with `cargo xtask install --to DIR`, then run `DIR/archogen`",
        )),
        Err(error) => Err(refused(
            format!("cannot read the install manifest {}: {error}", place(&file)),
            "install the toolchain again",
        )),
    }
}

/// What to start: on Linux the very file that was hashed, through the descriptor this process holds open until the
/// sibling ends. It is named under this process's own `/proc` entry, not `/proc/self`: the child's copy of the
/// descriptor closes as the child starts its program, so an interpreter the kernel hands a script's name to would find
/// nothing at `/proc/self/fd`, where this process's descriptor still stands (leaf `PROGRAM.151.1`).
#[cfg(target_os = "linux")]
fn runnable(opened: &File, _file: &Path) -> PathBuf {
    use std::os::fd::AsRawFd;
    PathBuf::from(format!(
        "/proc/{}/fd/{}",
        std::process::id(),
        opened.as_raw_fd()
    ))
}

/// What to start: elsewhere the file by its name, which may change between the hash and the start.
#[cfg(not(target_os = "linux"))]
fn runnable(_opened: &File, file: &Path) -> PathBuf {
    file.to_path_buf()
}
