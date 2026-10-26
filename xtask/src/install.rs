//! `cargo xtask install --to <dir> [--commit <rev>] [--out <dir>]` (leaf `M3.5.1.3`): the trust build's executables
//! placed in one directory, and `archogen-install/1` written beside them.
//!
//! ⭐ **Every digest from the inventory.** The step builds the trust inventory of the commit (`trust::inventory`), so
//! what it places is what the trust gate judges: the build's identity, the inventory's own sha256 and each program's
//! sha256 come from it, and each file is hashed before it is copied and again where it lands, so a copy that is not
//! the inventoried artifact is refused. Which roots are programs to place is read from the commit's own
//! `trust/roots.eadl`: the generator's executable is the front, and every other root whose target is an executable is
//! a sibling — the configuration checker's, `archogen-plan-check`, since `M3.5.6.2`; the scheduling checker's with
//! `M2.21.3`.
//!
//! ⛔ **It removes nothing, and a failed install leaves no manifest.** The directory must be absent or empty, and the
//! manifest is written last, through a file renamed into place, so the front never reads a manifest naming a program
//! that is not there. What the manifest guards against, and what not, is the format's (`archogen_install`).

use std::fs;
use std::path::{Path, PathBuf};

use archogen_evidence::sha256::Digest;
use archogen_install::{Build, Manifest, Program, FILE, FRONT_ROLE};

use crate::json::Json;
use crate::trust::{self, Outcome, Roots, Target};

/// Run the step: exit 0 installed, 1 the inventory or the install refused, 2 usage or a failure to run.
pub fn run(repo: &Path, args: &[&str]) -> i32 {
    let mut commit = "HEAD".to_owned();
    let mut out = repo.join("target").join("install").join("trust");
    let mut to: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        match (args[i], args.get(i + 1)) {
            ("--commit", Some(v)) => commit = (*v).to_owned(),
            ("--out", Some(v)) => out = repo.join(v),
            ("--to", Some(v)) => to = Some(repo.join(v)),
            (other, _) => {
                eprintln!("install: unknown argument `{other}`");
                return 2;
            }
        }
        i += 2;
    }
    let Some(to) = to else {
        eprintln!("install: the directory to install into is required: --to <dir>");
        return 2;
    };
    let inventory = match trust::inventory(repo, &commit, &out) {
        Ok(Outcome::Written(inventory)) => inventory,
        Ok(Outcome::Refused(refusals)) => {
            for r in &refusals {
                eprintln!("{r}");
            }
            eprintln!(
                "install: the inventory refused the commit ({} refusal(s)); nothing installed",
                refusals.len()
            );
            return 1;
        }
        Err(e) => {
            eprintln!("install: {e}");
            return 2;
        }
    };
    let placed = inventory_digest(&out).and_then(|digest| {
        let commit = identity(&inventory, "commit")?;
        let roots = crate::catalog_check::Git::at(repo)
            .text(&["show", &format!("{commit}:trust/roots.eadl")])
            .and_then(|text| trust::read_roots(&text))?;
        place(&inventory, digest, &roots, &out.join("target"), &to)
    });
    match placed {
        Ok(manifest) => {
            println!(
                "install: placed {} and {} sibling(s) in {}, from commit {}; manifest {}",
                manifest.front().name,
                manifest.siblings().len(),
                manifest.directory(),
                manifest.build().commit,
                FILE
            );
            0
        }
        Err(e) => {
            eprintln!("install: {e}");
            1
        }
    }
}

/// The sha256 of the inventory the step read.
fn inventory_digest(out: &Path) -> Result<Digest, String> {
    let path = out.join("trust-dependencies.json");
    fs::read(&path)
        .map(|bytes| Digest::of(&bytes))
        .map_err(|e| format!("cannot read the inventory `{}`: {e}", path.display()))
}

fn identity<'a>(inventory: &'a Json, key: &str) -> Result<&'a str, String> {
    inventory
        .get("identity")
        .and_then(|identity| identity.get(key))
        .and_then(Json::as_str)
        .ok_or_else(|| format!("the inventory names no `identity.{key}`"))
}

/// One program to place: where its artifact is, the name it takes, and what it must hash to.
struct Placing {
    role: String,
    name: String,
    source: PathBuf,
    sha256: Digest,
}

