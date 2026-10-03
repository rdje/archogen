//! `cargo xtask catalog-check` — the catalog's gate and CI check (leaf `M2.7.4.2`): the history read from git as
//! §4 of `docs/specs/catalog/decision_catalog-records.md` says, the catalog loaded at the judged commit by the crate
//! and replayed against its bases (§9).
//!
//! > The loader is I/O-free. Its caller gives it the tracked set and each file's bytes, and a history … The caller
//! > that runs git is the repository's own tooling under `xtask/`.
//!
//! Three modes judge three commits:
//! - `--index`: the gate's pending commit, read from the index, its parents `HEAD` and, during a merge,
//!   `MERGE_HEAD`, its date the one `git var GIT_COMMITTER_IDENT` gives (§4);
//! - `--commit <sha>`: a commit as made, against its parents — what the hook runs again after an amend (§4);
//! - `--base <dir> --judged <dir> --base-commit <sha> --judged-commit <sha>`: the CI check, the judged commit
//!   against the base (`M2.7.6.3`'s entry; the two directories are the trees the harness wrote from blobs, which
//!   §3's builds take, `M2.7.4.3`).
//!
//! Every git call runs under §4's allowlist — `PATH`, `HOME`, the `GIT_DIR`, `GIT_INDEX_FILE` and `GIT_WORK_TREE` a
//! hook is given, and `GIT_NO_REPLACE_OBJECTS` — with `core.commitGraph=false`, and the run refuses a shallow
//! repository and a grafts file (premise 2). Every file is read from its blob, never from the working tree. A
//! commit with no `catalog/` holds an empty catalog, which reads nothing else, so its tree is given empty; the
//! blobs of a commit holding records are read whole, since a record's hashes reach any file it names.
//!
//! Exit 0: the catalog loads and the replay passes. 1: a refusal, named with its code. 2: the run could not judge —
//! git failed, a premise's check failed, or the pin is not the compiler.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use archogen_catalog::history::{Commit, CommitterDate, History};
use archogen_catalog::load::load;

use crate::catalog_build;
use archogen_catalog::replay::replay;
use archogen_catalog::tree::Tree;
use archogen_catalog::Refusal;

/// Which commit the checker judges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// The gate's pending commit, from the index.
    Index,
    /// A commit as made, against its parents.
    Commit(String),
    /// The CI check: the judged commit against the base, with the trees the harness wrote.
    Ci {
        base_dir: PathBuf,
        judged_dir: PathBuf,
        base_commit: String,
        judged_commit: String,
    },
}

/// Why a run gave no verdict (exit 2), as against a refusal (exit 1).
#[derive(Debug)]
pub enum Failure {
    /// The catalog, the lock or the replay refused.
    Refused(Refusal),
    /// The run could not judge.
    Unjudged(String),
}

impl From<Refusal> for Failure {
    fn from(r: Refusal) -> Self {
        Self::Refused(r)
    }
}

impl From<String> for Failure {
    fn from(why: String) -> Self {
        Self::Unjudged(why)
    }
}

/// What a passing run judged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// The commit judged: `index` for the pending one.
    pub head: String,
    /// Its bases.
    pub bases: Vec<String>,
    /// How many records it holds.
    pub records: usize,
    /// Whether it holds a lock.
    pub lock: bool,
    /// The pin the compiler was held to.
    pub pin: String,
    /// What §3's builds did.
    pub built: catalog_build::Built,
}

/// Parse the subcommand's arguments.
///
/// # Errors
///
/// A usage error, in words.
pub fn parse(args: &[&str]) -> Result<Mode, String> {
    match args {
        ["--index"] => Ok(Mode::Index),
        ["--commit", sha] => Ok(Mode::Commit((*sha).to_owned())),
        _ => {
            let mut map: BTreeMap<&str, &str> = BTreeMap::new();
            let mut rest = args;
            const KEYS: [&str; 4] = ["--base", "--judged", "--base-commit", "--judged-commit"];
            while let [key, value, tail @ ..] = rest {
                if !KEYS.contains(key) || map.insert(key, value).is_some() {
                    return Err(format!("unexpected or repeated argument `{key}`"));
                }
                rest = tail;
            }
            if !rest.is_empty() {
                return Err(format!("a value is missing after `{}`", rest[0]));
            }
            let take = |name: &str| {
                map.get(name)
                    .copied()
                    .ok_or_else(|| format!("`{name}` is required"))
            };
            let mode = Mode::Ci {
                base_dir: PathBuf::from(take("--base")?),
                judged_dir: PathBuf::from(take("--judged")?),
                base_commit: take("--base-commit")?.to_owned(),
                judged_commit: take("--judged-commit")?.to_owned(),
            };
            if map.len() != 4 {
                return Err("`catalog-check` takes `--index`, `--commit <sha>`, or `--base <dir> --judged <dir> --base-commit <sha> --judged-commit <sha>`".to_owned());
            }
            Ok(mode)
        }
    }
}

