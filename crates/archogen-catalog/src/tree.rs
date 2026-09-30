//! The tracked set, as the loader's caller gives it (the record's §4).
//!
//! The loader never reads a working tree. Its caller hands it every tracked path with its bytes, from the git index
//! for the gate or from a commit for a claim, and every test builds one in memory. A directory is a path with at
//! least one tracked file under it, and nothing else.

use std::collections::BTreeMap;

/// Every tracked file, by path relative to the repository root, with its bytes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tree {
    files: BTreeMap<String, Vec<u8>>,
}

impl Tree {
    /// A tree of the given files.
    #[must_use]
    pub fn new(files: impl IntoIterator<Item = (String, Vec<u8>)>) -> Self {
        Self {
            files: files.into_iter().collect(),
        }
    }

    /// Add or replace one file.
    pub fn insert(&mut self, path: impl Into<String>, bytes: impl Into<Vec<u8>>) {
        self.files.insert(path.into(), bytes.into());
    }

    /// Remove one file.
    pub fn remove(&mut self, path: &str) {
        self.files.remove(path);
    }

    /// A tracked file's bytes.
    #[must_use]
    pub fn get(&self, path: &str) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }

    /// Whether `path` is a tracked file.
    #[must_use]
    pub fn is_file(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }

    /// Every tracked file under directory `dir`, in bytewise order. The root is `""`.
    pub fn under<'a>(&'a self, dir: &str) -> impl Iterator<Item = &'a str> + 'a {
        let prefix = if dir.is_empty() {
            String::new()
        } else {
            format!("{dir}/")
        };
        self.files
            .range(prefix.clone()..)
            .map(|(path, _)| path.as_str())
            .take_while(move |path| path.starts_with(&prefix))
    }

    /// Whether `path` is a directory: some tracked file lies under it.
    #[must_use]
    pub fn is_dir(&self, path: &str) -> bool {
        self.under(path).next().is_some()
    }

    /// Every tracked path, in bytewise order.
    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.files.keys().map(String::as_str)
    }
}

/// The directory holding `path`, `""` at the root.
#[must_use]
pub fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// `dir` joined with `name`, where `""` is the root.
#[must_use]
pub fn join(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_owned()
    } else {
        format!("{dir}/{name}")
    }
}

/// Resolve a relative path against `dir` lexically, `.` and `..` included. `None` when it climbs above the root.
#[must_use]
pub fn resolve(dir: &str, relative: &str) -> Option<String> {
    let mut parts: Vec<&str> = if dir.is_empty() {
        Vec::new()
    } else {
        dir.split('/').collect()
    };
    for segment in relative.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

/// `dir` and each of its ancestors, the root `""` last.
#[must_use]
pub fn ancestors(dir: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut current = dir;
    loop {
        out.push(current);
        if current.is_empty() {
            return out;
        }
        current = parent(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directories_are_prefixes_of_tracked_files() {
        let tree = Tree::new([
            ("a/b/c.rs".into(), vec![]),
            ("a/bc.rs".into(), vec![]),
            ("z".into(), vec![]),
        ]);
        assert_eq!(tree.under("a/b").collect::<Vec<_>>(), ["a/b/c.rs"]);
        assert_eq!(tree.under("a").count(), 2);
        assert_eq!(tree.under("").count(), 3);
        assert!(tree.is_dir("a/b") && !tree.is_dir("a/bc.rs") && !tree.is_dir("q"));
    }

    #[test]
    fn paths_resolve_lexically() {
        assert_eq!(resolve("crates/a", "../b").as_deref(), Some("crates/b"));
        assert_eq!(
            resolve("crates/a", "./x/../y").as_deref(),
            Some("crates/a/y")
        );
        assert_eq!(resolve("a", "../.."), None);
        assert_eq!(ancestors("a/b"), ["a/b", "a", ""]);
        assert_eq!(join("", "x"), "x");
    }
}
