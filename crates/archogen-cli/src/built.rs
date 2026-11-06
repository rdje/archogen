//! What a build records of itself (`docs/decisions/decision_hosted-build.md` §1): the read `archogen build` makes before
//! it replaces an earlier build, and `archogen explain` before it speaks for one.
//!
//! ⚠️ **What the digests are for.** Each listed file's, and the provenance's own, catch a file edited since; [`Built::foreign`]
//! catches what no build wrote. They never catch a forger: whoever can write the directory can rewrite `provenance.json` to
//! match, as whoever can write an install can rewrite its manifest (`crates/archogen-install`). A build is trusted as far as
//! its directory is.

use std::collections::BTreeSet;
use std::io::{ErrorKind, Read};
use std::path::{Component, Path};

use archogen_digest::{Digest, Sha256};
use archogen_emit::{
    provenance_is_whole, ASSIGNMENT_FILE, PROVENANCE_FILE, PROVENANCE_FORMAT, RESULT_FILE,
};
use archogen_json::Value;

use crate::place::{self, Way};

/// The most of a provenance the command reads, so that what is not one cannot take all there is.
const LIMIT: u64 = 1 << 30;

/// `file`'s digest, read as it goes, so that a file as long as a build wrote it is held to what it holds at no cost in memory.
fn digest_of(file: &Path) -> std::io::Result<Digest> {
    let mut opened = std::fs::File::open(file)?;
    let mut hasher = Sha256::default();
    let mut chunk = vec![0; 1 << 16];
    loop {
        match opened.read(&mut chunk)? {
            0 => return Ok(hasher.finish()),
            count => hasher.update(&chunk[..count]),
        }
    }
}

/// What building a package leaves at its top, cargo's or the desktop's, and no build wrote: a build neither judges them nor
/// deletes them, and carries them to the package that replaces it.
pub const CARRIED: [&str; 3] = ["target", "Cargo.lock", ".DS_Store"];

/// What every build holds, so a list without them is no build's.
const ALWAYS: [&str; 5] = [
    "Cargo.toml",
    "src/main.rs",
    "plan.txt",
    ASSIGNMENT_FILE,
    RESULT_FILE,
];

/// A file a build wrote, as its provenance lists it.
#[derive(Debug)]
pub struct Listed {
    /// Where it stands, under the build.
    pub name: String,
    /// How long it is.
    pub bytes: u64,
    /// What it holds.
    pub digest: Digest,
}

impl Listed {
    /// Whether the file at `file` is this one: another length is another file, and is not read to find that out, whatever it has
    /// grown to.
    fn holds(&self, file: &Path, meta: &std::fs::Metadata) -> std::io::Result<bool> {
        Ok(meta.len() == self.bytes && digest_of(file)? == self.digest)
    }
}

/// A build's provenance, as far as a later command reads it.
#[derive(Debug)]
pub struct Built {
    /// The system it was built for.
    pub system: String,
    /// The description's name under the build's `description/`.
    pub description: String,
    /// The digest of the configuration checker that accepted its plan.
    pub checker: Digest,
    /// Every file it wrote but the provenance itself.
    pub files: Vec<Listed>,
    /// Whether the provenance is what its own digest says.
    whole: bool,
}

/// A link loop's error, which the standard library has no stable kind for.
#[cfg(target_os = "linux")]
const ELOOP: i32 = 40;
#[cfg(not(target_os = "linux"))]
const ELOOP: i32 = 62;

/// Whether this error says what was asked for is not there as the build left it: gone, a name on the way to it now no directory, or a loop of links. Each is a change, and
/// no opening of anything would answer it.
fn says_it_changed(error: &std::io::Error) -> bool {
    matches!(error.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory)
        || error.raw_os_error() == Some(ELOOP)
}

/// Why a directory records nothing this engine reads.
#[derive(Debug)]
pub enum Unbuilt {
    /// It is a build of a format this engine does not read: another engine's, which nothing here can judge, so only a new directory or
    /// taking that build away helps.
    OtherFormat(String),
    /// Its provenance is closed to this user, or the directory holding it: nothing says what it is until it is opened.
    Closed(String),
    /// Anything else, what stands there being no build, or one that does not read.
    Other(String),
}

impl Unbuilt {
    /// What it says.
    #[must_use]
    pub fn said(self) -> String {
        match self {
            Self::OtherFormat(said) | Self::Closed(said) | Self::Other(said) => said,
        }
    }
}

impl From<String> for Unbuilt {
    fn from(said: String) -> Self {
        Self::Other(said)
    }
}

/// Why a build is not known to be as it was written.
#[derive(Debug)]
pub enum NotAsWritten {
    /// It is not: a file changed or gone, or something there no build wrote.
    Changed(String),
    /// What it holds could not be read, so nothing is said of whether it changed: an unanswerable question is not answered as a change.
    Unread(String),
}

impl NotAsWritten {
    /// What it says.
    #[must_use]
    pub fn said(self) -> String {
        match self {
            Self::Changed(said) | Self::Unread(said) => said,
        }
    }
}