/// Place every executable root's artifact from `builds` — the inventory's `target/`, one directory per root — in
/// `to`, and write the manifest there last.
pub(crate) fn place(
    inventory: &Json,
    inventory_sha256: Digest,
    roots: &Roots,
    builds: &Path,
    to: &Path,
) -> Result<Manifest, String> {
    let mut front: Option<Placing> = None;
    let mut siblings: Vec<Placing> = Vec::new();
    for root in roots.programs.iter().filter(|p| p.pair.is_none()) {
        let Target::Bin(bin) = &root.target else {
            if root.role == FRONT_ROLE {
                return Err(format!(
                    "the generator's root `{}` is not an executable, so there is no front to install",
                    root.name
                ));
            }
            continue;
        };
        let record = inventory
            .get("programs")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .find(|p| p.get("name").and_then(Json::as_str) == Some(root.name.as_str()))
            .ok_or_else(|| format!("the inventory holds no program `{}`", root.name))?;
        let artifact = |key: &str| {
            record
                .get("artifact")
                .and_then(|a| a.get(key))
                .and_then(Json::as_str)
                .ok_or_else(|| format!("the inventory's `{}` names no artifact {key}", root.name))
        };
        let hex = artifact("sha256")?;
        let placing = Placing {
            role: root.role.clone(),
            name: bin.clone(),
            source: builds.join(&root.name).join(artifact("path")?),
            sha256: Digest::parse(&format!("sha256:{hex}")).ok_or_else(|| {
                format!("the inventory's `{}` sha256 `{hex}` is not one", root.name)
            })?,
        };
        if root.role == FRONT_ROLE {
            front = Some(placing);
        } else {
            siblings.push(placing);
        }
    }
    let front = front.ok_or("the roots name no generator, so there is no front to install")?;
    siblings.sort_by(|a, b| a.name.cmp(&b.name));

    // The directory: absent or empty, so an install removes nothing.
    fs::create_dir_all(to).map_err(|e| format!("cannot create `{}`: {e}", to.display()))?;
    let mut entries =
        fs::read_dir(to).map_err(|e| format!("cannot read `{}`: {e}", to.display()))?;
    if entries.next().is_some() {
        return Err(format!(
            "`{}` is not empty: an install goes into an empty directory and removes nothing",
            to.display()
        ));
    }
    let directory = fs::canonicalize(to)
        .map_err(|e| format!("cannot resolve `{}`: {e}", to.display()))?
        .to_str()
        .ok_or_else(|| {
            format!(
                "`{}` is not UTF-8, so no manifest can name it",
                to.display()
            )
        })?
        .to_owned();

    let mut programs = Vec::new();
    for placing in std::iter::once(&front).chain(&siblings) {
        let bytes = fs::read(&placing.source)
            .map_err(|e| format!("cannot read `{}`: {e}", placing.source.display()))?;
        if Digest::of(&bytes) != placing.sha256 {
            return Err(format!(
                "`{}` has sha256 {}, and the inventory recorded {}: not the inventoried artifact",
                placing.source.display(),
                Digest::of(&bytes),
                placing.sha256
            ));
        }
        let placed = to.join(&placing.name);
        fs::copy(&placing.source, &placed)
            .map_err(|e| format!("cannot place `{}`: {e}", placed.display()))?;
        let landed =
            fs::read(&placed).map_err(|e| format!("cannot read `{}`: {e}", placed.display()))?;
        if Digest::of(&landed) != placing.sha256 {
            return Err(format!(
                "`{}` changed while it was placed",
                placed.display()
            ));
        }
        programs.push(Program {
            name: placing.name.clone(),
            role: placing.role.clone(),
            sha256: placing.sha256,
        });
    }
    let front = programs.remove(0);
    let build = Build {
        commit: identity(inventory, "commit")?.to_owned(),
        tree: identity(inventory, "tree")?.to_owned(),
        rustc: identity(inventory, "rustc")?.to_owned(),
        host: identity(inventory, "host")?.to_owned(),
    };
    let manifest = Manifest::new(directory, build, inventory_sha256, front, programs)
        .map_err(|r| format!("the manifest cannot be made: {r}"))?;
    let partial = to.join(format!(".{FILE}.partial"));
    fs::write(&partial, manifest.write())
        .map_err(|e| format!("cannot write `{}`: {e}", partial.display()))?;
    fs::rename(&partial, to.join(FILE)).map_err(|e| format!("cannot place the manifest: {e}"))?;
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::{place, FILE};
    use crate::json::parse;
    use crate::trust::read_roots;
    use archogen_evidence::sha256::Digest;
    use archogen_install::Manifest;
    use std::fs;
    use std::path::PathBuf;

    const ROOTS: &str = r#"
(defroot generator
  (role generator)
  (package "crates/archogen-cli")
  (target bin archogen)
  (role-packages "crates/archogen-cli"))

(defroot scheduling-checker
  (role scheduling-checker)
  (package "crates/archogen-analyze")
  (target bin archogen-analyze)
  (role-packages "crates/archogen-analyze"))

(defroot reference-model
  (role reference-model)
  (package "crates/rt-reference")
  (target lib)
  (role-packages "crates/rt-reference"))
"#;

    /// A case's scratch under `target/`: the builds the inventory made, and the directory to install into.
    struct Case {
        builds: PathBuf,
        to: PathBuf,
    }

    fn case(name: &str) -> Case {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target/install-tests")
            .join(name);
        let _ = fs::remove_dir_all(&base);
        let builds = base.join("target");
        for (root, file, bytes) in [
            ("generator", "release/archogen", b"the front".as_slice()),
            (
                "scheduling-checker",
                "release/archogen-analyze",
                b"a sibling",
            ),
            (
                "reference-model",
                "release/librt_reference.rlib",
                b"a library",
            ),
        ] {
            let path = builds.join(root).join(file);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
        }
        Case {
            builds,
            to: base.join("bin"),
        }
    }

    /// An inventory that records the three artifacts, the generator's sha256 `front`.
    fn inventory(front: Digest) -> crate::json::Json {
        let hex = |d: Digest| d.hex();
        parse(&format!(
            r#"{{"identity":{{"commit":"263e1533ff447ef79fd73772f18e3ebd02d75d09","tree":"68c537aa79b4c2d151c3a266c9cf0d5c3e731e31","rustc":"rustc 1.95.0\nhost: x","host":"aarch64-apple-darwin"}},
"programs":[
{{"name":"generator","role":"generator","artifact":{{"path":"release/archogen","sha256":"{}"}}}},
{{"name":"scheduling-checker","role":"scheduling-checker","artifact":{{"path":"release/archogen-analyze","sha256":"{}"}}}},
{{"name":"reference-model","role":"reference-model","artifact":{{"path":"release/librt_reference.rlib","sha256":"{}"}}}}]}}"#,
            hex(front),
            hex(Digest::of(b"a sibling")),
            hex(Digest::of(b"a library")),
        ))
        .unwrap()
    }

    #[test]
    fn the_front_and_each_executable_sibling_are_placed_and_named() {
        let c = case("placed");
        let roots = read_roots(ROOTS).unwrap();
        let manifest = place(
            &inventory(Digest::of(b"the front")),
            Digest::of(b"i"),
            &roots,
            &c.builds,
            &c.to,
        )
        .unwrap();
        assert_eq!(manifest.front().name, "archogen");
        let names: Vec<&str> = manifest
            .siblings()
            .iter()
            .map(|p| p.name.as_str())
            .collect();
        assert_eq!(
            names,
            ["archogen-analyze"],
            "the library root is no program to place"
        );
        assert_eq!(fs::read(c.to.join("archogen")).unwrap(), b"the front");
        assert_eq!(
            fs::read(c.to.join("archogen-analyze")).unwrap(),
            b"a sibling"
        );
        assert!(!c.to.join("librt_reference.rlib").exists());
        let written = fs::read(c.to.join(FILE)).unwrap();
        assert_eq!(Manifest::read(&written, 1 << 20).unwrap(), manifest);
        assert_eq!(
            manifest.directory(),
            fs::canonicalize(&c.to).unwrap().to_str().unwrap()
        );
        let mut left: Vec<String> = fs::read_dir(&c.to)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        left.sort();
        assert_eq!(
            left,
            ["archogen", "archogen-analyze", FILE],
            "no partial manifest is left"
        );
    }

    #[test]
    fn what_is_not_the_inventoried_build_is_refused_and_no_manifest_written() {
        let roots = read_roots(ROOTS).unwrap();
        // An artifact whose bytes are not the ones the inventory hashed.
        let c = case("mismatch");
        let e = place(
            &inventory(Digest::of(b"another front")),
            Digest::of(b"i"),
            &roots,
            &c.builds,
            &c.to,
        )
        .unwrap_err();
        assert!(e.contains("not the inventoried artifact"), "{e}");
        assert!(!c.to.join(FILE).exists());
        // A directory that is not empty.
        let c = case("occupied");
        fs::create_dir_all(&c.to).unwrap();
        fs::write(c.to.join("notes.txt"), b"someone's").unwrap();
        let e = place(
            &inventory(Digest::of(b"the front")),
            Digest::of(b"i"),
            &roots,
            &c.builds,
            &c.to,
        )
        .unwrap_err();
        assert!(e.contains("is not empty"), "{e}");
        assert_eq!(
            fs::read(c.to.join("notes.txt")).unwrap(),
            b"someone's",
            "nothing removed"
        );
        assert!(!c.to.join(FILE).exists());
        // A missing artifact.
        let c = case("missing");
        fs::remove_file(c.builds.join("scheduling-checker/release/archogen-analyze")).unwrap();
        let e = place(
            &inventory(Digest::of(b"the front")),
            Digest::of(b"i"),
            &roots,
            &c.builds,
            &c.to,
        )
        .unwrap_err();
        assert!(e.contains("cannot read"), "{e}");
        assert!(!c.to.join(FILE).exists());
        // A generator that is not an executable.
        let c = case("library-front");
        let library =
            read_roots(&ROOTS.replacen("(target bin archogen)", "(target lib)", 1)).unwrap();
        let e = place(
            &inventory(Digest::of(b"the front")),
            Digest::of(b"i"),
            &library,
            &c.builds,
            &c.to,
        )
        .unwrap_err();
        assert!(e.contains("not an executable"), "{e}");
        assert!(!c.to.join(FILE).exists());
    }
}
