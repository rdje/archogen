//! What a build records of itself (`docs/decisions/decision_hosted-build.md` §1): the read `archogen build` makes before
//! it replaces an earlier build, and `archogen explain` before it speaks for one.
//!
//! ⚠️ **What the digests are for.** Each listed file's, and the provenance's own, catch a file edited since; [`Built::foreign`]
//! catches what no build wrote. They never catch a forger: whoever can write the directory can rewrite `provenance.json` to
//! match, as whoever can write an install can rewrite its manifest (`crates/archogen-install`). A build is trusted as far as
//! its directory is.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::{Component, Path};

use archogen_digest::Digest;
use archogen_emit::{
    provenance_is_whole, ASSIGNMENT_FILE, PROVENANCE_FILE, PROVENANCE_FORMAT, RESULT_FILE,
};
use archogen_json::Value;

/// The most of a provenance the command reads.
const LIMIT: u64 = 1 << 24;

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

/// A build's provenance, as far as a later command reads it.
#[derive(Debug)]
pub struct Built {
    /// The system it was built for.
    pub system: String,
    /// The description's name under the build's `description/`.
    pub description: String,
    /// The digest of the configuration checker that accepted its plan.
    pub checker: Digest,
    /// Every file it wrote but the provenance itself, with its digest.
    pub files: Vec<(String, Digest)>,
    /// Whether the provenance is what its own digest says.
    whole: bool,
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
    pub fn read(dir: &Path) -> Result<Self, String> {
        let shown = dir.display();
        // A regular file, read within a bound: a link is no file a build wrote, and a pipe would never end.
        let file = dir.join(PROVENANCE_FILE);
        let meta = std::fs::symlink_metadata(&file)
            .map_err(|error| format!("{shown}'s `{PROVENANCE_FILE}`: {error}"))?;
        if !meta.is_file() {
            return Err(format!(
                "{shown}'s `{PROVENANCE_FILE}` is no file a build wrote"
            ));
        }
        let mut bytes = Vec::new();
        std::fs::File::open(&file)
            .and_then(|opened| opened.take(LIMIT + 1).read_to_end(&mut bytes))
            .map_err(|error| format!("{shown}'s `{PROVENANCE_FILE}`: {error}"))?;
        if bytes.len() as u64 > LIMIT {
            return Err(format!(
                "{shown}'s `{PROVENANCE_FILE}` is larger than any build's"
            ));
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
            return Err(format!(
                "{shown} is a build of `{format}`, and this engine reads `{PROVENANCE_FORMAT}`"
            ));
        }
        let description = text("description")?;
        if !inside(&description) {
            return Err(format!(
                "{shown}/{PROVENANCE_FILE} names the description `{description}`, which lies outside the build"
            ));
        }
        let Some(Value::Array(listed)) = provenance.get("files") else {
            return Err(format!("{shown}/{PROVENANCE_FILE} lists no `files`"));
        };
        let mut files = Vec::with_capacity(listed.len());
        for entry in listed {
            let path = entry.get("path").and_then(Value::as_str).unwrap_or("");
            let sha256 = entry.get("sha256").and_then(Value::as_str).unwrap_or("");
            if !inside(path) {
                return Err(format!(
                    "{shown}/{PROVENANCE_FILE} lists the file `{path}`, which lies outside the build"
                ));
            }
            files.push((path.to_owned(), digest(path, sha256)?));
        }
        if let Some(missing) = ALWAYS
            .iter()
            .find(|name| !files.iter().any(|(listed, _)| listed == *name))
        {
            return Err(format!(
                "{shown}/{PROVENANCE_FILE} does not list `{missing}`, which every build writes"
            ));
        }
        Ok(Self {
            system: text("system")?,
            checker: digest("checker", &text("checker")?)?,
            description,
            files,
            whole,
        })
    }

    /// The first file this build wrote that is not as it wrote it.
    pub fn edited(&self, dir: &Path) -> Option<String> {
        if !self.whole {
            return Some(format!(
                "`{PROVENANCE_FILE}` has changed since the build wrote it"
            ));
        }
        self.files.iter().find_map(|(name, digest)| {
            let file = dir.join(name);
            match std::fs::symlink_metadata(&file) {
                Ok(meta) if meta.is_file() => {}
                Ok(_) => return Some(format!("`{name}` is no file the build wrote")),
                Err(error) => return Some(format!("`{name}`: {error}")),
            }
            match std::fs::read(&file) {
                Ok(bytes) if Digest::of(&bytes) == *digest => None,
                Ok(_) => Some(format!("`{name}` has changed since the build wrote it")),
                Err(error) => Some(format!("`{name}`: {error}")),
            }
        })
    }

    /// Every directory that holds a file this build wrote, deepest first.
    fn holding(&self) -> Vec<String> {
        let mut held = BTreeSet::new();
        for (name, _) in &self.files {
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
    pub fn foreign(&self, dir: &Path) -> Option<String> {
        let listed: BTreeSet<&str> = self.files.iter().map(|(name, _)| name.as_str()).collect();
        let holding = self.holding();
        let mut pending = vec![dir.to_path_buf()];
        while let Some(next) = pending.pop() {
            let entries = match std::fs::read_dir(&next) {
                Ok(entries) => entries,
                Err(error) => return Some(format!("{}: {error}", next.display())),
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let relative = path.strip_prefix(dir).unwrap_or(&path);
                let Some(name) = relative.to_str() else {
                    return Some(format!("`{}` is no file a build wrote", relative.display()));
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
                    return Some(format!("`{name}` is no file a build wrote"));
                }
            }
        }
        None
    }

    /// Everything this build wrote taken out of `dir`, each file only if it is still as it was written, then each directory
    /// that held them and is empty now. Nothing else goes: what stays is for its owner to look at.
    pub fn remove(&self, dir: &Path) {
        for (name, digest) in &self.files {
            let file = dir.join(name);
            if std::fs::read(&file).is_ok_and(|bytes| Digest::of(&bytes) == *digest) {
                let _ = std::fs::remove_file(&file);
            }
        }
        let provenance = dir.join(PROVENANCE_FILE);
        if std::fs::read_to_string(&provenance).is_ok_and(|text| provenance_is_whole(&text)) {
            let _ = std::fs::remove_file(&provenance);
        }
        for name in self.holding() {
            let _ = std::fs::remove_dir(dir.join(name));
        }
    }
}