/// A name that stays inside the build: a provenance is text anyone may have edited.
fn inside(name: &str) -> bool {
    !name.is_empty()
        && Path::new(name)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

impl Built {
    /// What `dir` records, or why it records nothing this engine reads.
    pub fn read(dir: &Path) -> Result<Self, Unbuilt> {
        let shown = dir.display();
        // A regular file, read within a bound: a link is no file a build wrote, and a pipe would never end.
        let file = dir.join(PROVENANCE_FILE);
        let unread = |error: std::io::Error| {
            let said = format!("{shown}'s `{PROVENANCE_FILE}`: {error}");
            // A build is no less one for being closed, or named by a path past what can be named.
            if matches!(
                error.kind(),
                ErrorKind::PermissionDenied | ErrorKind::InvalidFilename
            ) {
                Unbuilt::Closed(said)
            } else {
                Unbuilt::Other(said)
            }
        };
        let meta = std::fs::symlink_metadata(&file).map_err(unread)?;
        if !meta.is_file() {
            return Err(format!("{shown}'s `{PROVENANCE_FILE}` is no file a build wrote").into());
        }
        // Past the bound by its length, unread: a file of that size costs the shared host what it holds, sparse or not. The bound is held while reading too, for one that grows.
        let past = || {
            format!(
                "{shown}'s `{PROVENANCE_FILE}` is larger than this engine reads one, 2^30 bytes"
            )
        };
        if meta.len() > LIMIT {
            return Err(past().into());
        }
        let mut bytes = Vec::new();
        std::fs::File::open(&file)
            .and_then(|opened| opened.take(LIMIT + 1).read_to_end(&mut bytes))
            .map_err(unread)?;
        if bytes.len() as u64 > LIMIT {
            return Err(past().into());
        }
        let provenance = archogen_json::read(&bytes, bytes.len().max(1))
            .map_err(|refusal| format!("{shown}/{PROVENANCE_FILE} does not read: {refusal}"))?;
        let whole = std::str::from_utf8(&bytes).is_ok_and(provenance_is_whole);
        let text = |name: &str| {
            provenance
                .get(name)
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| format!("{shown}/{PROVENANCE_FILE} names no `{name}`"))
        };
        let digest = |name: &str, value: &str| {
            Digest::parse(value)
                .ok_or_else(|| format!("{shown}/{PROVENANCE_FILE}: `{name}` is no digest"))
        };
        let format = text("format")?;
        if format != PROVENANCE_FORMAT {
            return Err(Unbuilt::OtherFormat(format!(
                "{shown} is a build of `{format}`, and this engine reads `{PROVENANCE_FORMAT}`"
            )));
        }
        let description = text("description")?;
        if !inside(&description) {
            return Err(format!(
                "{shown}/{PROVENANCE_FILE} names the description `{description}`, which lies outside the build"
            )
            .into());
        }
        let Some(Value::Array(listed)) = provenance.get("files") else {
            return Err(format!("{shown}/{PROVENANCE_FILE} lists no `files`").into());
        };
        let mut files = Vec::with_capacity(listed.len());
        for entry in listed {
            let path = entry.get("path").and_then(Value::as_str).unwrap_or("");
            let sha256 = entry.get("sha256").and_then(Value::as_str).unwrap_or("");
            if !inside(path) {
                return Err(format!(
                    "{shown}/{PROVENANCE_FILE} lists the file `{path}`, which lies outside the build"
                )
                .into());
            }
            let bytes = match entry.get("bytes") {
                Some(Value::Number(number)) => {
                    number.as_i64().and_then(|bytes| u64::try_from(bytes).ok())
                }
                _ => None,
            }
            .ok_or_else(|| format!("{shown}/{PROVENANCE_FILE} lists `{path}` with no length"))?;
            files.push(Listed {
                name: path.to_owned(),
                bytes,
                digest: digest(path, sha256)?,
            });
        }
        if let Some(missing) = ALWAYS
            .iter()
            .find(|name| !files.iter().any(|listed: &Listed| listed.name == **name))
        {
            return Err(format!(
                "{shown}/{PROVENANCE_FILE} does not list `{missing}`, which every build writes"
            )
            .into());
        }
        Ok(Self {
            system: text("system")?,
            checker: digest("checker", &text("checker")?)?,
            description,
            files,
            whole,
        })
    }

    /// The first file this build wrote that is not as it wrote it, or that could not be read to tell.
    pub fn edited(&self, dir: &Path) -> Option<NotAsWritten> {
        if !self.whole {
            return Some(NotAsWritten::Changed(format!(
                "`{PROVENANCE_FILE}` has changed since the build wrote it"
            )));
        }
        self.files.iter().find_map(|listed| {
            let name = &listed.name;
            let file = dir.join(name);
            match place::way_to(dir, name) {
                Way::Real => {}
                Way::Changed(said) => return Some(NotAsWritten::Changed(said)),
                Way::Unread(said) => return Some(NotAsWritten::Unread(said)),
            }
            let meta = match std::fs::symlink_metadata(&file) {
                Ok(meta) if meta.is_file() => meta,
                Ok(_) => {
                    return Some(NotAsWritten::Changed(format!(
                        "`{name}` is no file the build wrote"
                    )))
                }
                Err(error) if says_it_changed(&error) => {
                    return Some(NotAsWritten::Changed(format!("`{name}`: {error}")))
                }
                Err(error) => return Some(NotAsWritten::Unread(format!("`{name}`: {error}"))),
            };
            match listed.holds(&file, &meta) {
                Ok(true) => None,
                Ok(false) => Some(NotAsWritten::Changed(format!(
                    "`{name}` has changed since the build wrote it"
                ))),
                Err(error) if says_it_changed(&error) => {
                    Some(NotAsWritten::Changed(format!("`{name}`: {error}")))
                }
                Err(error) => Some(NotAsWritten::Unread(format!("`{name}`: {error}"))),
            }
        })
    }

    /// Every directory that holds a file this build wrote, deepest first.
    fn holding(&self) -> Vec<String> {
        let mut held = BTreeSet::new();
        for Listed { name, .. } in &self.files {
            let mut at = Path::new(name).parent();
            while let Some(up) = at.filter(|up| !up.as_os_str().is_empty()) {
                held.insert(up.display().to_string());
                at = up.parent();
            }
        }
        let mut held: Vec<String> = held.into_iter().collect();
        held.sort_by_key(|name| std::cmp::Reverse(name.matches('/').count()));
        held
    }

    /// The first thing under `dir` no build wrote, empty directories and names that are no text among them: all there is
    /// beside what it lists, and what [`CARRIED`] names at its top.
    pub fn foreign(&self, dir: &Path) -> Option<NotAsWritten> {
        let listed: BTreeSet<&str> = self
            .files
            .iter()
            .map(|listed| listed.name.as_str())
            .collect();
        let holding = self.holding();
        let mut pending = vec![dir.to_path_buf()];
        while let Some(next) = pending.pop() {
            let entries = match std::fs::read_dir(&next) {
                Ok(entries) => entries,
                Err(error) => {
                    return Some(NotAsWritten::Unread(format!("{}: {error}", next.display())))
                }
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let relative = path.strip_prefix(dir).unwrap_or(&path);
                let Some(name) = relative.to_str() else {
                    return Some(NotAsWritten::Changed(format!(
                        "`{}` is no file a build wrote",
                        relative.display()
                    )));
                };
                if name == PROVENANCE_FILE || CARRIED.contains(&name) {
                    continue;
                }
                let kind = entry.file_type();
                let is_dir = kind.as_ref().is_ok_and(std::fs::FileType::is_dir);
                if is_dir && holding.iter().any(|held| held == name) {
                    pending.push(path);
                } else if is_dir
                    || !(kind.is_ok_and(|kind| kind.is_file()) && listed.contains(name))
                {
                    return Some(NotAsWritten::Changed(format!(
                        "`{name}` is no file a build wrote"
                    )));
                }
            }
        }
        None
    }

    /// Everything this build wrote taken out of `dir`, each file only if it is still the regular file it wrote, reached
    /// through real directories, then each directory that held them and is empty now. Nothing else goes: what stays is for
    /// its owner to look at.
    ///
    /// ⚠️ Each path is held just before it is used, which is as far as `std` goes: in the instant between, whatever another
    /// program puts at that path goes, whatever its bytes, and a directory it swaps for a link redirects the removal.
    pub fn remove(&self, dir: &Path) {
        // A directory on the way that is no longer a real one is not this build's.
        let reached = |name: &str| {
            let mut at = dir.to_path_buf();
            for part in Path::new(name)
                .parent()
                .into_iter()
                .flat_map(Path::components)
            {
                at.push(part);
                if !std::fs::symlink_metadata(&at).is_ok_and(|meta| meta.is_dir()) {
                    return false;
                }
            }
            true
        };
        let as_written = |listed: &Listed, file: &Path| {
            std::fs::symlink_metadata(file).is_ok_and(|meta| {
                meta.is_file() && listed.holds(file, &meta).is_ok_and(|same| same)
            })
        };
        for listed in &self.files {
            let file = dir.join(&listed.name);
            if reached(&listed.name) && as_written(listed, &file) {
                let _ = std::fs::remove_file(&file);
            }
        }
        let provenance = dir.join(PROVENANCE_FILE);
        if std::fs::symlink_metadata(&provenance).is_ok_and(|meta| meta.is_file())
            && std::fs::read_to_string(&provenance).is_ok_and(|text| provenance_is_whole(&text))
        {
            let _ = std::fs::remove_file(&provenance);
        }
        for name in self.holding() {
            let held = dir.join(&name);
            if std::fs::symlink_metadata(&held).is_ok_and(|meta| meta.is_dir()) {
                let _ = std::fs::remove_dir(held);
            }
        }
    }
}
