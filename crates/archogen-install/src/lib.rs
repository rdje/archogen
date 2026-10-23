//! `archogen-install/1`: the manifest an install writes beside the programs it placed (leaf `M3.5.1.3`).
//!
//! `cargo xtask install` builds the trust inventory of one commit, places that build's executables in one directory —
//! the front, `archogen`, and each checker's program — and writes this manifest there, last, every digest and the
//! build's identity taken from the inventory, so a program the manifest names is an inventoried one (`TI-H8`). The
//! front reads it before it runs a sibling (`M3.5.1.4`): it refuses a manifest that does not name the directory it
//! lies in and the front's own sha256, so a front and its siblings are one build, and it hashes a sibling before
//! running it.
//!
//! ⚠️ **What it guards against.** Whoever can write the install directory can replace the front and this manifest
//! together, so the digests catch a stale or mismatched sibling — an older build left beside a newer front, a program
//! copied in by hand — and never tampering (`M3.5.1`, the review's D2). The directory is trusted as far as the `archogen`
//! executable in it is.
//!
//! ⭐ **One text per manifest**, as `archogen-result/1`'s: [`Manifest::write`] writes compact JSON, its members in one
//! order, then a newline, and [`Manifest::read`] accepts a text only when writing what it read gives back the same
//! bytes. It does no I/O: the caller reads the file, and bounds how much.

use std::fmt;
use std::path::Path;

use archogen_digest::Digest;
use archogen_json::{self as json, Value};

/// This format's identifier. A reader that meets another refuses the manifest rather than guess at it.
pub const FORMAT: &str = "archogen-install/1";

/// The manifest's name in the directory it describes.
pub const FILE: &str = "archogen-install.json";

/// The role of the program the manifest calls the front.
pub const FRONT_ROLE: &str = "generator";

/// The build the installed programs came from, as the trust inventory names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Build {
    /// The commit inventoried: 40 or 64 lowercase hexadecimal digits.
    pub commit: String,
    /// That commit's tree, written the same way.
    pub tree: String,
    /// The compiler, as `rustc -vV` describes itself, every line of it.
    pub rustc: String,
    /// The host the build ran on.
    pub host: String,
}

/// One installed program: the name of its executable, the role it is the root of, and its sha256.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    /// The executable's file name in the directory: lowercase letters, digits and `-`.
    pub name: String,
    /// Its role in `trust/roots.eadl`, written the same way.
    pub role: String,
    /// The sha256 the inventory recorded for its artifact.
    pub sha256: Digest,
}

/// One install: built only through [`Manifest::new`] or [`Manifest::read`], so a manifest that exists is whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    directory: String,
    build: Build,
    inventory: Digest,
    front: Program,
    siblings: Vec<Program>,
}

/// Why a text is not a manifest, or why a manifest cannot be made: each names what disagreed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// Not a JSON text the strict reader admits, or longer than the bound.
    Json(json::Refusal),
    /// A JSON text, but not the one text [`Manifest::write`] writes for what it holds.
    NotCanonical,
    /// A format other than [`FORMAT`].
    Format(String),
    /// A member missing, unexpected or out of order, or a value of the wrong kind.
    Shape(String),
    /// A value its member does not admit, and the member's path.
    Value(String),
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(refusal) => write!(f, "not an install manifest: {refusal}"),
            Self::NotCanonical => write!(
                f,
                "not an install manifest: not the one text its writer writes for it"
            ),
            Self::Format(format) => write!(f, "a manifest in `{format}`, not `{FORMAT}`"),
            Self::Shape(what) | Self::Value(what) => {
                write!(f, "an install manifest that cannot be read: {what}")
            }
        }
    }
}

