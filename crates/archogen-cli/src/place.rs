//! What an `--out` names, read one way by both commands that write one (`docs/decisions/decision_hosted-build.md` §5). What each
//! route does is a row of a table a test runs: `docs/book/src/building.md`'s for `build`, `docs/book/src/cli.md`'s for `resolve`.
//! So this file says how to read a place, and no sentence here says what happens to one.

use std::cell::{Cell, RefCell};
use std::fs::Metadata;
use std::io::{ErrorKind, Write};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use archogen_digest::Digest;

/// What stands on the way to `name` under `dir`.
pub enum Way {
    /// Real directories, all the way to the name's own parent.
    Real,
    /// A file, a link or nothing, where a directory was: what stands there is not what was left.
    Changed(String),
    /// A directory closed to this user, or a path past what can be named: nothing says what lies beyond.
    Unread(String),
}

/// How `name` is reached from `dir`. A link is never followed: one on the way is a change, whatever it points at, so nothing about what it points at can make that unanswerable.
#[must_use]
pub fn way_to(dir: &Path, name: &str) -> Way {
    let mut at = dir.to_path_buf();
    for part in Path::new(name)
        .parent()
        .into_iter()
        .flat_map(Path::components)
    {
        at.push(part);
        match std::fs::symlink_metadata(&at) {
            Ok(meta) if meta.is_dir() => {}
            Ok(_) => return Way::Changed(format!("`{name}`: {} is no directory", at.display())),
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return Way::Changed(format!("`{name}`: {error}"))
            }
            Err(error) => return Way::Unread(format!("`{name}`: {error}")),
        }
    }
    Way::Real
}

/// A file a run wrote in its stage, known from the moment it stands, before a byte is in it.
struct WrittenFile {
    name: String,
    /// What tells it from a file that comes to stand at its name, which no contents can: another of the same length, or closed, is another.
    identity: (u64, u64),
    /// Its length, when it was last written and its digest, once it is written whole. Without them it is what a failed write left, a part of what it would have held.
    whole: Option<(u64, Option<SystemTime>, Digest)>,
}

/// What a run has made in a stage of its own: each directory and each file, from the moment it stands, so that one it could not finish is its own too. On a refusal that much goes,
/// a file only if it is still the one it wrote, and what came stays with the stage, which is said. This is the rule for every route of both commands: a run removes what it wrote, and
/// nothing else. It is one way to write in a stage, so no file written there can miss it.
#[derive(Default)]
pub struct Written {
    directories: RefCell<Vec<PathBuf>>,
    files: RefCell<Vec<WrittenFile>>,
    /// Whether the checker was handed the stage, nothing then standing at its record's name: only then is what stands there its.
    pub checker_ran: Cell<bool>,
}

/// Why a file could not be written in a stage.
pub enum NotWritten {
    /// Something else stands where it, or a directory on the way to it, goes.
    Came(PathBuf),
    /// What could not be done, to what, and why.
    Failed(&'static str, PathBuf, std::io::Error),
}

impl Written {
    /// `contents` written at `name` in `stage`, each directory on the way made if absent and otherwise a real one, and the file new: `fs::write` would write through a link, and over what
    /// something else put there. A file there already that holds exactly this is not written again, and is not this call's: the checker's record among them.
    ///
    /// # Errors
    ///
    /// What stands in the way, or what the host refused.
    pub fn write(&self, stage: &Path, name: &str, contents: &[u8]) -> Result<(), NotWritten> {
        let mut at = stage.to_path_buf();
        for part in Path::new(name)
            .parent()
            .into_iter()
            .flat_map(Path::components)
        {
            at.push(part);
            match std::fs::symlink_metadata(&at) {
                Ok(meta) if meta.is_dir() => {}
                Ok(_) => return Err(NotWritten::Came(at)),
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    match std::fs::create_dir(&at) {
                        Ok(()) => self.directories.borrow_mut().push(at.clone()),
                        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                            return Err(NotWritten::Came(at))
                        }
                        Err(error) => {
                            return Err(NotWritten::Failed("make the directory", at, error))
                        }
                    }
                }
                Err(error) => return Err(NotWritten::Failed("make the directory", at, error)),
            }
        }
        let file = stage.join(name);
        let mut opened = match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file)
        {
            Ok(opened) => opened,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                let same = std::fs::symlink_metadata(&file).is_ok_and(|meta| meta.is_file())
                    && std::fs::read(&file).is_ok_and(|bytes| bytes == contents);
                return if same {
                    Ok(())
                } else {
                    Err(NotWritten::Came(file))
                };
            }
            Err(error) => return Err(NotWritten::Failed("write", file, error)),
        };
        let identity = match opened.metadata() {
            Ok(meta) => identity_of(&meta),
            Err(error) => return Err(NotWritten::Failed("write", file, error)),
        };
        let at = self.files.borrow().len();
        self.files.borrow_mut().push(WrittenFile {
            name: name.to_owned(),
            identity,
            whole: None,
        });
        if let Err(error) = opened.write_all(contents) {
            return Err(NotWritten::Failed("write", file, error));
        }
        if let Ok(meta) = opened.metadata() {
            self.files.borrow_mut()[at].whole =
                Some((meta.len(), meta.modified().ok(), Digest::of(contents)));
        }
        Ok(())
    }

    /// Each file this run wrote out of `stage`, still the file it wrote and reached through real directories; then each directory it made, and the stage, where that leaves them empty.
    pub fn remove_from(&self, stage: &Path) {
        for file in self.files.borrow().iter() {
            if !matches!(way_to(stage, &file.name), Way::Real) {
                continue;
            }
            let path = stage.join(&file.name);
            let Ok(meta) = std::fs::symlink_metadata(&path) else {
                continue;
            };
            let its = meta.is_file()
                && identity_of(&meta) == file.identity
                && file.whole.as_ref().is_none_or(|(length, when, digest)| {
                    meta.len() == *length
                        && meta.modified().ok() == *when
                        // One the umask closed to its owner, as this run wrote it, cannot be read to tell, and it is the same file, last written then.
                        && match std::fs::read(&path) {
                            Ok(bytes) => Digest::of(&bytes) == *digest,
                            Err(error) => error.kind() == ErrorKind::PermissionDenied,
                        }
                });
            if its {
                let _ = std::fs::remove_file(&path);
            }
        }
        // Made outermost first, so gone innermost first.
        for directory in self.directories.borrow().iter().rev() {
            let _ = std::fs::remove_dir(directory);
        }
        let _ = std::fs::remove_dir(stage);
    }
}