/// Git, run under §4's allowlist from `cwd`, in the repository git discovers from there.
struct Git {
    cwd: PathBuf,
    env: Vec<(&'static str, OsString)>,
}

impl Git {
    fn at(cwd: &Path) -> Self {
        let mut env: Vec<(&'static str, OsString)> = Vec::new();
        for key in ["PATH", "HOME", "GIT_DIR", "GIT_INDEX_FILE", "GIT_WORK_TREE"] {
            if let Some(value) = std::env::var_os(key) {
                env.push((key, value));
            }
        }
        env.push(("GIT_NO_REPLACE_OBJECTS", OsString::from("1")));
        Self {
            cwd: cwd.to_path_buf(),
            env,
        }
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("git");
        command
            .args(["-c", "core.commitGraph=false"])
            .args(args)
            .current_dir(&self.cwd)
            .env_clear();
        for (key, value) in &self.env {
            command.env(key, value);
        }
        command
    }

    /// Run git, taking its standard output as bytes.
    fn bytes(&self, args: &[&str]) -> Result<Vec<u8>, String> {
        let output = self
            .command(args)
            .output()
            .map_err(|e| format!("git did not run: {e}"))?;
        if !output.status.success() {
            return Err(format!(
                "`git {}` failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(output.stdout)
    }

    fn text(&self, args: &[&str]) -> Result<String, String> {
        let bytes = self.bytes(args)?;
        String::from_utf8(bytes)
            .map_err(|_| format!("`git {}` printed bytes that are not UTF-8", args.join(" ")))
    }

    /// Run git with `input` on its standard input, taking its standard output as bytes; the input is written on
    /// a thread of its own, so an output larger than a pipe does not block the write.
    fn piped(&self, args: &[&str], input: Vec<u8>) -> Result<Vec<u8>, String> {
        let mut child = self
            .command(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("git did not run: {e}"))?;
        let mut stdin = child.stdin.take().ok_or("git's standard input")?;
        let writer = std::thread::spawn(move || stdin.write_all(&input));
        let output = child
            .wait_with_output()
            .map_err(|e| format!("git did not finish: {e}"))?;
        writer
            .join()
            .map_err(|_| "the writer thread panicked".to_owned())?
            .map_err(|e| format!("writing to git: {e}"))?;
        if !output.status.success() {
            return Err(format!(
                "`git {}` failed: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(output.stdout)
    }

    /// Premise 2's cheap checks: no shallow repository, no grafts file.
    fn premise_2(&self) -> Result<(), String> {
        if self.text(&["rev-parse", "--is-shallow-repository"])?.trim() != "false" {
            return Err("the repository is shallow: a shallow boundary would be a false ledgering commit of every line (§4)".to_owned());
        }
        let grafts = self.text(&["rev-parse", "--git-path", "info/grafts"])?;
        let grafts = self.cwd.join(grafts.trim());
        if grafts.exists() {
            return Err(format!(
                "a grafts file is present at {} (premise 2)",
                grafts.display()
            ));
        }
        Ok(())
    }

    /// The full object name `rev` names.
    fn resolve(&self, rev: &str) -> Result<String, String> {
        Ok(self
            .text(&[
                "rev-parse",
                "--verify",
                "--end-of-options",
                &format!("{rev}^{{commit}}"),
            ])?
            .trim()
            .to_owned())
    }

    /// Whether `rev` names a commit.
    fn exists(&self, rev: &str) -> bool {
        self.command(&[
            "rev-parse",
            "--verify",
            "-q",
            "--end-of-options",
            &format!("{rev}^{{commit}}"),
        ])
        .output()
        .is_ok_and(|o| o.status.success())
    }
}

/// One entry of a tree or index listing.
struct Entry {
    mode: String,
    sha: String,
    path: String,
}

/// `git ls-tree -r -z --full-tree <commit>`'s entries.
fn tree_entries(git: &Git, commit: &str) -> Result<Vec<Entry>, String> {
    let out = git.bytes(&["ls-tree", "-r", "-z", "--full-tree", commit])?;
    let mut entries = Vec::new();
    for record in out.split(|&b| b == 0).filter(|r| !r.is_empty()) {
        let text = String::from_utf8_lossy(record);
        let (meta, path) = text
            .split_once('\t')
            .ok_or_else(|| format!("an `ls-tree` line without a tab: {text}"))?;
        let mut parts = meta.split(' ');
        let mode = parts.next().unwrap_or_default().to_owned();
        let _kind = parts.next();
        let sha = parts.next().unwrap_or_default().to_owned();
        entries.push(Entry {
            mode,
            sha,
            path: path.to_owned(),
        });
    }
    Ok(entries)
}

/// `git ls-files -s -z`'s entries: the index, every stage `0`.
fn index_entries(git: &Git) -> Result<Vec<Entry>, String> {
    let out = git.bytes(&["ls-files", "-s", "-z"])?;
    let mut entries = Vec::new();
    for record in out.split(|&b| b == 0).filter(|r| !r.is_empty()) {
        let text = String::from_utf8_lossy(record);
        let (meta, path) = text
            .split_once('\t')
            .ok_or_else(|| format!("an `ls-files` line without a tab: {text}"))?;
        let mut parts = meta.split(' ');
        let mode = parts.next().unwrap_or_default().to_owned();
        let sha = parts.next().unwrap_or_default().to_owned();
        let stage = parts.next().unwrap_or_default();
        if stage != "0" {
            return Err(format!(
                "`{path}` is at stage {stage}: the index holds an unresolved merge"
            ));
        }
        entries.push(Entry {
            mode,
            sha,
            path: path.to_owned(),
        });
    }
    Ok(entries)
}

/// A tree from entries: every file of mode `100644` or `100755`, its bytes from its blob (§4's "tracked"); a
/// symbolic link or a submodule entry under `catalog/` is refused, since no reader of the catalog may follow one.
fn tree_of(git: &Git, entries: &[Entry]) -> Result<Tree, Failure> {
    let mut files: Vec<&Entry> = Vec::new();
    for entry in entries {
        match entry.mode.as_str() {
            "100644" | "100755" => files.push(entry),
            mode if entry.path == "catalog" || entry.path.starts_with("catalog/") => {
                return Err(Refusal::new(
                    archogen_catalog::Code::Layout,
                    &entry.path,
                    "(file)",
                    None,
                    format!("mode {mode} under `catalog/`: a symbolic link or a submodule entry is not tracked (§4)"),
                )
                .into());
            }
            _ => {}
        }
    }
    let input: Vec<u8> = files
        .iter()
        .flat_map(|e| format!("{}\n", e.sha).into_bytes())
        .collect();
    let out = git.piped(&["cat-file", "--batch"], input)?;
    let mut at = 0;
    let mut tree = Tree::new([]);
    for entry in files {
        let header_end = out[at..]
            .iter()
            .position(|&b| b == b'\n')
            .ok_or_else(|| format!("`cat-file --batch` ended before `{}`", entry.path))?;
        let header = String::from_utf8_lossy(&out[at..at + header_end]).into_owned();
        at += header_end + 1;
        let mut parts = header.split(' ');
        let sha = parts.next().unwrap_or_default();
        let kind = parts.next().unwrap_or_default();
        let size: usize = parts.next().and_then(|s| s.parse().ok()).ok_or_else(|| {
            format!(
                "`cat-file --batch` answered `{header}` for `{}`",
                entry.path
            )
        })?;
        if sha != entry.sha || kind != "blob" {
            return Err(format!(
                "`cat-file --batch` answered `{header}` for `{}`",
                entry.path
            )
            .into());
        }
        let bytes = out
            .get(at..at + size)
            .ok_or_else(|| format!("`cat-file --batch` cut `{}` short", entry.path))?
            .to_vec();
        at += size + 1;
        tree.insert(entry.path.clone(), bytes);
    }
    Ok(tree)
}

/// Which of `commits` hold a `catalog/` tree: one `cat-file --batch-check` over `<sha>:catalog` each.
fn with_catalog(git: &Git, commits: &[String]) -> Result<Vec<bool>, String> {
    let input: Vec<u8> = commits
        .iter()
        .flat_map(|c| format!("{c}:catalog\n").into_bytes())
        .collect();
    let out = git.piped(&["cat-file", "--batch-check"], input)?;
    let text = String::from_utf8_lossy(&out);
    let answers: Vec<&str> = text.lines().collect();
    if answers.len() != commits.len() {
        return Err("`cat-file --batch-check` answered a different number of lines".to_owned());
    }
    Ok(answers.iter().map(|a| !a.ends_with(" missing")).collect())
}

/// A committer date from `<seconds> <+hhmm>`.
fn committer_date(seconds: &str, offset: &str) -> Result<CommitterDate, String> {
    let seconds: i64 = seconds
        .parse()
        .map_err(|_| format!("a committer date `{seconds}` that is not a number of seconds"))?;
    let (sign, digits) = match offset.as_bytes().first() {
        Some(b'+') => (1, &offset[1..]),
        Some(b'-') => (-1, &offset[1..]),
        _ => return Err(format!("a time-zone offset `{offset}` without a sign")),
    };
    if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("a time-zone offset `{offset}` that is not `+hhmm`"));
    }
    let hours: i32 = digits[..2].parse().unwrap_or(0);
    let minutes: i32 = digits[2..].parse().unwrap_or(0);
    Ok(CommitterDate {
        seconds,
        offset_minutes: sign * (hours * 60 + minutes),
    })
}

/// The pending commit's date: what `git var GIT_COMMITTER_IDENT` gives, `… <seconds> <+hhmm>` (§4).
fn pending_date(git: &Git) -> Result<CommitterDate, String> {
    let ident = git.text(&["var", "GIT_COMMITTER_IDENT"])?;
    let mut tokens = ident.split_whitespace().rev();
    let offset = tokens.next().ok_or("an empty `GIT_COMMITTER_IDENT`")?;
    let seconds = tokens
        .next()
        .ok_or("a `GIT_COMMITTER_IDENT` with no seconds")?;
    committer_date(seconds, offset)
}

/// The history every commit `heads` descend from: parents and dates from one `git log`, and each commit's tree —
/// empty where the commit holds no `catalog/`, read whole from its blobs otherwise.
fn history_of(git: &Git, heads: &[&str]) -> Result<History, Failure> {
    let mut args = vec!["log", "--format=%H%x00%P%x00%ct%x00%ci", "--end-of-options"];
    args.extend_from_slice(heads);
    let log = git.text(&args)?;
    let mut rows: Vec<(String, Vec<String>, CommitterDate)> = Vec::new();
    for line in log.lines() {
        let fields: Vec<&str> = line.split('\0').collect();
        let [sha, parents, seconds, iso] = fields.as_slice() else {
            return Err(format!("a `git log` line of {} fields", fields.len()).into());
        };
        let offset = iso.rsplit(' ').next().unwrap_or_default();
        rows.push((
            (*sha).to_owned(),
            parents.split_whitespace().map(str::to_owned).collect(),
            committer_date(seconds, offset)?,
        ));
    }
    let names: Vec<String> = rows.iter().map(|r| r.0.clone()).collect();
    let holds = with_catalog(git, &names)?;
    let mut history = History::default();
    for ((sha, parents, date), holds_catalog) in rows.into_iter().zip(holds) {
        let tree = if holds_catalog {
            tree_of(git, &tree_entries(git, &sha)?)?
        } else {
            Tree::new([])
        };
        history.insert(
            sha,
            Commit {
                parents,
                date,
                tree,
                hosting: false,
            },
        );
    }
    Ok(history)
}

/// `rustc -vV` under §3's allowlist, with `RUSTUP_TOOLCHAIN` the pin, names the pin (premise 1).
fn compiler_is(pin: &str, scratch: &Path) -> Result<(), String> {
    let cargo_home = scratch.join("cargo-home");
    std::fs::create_dir_all(&cargo_home).map_err(|e| e.to_string())?;
    let mut command = Command::new("rustc");
    command.arg("-vV").env_clear();
    for key in ["PATH", "HOME", "RUSTUP_HOME"] {
        if let Some(value) = std::env::var_os(key) {
            command.env(key, value);
        }
    }
    if std::env::var_os("RUSTUP_HOME").is_none() {
        if let Some(home) = std::env::var_os("HOME") {
            command.env("RUSTUP_HOME", Path::new(&home).join(".rustup"));
        }
    }
    command
        .env("RUSTUP_TOOLCHAIN", pin)
        .env("CARGO_HOME", &cargo_home)
        .env("CARGO_TARGET_DIR", scratch.join("records-target"));
    let output = command
        .output()
        .map_err(|e| format!("`rustc -vV` did not run: {e}"))?;
    let text = String::from_utf8_lossy(&output.stdout);
    let release = text.lines().find_map(|l| l.strip_prefix("release: "));
    match release {
        Some(r) if r.trim() == pin => Ok(()),
        _ => Err(format!(
            "`rustc -vV` does not name the pin {pin}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )),
    }
}

/// Judge `mode` in the repository git discovers from `cwd`, with `scratch` for the compiler check's own
/// directories.
///
/// # Errors
///
/// [`Failure::Refused`] with the catalog's refusal; [`Failure::Unjudged`] when the run could not judge.
pub fn judge(cwd: &Path, scratch: &Path, mode: &Mode) -> Result<Verdict, Failure> {
    let git = Git::at(cwd);
    git.premise_2()?;
    let (head, bases, tree, date, history) = match mode {
        Mode::Index => {
            let mut bases = vec![git.resolve("HEAD")?];
            if git.exists("MERGE_HEAD") {
                bases.push(git.resolve("MERGE_HEAD")?);
            }
            let untracked = git.bytes(&["ls-files", "--others", "-z", "--", "catalog"])?;
            if let Some(first) = untracked.split(|&b| b == 0).find(|p| !p.is_empty()) {
                return Err(Refusal::new(
                    archogen_catalog::Code::Layout,
                    &String::from_utf8_lossy(first),
                    "(file)",
                    None,
                    "an untracked file under `catalog/`, which no reader of the index can see (§4)",
                )
                .into());
            }
            let tree = tree_of(&git, &index_entries(&git)?)?;
            let date = pending_date(&git)?;
            let refs: Vec<&str> = bases.iter().map(String::as_str).collect();
            let history = history_of(&git, &refs)?;
            ("index".to_owned(), bases, tree, date, history)
        }
        Mode::Commit(rev) => {
            let head = git.resolve(rev)?;
            let history = history_of(&git, &[head.as_str()])?;
            let commit = history.get(&head)?;
            let bases = commit.parents.clone();
            let date = commit.date;
            let tree = tree_of(&git, &tree_entries(&git, &head)?)?;
            (head, bases, tree, date, history)
        }
        Mode::Ci {
            base_dir,
            judged_dir,
            base_commit,
            judged_commit,
        } => {
            for (name, dir) in [("base", base_dir), ("judged", judged_dir)] {
                if !dir.is_dir() {
                    return Err(
                        format!("the {name} tree `{}` is not a directory", dir.display()).into(),
                    );
                }
            }
            let head = git.resolve(judged_commit)?;
            let base = git.resolve(base_commit)?;
            let history = history_of(&git, &[head.as_str(), base.as_str()])?;
            let date = history.get(&head)?.date;
            let tree = tree_of(&git, &tree_entries(&git, &head)?)?;
            (head, vec![base], tree, date, history)
        }
    };
    let pin = catalog_build::toolchain_file(&tree)?;
    compiler_is(&pin, scratch)?;
    let repo = PathBuf::from(git.text(&["rev-parse", "--show-toplevel"])?.trim());
    let root: PathBuf = match mode {
        Mode::Ci { judged_dir, .. } => judged_dir.clone(),
        Mode::Index | Mode::Commit(_) => {
            let dir = scratch.join("tree");
            catalog_build::write_tree(&tree, &dir)?;
            dir
        }
    };
    let mut history = history;
    let records = tree
        .under("catalog")
        .filter(|p| p.ends_with(".catalog"))
        .count();
    let lock = tree.is_file(archogen_catalog::lock::PATH);
    history.insert(
        head.clone(),
        Commit {
            parents: bases.clone(),
            date,
            tree,
            hosting: false,
        },
    );
    let loaded = load(&history, &head)?;
    let base_refs: Vec<&str> = bases.iter().map(String::as_str).collect();
    replay(&history, &base_refs, &head)?;
    let judged_tree = &history.get(&head)?.tree;
    let built = catalog_build::check(&loaded, judged_tree, &root, &repo, &pin, scratch)?;
    Ok(Verdict {
        head,
        bases,
        records,
        lock,
        pin,
        built,
    })
}

/// The subcommand: judge, report, and exit as the module doc says.
pub fn run(root: &Path, args: &[&str]) -> i32 {
    let mode = match parse(args) {
        Ok(m) => m,
        Err(why) => {
            eprintln!("catalog-check: {why}");
            return 2;
        }
    };
    let cwd = std::env::current_dir().unwrap_or_else(|_| root.to_path_buf());
    match judge(&cwd, &root.join("target/catalog-check/scratch"), &mode) {
        Ok(verdict) => {
            println!(
                "catalog-check: {} against {}: {} record(s), {}, the compiler the pin {}, {} package(s) built {} time(s), {} unit(s) read: pass",
                verdict.head,
                if verdict.bases.is_empty() {
                    "no base".to_owned()
                } else {
                    verdict.bases.join(" and ")
                },
                verdict.records,
                if verdict.lock { "a lock" } else { "no lock" },
                verdict.pin,
                verdict.built.packages,
                verdict.built.builds,
                verdict.built.units
            );
            0
        }
        Err(Failure::Refused(r)) => {
            eprintln!("catalog-check: refused: {r}");
            1
        }
        Err(Failure::Unjudged(why)) => {
            eprintln!("catalog-check: could not judge: {why}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use archogen_catalog::hash::Catalog;
    use archogen_catalog::lock::{blessed, Line, Lock};
    use archogen_catalog::tree::Tree;
    use archogen_catalog::Code;

    use super::{committer_date, judge, parse, Failure, Mode};

    #[test]
    fn the_arguments_parse_or_say_why() {
        assert_eq!(parse(&["--index"]).unwrap(), Mode::Index);
        assert_eq!(
            parse(&["--commit", "abc"]).unwrap(),
            Mode::Commit("abc".into())
        );
        let ci = parse(&[
            "--base",
            "b",
            "--judged",
            "j",
            "--base-commit",
            "1",
            "--judged-commit",
            "2",
        ])
        .unwrap();
        assert!(matches!(ci, Mode::Ci { ref base_commit, .. } if base_commit == "1"));
        assert!(parse(&["--base", "b"]).unwrap_err().contains("--judged"));
        assert!(parse(&["--base"]).unwrap_err().contains("missing"));
        assert!(parse(&["--index", "x"]).unwrap_err().contains("unexpected"));
    }

    #[test]
    fn a_committer_date_reads_its_offset() {
        let d = committer_date("1759500000", "+0200").unwrap();
        assert_eq!((d.seconds, d.offset_minutes), (1_759_500_000, 120));
        assert_eq!(committer_date("5", "-0530").unwrap().offset_minutes, -330);
        assert!(committer_date("x", "+0000").is_err());
        assert!(committer_date("5", "0200").is_err());
    }

    /// The worked example's `example.base`, without its review: a record that loads over the example's files.
    const RECORD: &str = r#"(catalog-record example.base
  (version "0.1.0")
  (catalog machine)
  (source (origin "the worked example of decision_catalog-records.md") (license "MIT OR Apache-2.0"))
  (maintainer M2)
  (depends)
  (supersedes)
  (profiles rt-static-up-v1)
  (targets example-target)
  (preconditions "a \"quoted\" precondition")
  (guarantees "one processor")
  (implementation (version "0.1.0") (none "a machine has no code"))
  (behavior-model (version "0.1.0") (sources "docs/example/model.txt") (describes)
    (facts (fact one-processor yes (locator (file "docs/example/model.txt")) (basis "the model says so"))))
  (timing-model (version "0.1.0") (none "the costs of a machine are its devices'")))
"#;

    /// The example's review of the behavioral model, which verifies over the example's files.
    const REVIEW: &str = r#"  (review (facet behavior-model)
          (hash "sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813")
          (verdict production) (by independent-context "the worked example") (date "2026-09-30")
          (basis "the model file says one processor")))
"#;

    fn with_review(record: &str) -> String {
        let body = record.trim_end();
        format!("{}\n{REVIEW}", &body[..body.len() - 1])
    }

    /// A scratch repository under `target/`, with git's identity fixed.
    struct Repo {
        dir: PathBuf,
    }

    impl Repo {
        fn new(name: &str) -> Self {
            let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("target/catalog-check-tests")
                .join(name);
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            let repo = Self { dir };
            repo.git(&["init", "-q"]);
            repo
        }

        fn git(&self, args: &[&str]) -> String {
            let out = Command::new("git")
                .args([
                    "-c",
                    "user.name=t",
                    "-c",
                    "user.email=t@t",
                    "-c",
                    "commit.gpgsign=false",
                ])
                .args(args)
                .current_dir(&self.dir)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "git {args:?}: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8_lossy(&out.stdout).trim().to_owned()
        }

        fn write(&self, path: &str, text: &str) {
            let full = self.dir.join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(full, text).unwrap();
        }

        fn commit(&self, message: &str) -> String {
            self.git(&["add", "-A"]);
            self.git(&["commit", "-q", "-m", message]);
            self.git(&["rev-parse", "HEAD"])
        }

        /// The files the test wrote, staged, as a tree.
        fn tree(&self) -> Tree {
            self.git(&["add", "-A"]);
            let listed = self.git(&["ls-files"]);
            Tree::new(
                listed
                    .lines()
                    .map(|p| (p.to_owned(), fs::read(self.dir.join(p)).unwrap())),
            )
        }

        /// Bless the lock over the tree as written, keeping `parent`'s lines.
        fn bless(&self, parent: Option<&str>) {
            let mut tree = self.tree();
            tree.remove(archogen_catalog::lock::PATH);
            let catalog = Catalog::read(tree).unwrap_or_else(|e| panic!("{e}"));
            let hashes = catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
            let mut lines: std::collections::BTreeSet<Line> = blessed(&catalog, &hashes);
            if let Some(parent) = parent {
                lines.extend(Lock::parse(parent.as_bytes()).unwrap().lines);
            }
            self.write(archogen_catalog::lock::PATH, &Lock::new(1, lines).render());
        }

        fn lock(&self) -> String {
            fs::read_to_string(self.dir.join(archogen_catalog::lock::PATH)).unwrap()
        }

        fn judge(&self, mode: &Mode) -> Result<super::Verdict, Failure> {
            judge(&self.dir, &self.dir.join("scratch"), mode)
        }
    }

    /// The example's files, the pin, and the record; committed with its blessed lock.
    fn seeded(name: &str) -> (Repo, String) {
        let repo = Repo::new(name);
        let pin = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("rust-toolchain.toml"),
        )
        .unwrap();
        repo.write("rust-toolchain.toml", &pin);
        repo.write("docs/example/model.txt", "model\n");
        repo.write("targets/example-target.env", "TARGET_ID=example-target\n");
        repo.write("targets/example-target.eadl", "(platform)\n");
        repo.write("catalog/experimental/example.base.catalog", RECORD);
        repo.bless(None);
        let first = repo.commit("records and lock");
        (repo, first)
    }

    #[test]
    fn an_empty_catalog_passes_and_so_does_a_seeded_one() {
        let repo = Repo::new("empty");
        let pin = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("rust-toolchain.toml"),
        )
        .unwrap();
        repo.write("rust-toolchain.toml", &pin);
        repo.write("README", "nothing\n");
        let first = repo.commit("empty");
        let v = repo
            .judge(&Mode::Commit(first.clone()))
            .unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!((v.records, v.lock, v.bases.len()), (0, false, 0));
        // The index, with the second commit pending: one base, HEAD.
        repo.write("README", "something\n");
        repo.git(&["add", "-A"]);
        let v = repo.judge(&Mode::Index).unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!(
            (v.head.as_str(), v.bases.as_slice()),
            ("index", &[first][..])
        );

        let (repo, first) = seeded("seeded");
        let v = repo
            .judge(&Mode::Commit(first.clone()))
            .unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!((v.records, v.lock), (1, true));
        // A review ledgered in a second commit verifies there, and the CI mode judges it against the first.
        repo.write(
            "catalog/experimental/example.base.catalog",
            &with_review(RECORD),
        );
        let parent = repo.lock();
        repo.bless(Some(&parent));
        let second = repo.commit("a review");
        repo.judge(&Mode::Commit(second.clone()))
            .unwrap_or_else(|e| panic!("{e:?}"));
        let ci = Mode::Ci {
            base_dir: repo.dir.clone(),
            judged_dir: repo.dir.clone(),
            base_commit: first.clone(),
            judged_commit: second.clone(),
        };
        let v = repo.judge(&ci).unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!((v.head, v.bases), (second, vec![first]));
    }

    #[track_caller]
    fn refused(result: Result<super::Verdict, Failure>, code: Code, says: &str) {
        match result {
            Err(Failure::Refused(r)) => {
                assert_eq!(r.code, code, "{r}");
                assert!(r.message.contains(says), "not about `{says}`: {r}");
            }
            other => panic!("expected {code} about `{says}`, got {other:?}"),
        }
    }

    #[test]
    fn a_changed_record_whose_version_did_not_move_is_refused() {
        let (repo, _) = seeded("unbumped");
        repo.write(
            "catalog/experimental/example.base.catalog",
            &RECORD.replace("the model says so", "the model said so"),
        );
        repo.git(&["add", "-A"]);
        refused(repo.judge(&Mode::Index), Code::LockUnbumped, "");
        let sha = repo.commit("edited without a bump");
        refused(repo.judge(&Mode::Commit(sha)), Code::LockUnbumped, "");
    }

    #[test]
    fn a_removed_review_is_refused() {
        let (repo, _) = seeded("removed-review");
        repo.write(
            "catalog/experimental/example.base.catalog",
            &with_review(RECORD),
        );
        let parent = repo.lock();
        repo.bless(Some(&parent));
        let with = repo.commit("a review");
        let lock_with = repo.lock();
        // The form removed, the line kept: a present record lacks a review the ledger holds.
        repo.write("catalog/experimental/example.base.catalog", RECORD);
        repo.git(&["add", "-A"]);
        refused(repo.judge(&Mode::Index), Code::LockReview, "");
        // The line removed too: a parent's line dropped.
        repo.write(
            archogen_catalog::lock::PATH,
            &lock_with
                .lines()
                .filter(|l| !l.contains(" review "))
                .map(|l| format!("{l}\n"))
                .collect::<String>(),
        );
        repo.git(&["add", "-A"]);
        refused(repo.judge(&Mode::Index), Code::LockReview, "");
        let sha = repo.commit("review gone");
        refused(repo.judge(&Mode::Commit(sha.clone())), Code::LockReview, "");
        refused(
            repo.judge(&Mode::Ci {
                base_dir: repo.dir.clone(),
                judged_dir: repo.dir.clone(),
                base_commit: with,
                judged_commit: sha,
            }),
            Code::LockReview,
            "",
        );
    }

    #[test]
    fn a_version_below_the_lock_and_a_hand_added_line_are_refused() {
        let (repo, _) = seeded("downgrade");
        repo.write(
            "catalog/experimental/example.base.catalog",
            &RECORD.replace(
                "(version \"0.1.0\")\n  (catalog",
                "(version \"0.0.9\")\n  (catalog",
            ),
        );
        repo.git(&["add", "-A"]);
        refused(repo.judge(&Mode::Index), Code::LockDowngrade, "");
        repo.git(&["checkout", "--", "."]);
        let lock = repo.lock();
        repo.write(
            archogen_catalog::lock::PATH,
            &format!(
                "{lock}example.base contract 9.9.9 sha256:{}\n",
                "0".repeat(64)
            ),
        );
        repo.git(&["add", "-A"]);
        refused(repo.judge(&Mode::Index), Code::LockReview, "");
    }

    #[test]
    fn an_untracked_file_under_catalog_and_a_missing_pin_are_refused() {
        let (repo, first) = seeded("untracked");
        repo.write(
            "catalog/experimental/stray.catalog",
            "(catalog-record stray)\n",
        );
        refused(repo.judge(&Mode::Index), Code::Layout, "untracked");
        fs::remove_file(repo.dir.join("catalog/experimental/stray.catalog")).unwrap();
        repo.judge(&Mode::Commit(first))
            .unwrap_or_else(|e| panic!("{e:?}"));
        repo.git(&["rm", "-q", "rust-toolchain.toml"]);
        match repo.judge(&Mode::Index) {
            Err(Failure::Unjudged(why)) => assert!(why.contains("no pin"), "{why}"),
            other => panic!("{other:?}"),
        }
    }
}