impl Manifest {
    /// A manifest; refused when it would not read back as itself or would not describe one install: a directory that
    /// is not absolute, a name or role outside its alphabet, an object id that is not one, a front that is not the
    /// generator's, a sibling that is, two programs of one name, or siblings out of their order by name.
    pub fn new(
        directory: String,
        build: Build,
        inventory: Digest,
        front: Program,
        siblings: Vec<Program>,
    ) -> Result<Self, Refusal> {
        if !Path::new(&directory).is_absolute() {
            return Err(Refusal::Value(format!(
                "`directory` is `{directory}`, not an absolute path"
            )));
        }
        object_id("build.commit", &build.commit)?;
        object_id("build.tree", &build.tree)?;
        if build.rustc.is_empty() {
            return Err(Refusal::Value("`build.rustc` is empty".to_owned()));
        }
        line("build.host", &build.host)?;
        program("front", &front)?;
        if front.role != FRONT_ROLE {
            return Err(Refusal::Value(format!(
                "`front.role` is `{}`, not `{FRONT_ROLE}`",
                front.role
            )));
        }
        for (index, sibling) in siblings.iter().enumerate() {
            let path = format!("siblings[{index}]");
            program(&path, sibling)?;
            if sibling.role == FRONT_ROLE || sibling.name == front.name {
                return Err(Refusal::Value(format!(
                    "`{path}` is `{}` of `{}`, the front's",
                    sibling.name, sibling.role
                )));
            }
        }
        if siblings.windows(2).any(|pair| pair[0].name >= pair[1].name) {
            return Err(Refusal::Value(
                "`siblings` are not one each, in the order of their names".to_owned(),
            ));
        }
        Ok(Self {
            directory,
            build,
            inventory,
            front,
            siblings,
        })
    }

    /// The directory the install placed its programs in, canonical when it was written.
    #[must_use]
    pub fn directory(&self) -> &str {
        &self.directory
    }

    /// The build the programs came from.
    #[must_use]
    pub const fn build(&self) -> &Build {
        &self.build
    }

    /// The sha256 of the inventory the install read its digests from.
    #[must_use]
    pub const fn inventory(&self) -> Digest {
        self.inventory
    }

    /// The front, `archogen`.
    #[must_use]
    pub const fn front(&self) -> &Program {
        &self.front
    }

    /// Every other program placed, in the order of their names.
    #[must_use]
    pub fn siblings(&self) -> &[Program] {
        &self.siblings
    }

    /// The sibling named `name`, if the install placed one.
    #[must_use]
    pub fn sibling(&self, name: &str) -> Option<&Program> {
        self.siblings.iter().find(|program| program.name == name)
    }

    /// The manifest's one text: compact JSON, the members in this order, and a newline.
    #[must_use]
    pub fn write(&self) -> String {
        let value = object(vec![
            ("format", string(FORMAT)),
            ("directory", string(&self.directory)),
            (
                "build",
                object(vec![
                    ("commit", string(&self.build.commit)),
                    ("tree", string(&self.build.tree)),
                    ("rustc", string(&self.build.rustc)),
                    ("host", string(&self.build.host)),
                ]),
            ),
            ("inventory", string(&self.inventory.to_string())),
            ("front", program_value(&self.front)),
            (
                "siblings",
                Value::Array(self.siblings.iter().map(program_value).collect()),
            ),
        ]);
        let mut text = json::write(&value);
        text.push('\n');
        text
    }

