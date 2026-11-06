//! What an `--out` names, read one way by both commands that write one (`docs/decisions/decision_hosted-build.md` §5). What each
//! route does is a row of a table a test runs: `docs/book/src/building.md`'s for `build`, `docs/book/src/cli.md`'s for `resolve`.
//! So this file says how to read a place, and no sentence here says what happens to one.

use std::fs::Metadata;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};

/// Why a place cannot be used: what, and its repair. Always `usage`, nothing having been written.
pub struct Refused {
    /// What stops it.
    pub message: String,
    /// What to do about it.
    pub repair: String,
}

impl Refused {
    fn new(message: String, repair: &str) -> Self {
        Self {
            message,
            repair: repair.to_owned(),
        }
    }
}

/// The repair for a name or a path too long. It cannot say which: the error is one, and a host's limits are not `std`'s to read.
const TOO_LONG: &str = "a name or the whole path is too long: shorten it, leaving room for what goes beside it (`.building` \
                        and `.replaced` for a package, `.part` and `.check` for a plan), or name a place nearer the root";

/// The repair for a place an `--out` cannot use, by what stopped it.
pub fn repair(error: &std::io::Error) -> &'static str {
    match error.kind() {
        ErrorKind::InvalidFilename => TOO_LONG,
        ErrorKind::NotADirectory => {
            "a name on the way to it is no directory: name a path whose parents are directories"
        }
        ErrorKind::PermissionDenied => "name a place this user can reach and write, with a umask that lets it write what it makes",
        ErrorKind::AlreadyExists => {
            "a name on the way to it is a link to nothing, or something took it meanwhile: look at it, or name another place"
        }
        _ => "name another place, nearer the root if its path is long",
    }
}

/// `out` as the path it names, without a trailing separator so that what goes beside it lies beside it; `noun` is what it must
/// name, and `repair` how to name one. `missing/../here` reads as absent until `missing` is made, and is `here` after, so a path
/// that goes back through a name is refused.
pub fn read(out: &str, noun: &str, repair: &str) -> Result<PathBuf, Refused> {
    let path = Path::new(out).components().as_path().to_path_buf();
    if path.file_name().is_none() {
        return Err(Refused::new(
            format!("`--out {out}` names no {noun} of its own"),
            repair,
        ));
    }
    let mut after_a_name = false;
    for part in path.components() {
        match part {
            Component::Normal(_) => after_a_name = true,
            Component::ParentDir if after_a_name => {
                return Err(Refused::new(
                    format!("`--out {out}` goes back through a name, and a name may not exist yet"),
                    "name it without `..` after a name",
                ))
            }
            _ => {}
        }
    }
    Ok(path)
}

/// `path` with `suffix` on its last name: what goes beside it.
pub fn beside(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(suffix);
    PathBuf::from(name)
}

/// That nothing stands at `other`, which `--out` needs beside it. `because` says why what stands there stays, and only a name
/// found absent is absent: anything else that stops it being read is the refusal.
pub fn free(other: &Path, out: &str, because: &str) -> Result<(), Refused> {
    match std::fs::symlink_metadata(other) {
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(Refused::new(
            format!(
                "cannot use {}, which `--out {out}` needs beside it: {error}",
                other.display()
            ),
            repair(&error),
        )),
        Ok(_) => Err(Refused::new(
            format!("{} exists, {because}", other.display()),
            "look at it, and move or remove it; or name another place",
        )),
    }
}

/// The directories that must be made to reach `path`, innermost first: only these are a run's to remove again.
pub fn missing_parents(path: &Path) -> Vec<PathBuf> {
    let mut missing = Vec::new();
    let mut parent = path.parent();
    while let Some(up) = parent.filter(|up| !up.as_os_str().is_empty()) {
        if !std::fs::symlink_metadata(up).is_err_and(|error| error.kind() == ErrorKind::NotFound) {
            break;
        }
        missing.push(up.to_path_buf());
        parent = up.parent();
    }
    missing
}

/// The directories `path` lies in made, whichever of them are missing.
pub fn make_parents(path: &Path) -> Result<(), Refused> {
    let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) else {
        return Ok(());
    };
    std::fs::create_dir_all(parent).map_err(|error| {
        Refused::new(
            format!(
                "cannot make {}, where `--out` lies: {error}",
                parent.display()
            ),
            repair(&error),
        )
    })
}

/// The directories `made` gone again, where they stayed empty.
pub fn unmake(made: &[PathBuf]) {
    for dir in made {
        let _ = std::fs::remove_dir(dir);
    }
}

/// That `deepest`, beneath `place`, has room in a path when named as the checker is handed it: `place`'s directory canonical
/// and its own name kept, as `crate::consume` names what it hands over. So its parents must stand.
pub fn room(place: &Path, deepest: &str, out: &str) -> Result<(), Refused> {
    let parent = place
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let unnamed = |error: std::io::Error| {
        Refused::new(
            format!("cannot name `--out {out}` absolutely: {error}"),
            repair(&error),
        )
    };
    let name = place.file_name().unwrap_or_default();
    let named = std::fs::canonicalize(parent)
        .map_err(unnamed)?
        .join(name)
        .join(deepest);
    match std::fs::symlink_metadata(&named) {
        Err(error) if error.kind() == ErrorKind::InvalidFilename => Err(Refused::new(
            format!(
                "`--out {out}` leaves no room in a path for what goes beneath it, `{deepest}`: {error}"
            ),
            TOO_LONG,
        )),
        _ => Ok(()),
    }
}

/// A directory made by a call that fails if anything stands there, private to its owner from that call on: nobody else reads or
/// fills a stage, whatever what stands beside it lets others do.
#[cfg(unix)]
pub fn make_private(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    std::fs::DirBuilder::new().mode(0o700).create(path)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
}

/// A directory made by a call that fails if anything stands there; a platform with no owner's bits has nothing to close.
#[cfg(not(unix))]
pub fn make_private(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

/// What tells the directory a run made from one that came to stand at its name: never a link.
#[cfg(unix)]
pub fn identity(path: &Path) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let meta = std::fs::symlink_metadata(path).ok()?;
    meta.is_dir().then(|| (meta.dev(), meta.ino()))
}

/// What tells the directory a run made from one that came to stand at its name: never a link.
#[cfg(not(unix))]
pub fn identity(path: &Path) -> Option<(u64, u64)> {
    std::fs::symlink_metadata(path)
        .ok()?
        .is_dir()
        .then_some((0, 0))
}

/// `path` given what it takes of `from`, what it replaces: its group first, since a change of group can clear a mode's bits, then
/// its mode, whichever way the first went. An access control list, an owner and a flag are not among it: `std` can read none.
#[cfg(unix)]
pub fn take_over(path: &Path, from: &Metadata) -> std::io::Result<()> {
    use std::os::unix::fs::MetadataExt;
    let group = if std::fs::symlink_metadata(path)?.gid() == from.gid() {
        Ok(())
    } else {
        std::os::unix::fs::chown(path, None, Some(from.gid()))
    };
    let mode = std::fs::set_permissions(path, from.permissions());
    group.and(mode)
}

/// `path` given the mode of `from`, what it replaces.
#[cfg(not(unix))]
pub fn take_over(path: &Path, from: &Metadata) -> std::io::Result<()> {
    std::fs::set_permissions(path, from.permissions())
}