/// What tells one file from another at the same name.
#[cfg(unix)]
fn identity_of(meta: &Metadata) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    (meta.dev(), meta.ino())
}

/// What tells one file from another at the same name: here, nothing `std` reads, so its length and its time do alone.
#[cfg(not(unix))]
fn identity_of(_meta: &Metadata) -> (u64, u64) {
    (0, 0)
}

/// `path`, opened to read if it is a regular file: never through a link, and never a pipe, which nobody may ever end. A file no longer one is a refusal of its own kind.
///
/// # Errors
///
/// What stopped it, `InvalidInput` for what is no regular file.
pub fn open_regular(path: &Path) -> std::io::Result<std::fs::File> {
    if !std::fs::symlink_metadata(path)?.is_file() {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "it is no regular file",
        ));
    }
    std::fs::File::open(path)
}

/// What a refused run says of a stage that may stand: that the next run refuses while it does, and what to do. It always says so, so that no stage is found with nothing having been said of it. Only a name
/// found absent is absent: one that cannot be looked at may stand.
pub fn say_it_stays(stage: &Path, err: &mut dyn Write) {
    match std::fs::symlink_metadata(stage) {
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => {
            let _ = writeln!(
                err,
                "  {} cannot be looked at ({error}), so it may stand, and the next run refuses while it does: open what closes it, look at it, and remove it",
                stage.display()
            );
        }
        Ok(_) => {
            let _ = writeln!(
                err,
                "  {} could not be removed, or holds what this run did not write, what its checker left among it, and the next run refuses while it stands: look at it, and remove it",
                stage.display()
            );
        }
    }
}

/// What to say of `stage` where a refusal has tried to remove it: that it stands, if it does.
pub fn if_it_stays(stage: &Path) -> String {
    if std::fs::symlink_metadata(stage).is_err_and(|error| {
        matches!(
            error.kind(),
            ErrorKind::NotFound | ErrorKind::InvalidFilename
        )
    }) {
        String::new()
    } else {
        format!(
            "; and {} could not be removed, and the next run refuses while it stands: look at it, and remove it",
            stage.display()
        )
    }
}

/// A stage this run made and could not close, gone if it is empty; and what to say if it could not go. A name too long for the place names nothing that was made.
pub fn remove_empty_stage(stage: &Path) -> String {
    match std::fs::remove_dir(stage) {
        Ok(()) => String::new(),
        Err(error) if matches!(error.kind(), ErrorKind::NotFound | ErrorKind::InvalidFilename) => {
            String::new()
        }
        Err(error) => format!(
            "; and it could not be removed ({error}), and the next run refuses while it stands: look at it, and remove it"
        ),
    }
}

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
        ErrorKind::PermissionDenied => "name a place this user can reach and write, with a umask that lets it write what it makes and no flag or access list against it",
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
