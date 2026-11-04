//! What a build records of itself (`docs/decisions/decision_hosted-build.md` §1): the read `archogen build` makes before
//! it replaces an earlier build, and `archogen explain` before it speaks for one.
//!
//! ⚠️ **What the digests are for.** They catch a file edited since, and a file no build wrote. They never catch a forger:
//! whoever can write the directory can rewrite `provenance.json` to match, as whoever can write an install can rewrite
//! its manifest (`crates/archogen-install`). A build is trusted as far as its directory is.

use std::collections::BTreeSet;
use std::path::{Component, Path};

use archogen_digest::Digest;
use archogen_emit::{PROVENANCE_FILE, PROVENANCE_FORMAT};
use archogen_json::Value;

/// The most of a provenance the command reads.
const LIMIT: usize = 1 << 24;

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
        let bytes = std::fs::read(dir.join(PROVENANCE_FILE))
            .map_err(|error| format!("{shown}'s `{PROVENANCE_FILE}`: {error}"))?;
        let provenance = archogen_json::read(&bytes, LIMIT)
            .map_err(|refusal| format!("{shown}/{PROVENANCE_FILE} does not read: {refusal}"))?;
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
        Ok(Self {
            system: text("system")?,
            checker: digest("checker", &text("checker")?)?,
            description,
            files,
        })
    }

    /// The first file this build wrote that is not as it wrote it.
    pub fn edited(&self, dir: &Path) -> Option<String> {
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

    /// The first thing under `dir` no build wrote. Cargo's own output, `target/` and `Cargo.lock`, is a build's too: it
    /// is what building the package leaves, and what the next build of it makes again.
    pub fn foreign(&self, dir: &Path) -> Option<String> {
        let listed: BTreeSet<&str> = self.files.iter().map(|(name, _)| name.as_str()).collect();
        let mut pending = vec![dir.to_path_buf()];
        while let Some(next) = pending.pop() {
            let entries = match std::fs::read_dir(&next) {
                Ok(entries) => entries,
                Err(error) => return Some(format!("{}: {error}", next.display())),
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path
                    .strip_prefix(dir)
                    .map_or_else(|_| path.display().to_string(), |p| p.display().to_string());
                let kind = entry.file_type();
                if name == "target" || name == "Cargo.lock" || name == PROVENANCE_FILE {
                    continue;
                }
                if kind.as_ref().is_ok_and(std::fs::FileType::is_dir) {
                    pending.push(path);
                } else if !(kind.is_ok_and(|kind| kind.is_file()) && listed.contains(name.as_str()))
                {
                    return Some(format!("`{name}` is no file a build wrote"));
                }
            }
        }
        None
    }
}