    /// Read a manifest's text, at most `limit` bytes of it.
    pub fn read(bytes: &[u8], limit: usize) -> Result<Self, Refusal> {
        let text = bytes.strip_suffix(b"\n").ok_or(Refusal::NotCanonical)?;
        let value = json::read(text, limit).map_err(Refusal::Json)?;
        let [format, directory, build, inventory, front, siblings] = members(
            &value,
            "",
            &[
                "format",
                "directory",
                "build",
                "inventory",
                "front",
                "siblings",
            ],
        )?;
        let format = text_of(format, "format")?;
        if format != FORMAT {
            return Err(Refusal::Format(format.to_owned()));
        }
        let [commit, tree, rustc, host] =
            members(build, "build", &["commit", "tree", "rustc", "host"])?;
        let build = Build {
            commit: text_of(commit, "build.commit")?.to_owned(),
            tree: text_of(tree, "build.tree")?.to_owned(),
            rustc: text_of(rustc, "build.rustc")?.to_owned(),
            host: text_of(host, "build.host")?.to_owned(),
        };
        let Value::Array(siblings) = siblings else {
            return Err(Refusal::Shape("`siblings` is not an array".to_owned()));
        };
        let manifest = Self::new(
            text_of(directory, "directory")?.to_owned(),
            build,
            digest(inventory, "inventory")?,
            read_program(front, "front")?,
            siblings
                .iter()
                .enumerate()
                .map(|(index, item)| read_program(item, &format!("siblings[{index}]")))
                .collect::<Result<_, _>>()?,
        )?;
        if manifest.write().as_bytes() != bytes {
            return Err(Refusal::NotCanonical);
        }
        Ok(manifest)
    }
}

/// Lowercase ASCII letters, digits and `-`, beginning with a letter.
fn name(path: &str, text: &str) -> Result<(), Refusal> {
    let mut chars = text.chars();
    let first = chars.next().is_some_and(|c| c.is_ascii_lowercase());
    if first && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        Ok(())
    } else {
        Err(Refusal::Value(format!(
            "`{path}` is `{text}`, not a name of lowercase letters, digits and `-`"
        )))
    }
}

/// A git object id: 40 lowercase hexadecimal digits, or 64 in a repository that names its objects by sha256.
fn object_id(path: &str, text: &str) -> Result<(), Refusal> {
    let hex = text
        .bytes()
        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if hex && matches!(text.len(), 40 | 64) {
        Ok(())
    } else {
        Err(Refusal::Value(format!(
            "`{path}` is `{text}`, not an object id"
        )))
    }
}

/// One line of text, not empty: what a tool printed about itself.
fn line(path: &str, text: &str) -> Result<(), Refusal> {
    if text.is_empty() || text.chars().any(char::is_control) {
        Err(Refusal::Value(format!(
            "`{path}` is {text:?}, not one line of text"
        )))
    } else {
        Ok(())
    }
}

fn program(path: &str, program: &Program) -> Result<(), Refusal> {
    name(&format!("{path}.name"), &program.name)?;
    name(&format!("{path}.role"), &program.role)
}

fn object(members: Vec<(&str, Value)>) -> Value {
    Value::Object(
        members
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

fn string(text: &str) -> Value {
    Value::String(text.to_owned())
}

fn program_value(program: &Program) -> Value {
    object(vec![
        ("name", string(&program.name)),
        ("role", string(&program.role)),
        ("sha256", string(&program.sha256.to_string())),
    ])
}

/// The values of an object's members, which must be exactly `names`, in that order ([`json::members`]).
fn members<'a, const N: usize>(
    value: &'a Value,
    path: &str,
    names: &[&str; N],
) -> Result<[&'a Value; N], Refusal> {
    json::members(value, path, names).map_err(Refusal::Shape)
}

fn text_of<'a>(value: &'a Value, path: &str) -> Result<&'a str, Refusal> {
    value
        .as_str()
        .ok_or_else(|| Refusal::Shape(format!("`{path}` is not a string")))
}

fn digest(value: &Value, path: &str) -> Result<Digest, Refusal> {
    let text = text_of(value, path)?;
    Digest::parse(text).ok_or_else(|| {
        Refusal::Value(format!(
            "`{path}` is `{text}`, not `sha256:` and 64 lowercase hexadecimal digits"
        ))
    })
}

fn read_program(value: &Value, path: &str) -> Result<Program, Refusal> {
    let [name, role, sha256] = members(value, path, &["name", "role", "sha256"])?;
    Ok(Program {
        name: text_of(name, &format!("{path}.name"))?.to_owned(),
        role: text_of(role, &format!("{path}.role"))?.to_owned(),
        sha256: digest(sha256, &format!("{path}.sha256"))?,
    })
}
