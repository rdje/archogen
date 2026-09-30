//! The catalog's SHA-256 against two oracles that are not its own code (leaf `M2.7.2`).
//!
//! 1. The standard's published examples: FIPS 180-4's one-block and two-block messages, the empty message, the
//!    896-bit message and one million `a`.
//! 2. The system's own tool, over every message length from 0 to 129 bytes and a few longer ones, written into
//!    `tests/fixtures/sha256_boundaries.txt` by `scripts/sha256_fixture.sh`. Those lengths put the padding's `0x80`
//!    byte and the 64-bit length at every offset of one block and across two, which is where a hand-written
//!    SHA-256 goes wrong.

use archogen_evidence::sha256::{Digest, Sha256};

/// FIPS 180-4's examples (the NIST example files for SHA-256), which the system's tool also reproduces.
const PUBLISHED: &[(&str, &str)] = &[
    (
        "abc",
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    ),
    (
        "",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ),
    (
        "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
    ),
    (
        "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
        "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1",
    ),
];

#[test]
fn the_published_examples_are_reproduced() {
    for (message, digest) in PUBLISHED {
        assert_eq!(
            Digest::of(message.as_bytes()).hex(),
            *digest,
            "the digest of {message:?}"
        );
    }
}

#[test]
fn one_million_a_is_reproduced() {
    let mut hasher = Sha256::new();
    for _ in 0..1000 {
        hasher.update(&[b'a'; 1000]);
    }
    assert_eq!(
        hasher.finish().hex(),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

/// The fixture's message of length `n`: byte `i` is `(7 × i + 3) mod 256` (`scripts/sha256_fixture.sh`).
fn message(n: usize) -> Vec<u8> {
    (0..n)
        .map(|i| u8::try_from((7 * i + 3) % 256).expect("reduced modulo 256"))
        .collect()
}

/// The fixture's rows. It is an oracle only if it covers the boundaries, so a missing length fails here.
fn boundaries() -> Vec<(usize, String)> {
    let text = include_str!("fixtures/sha256_boundaries.txt");
    let rows: Vec<(usize, String)> = text
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let (length, digest) = line
                .split_once(' ')
                .unwrap_or_else(|| panic!("`{line}` is not `<length> sha256:<digest>`"));
            let length = length
                .parse()
                .unwrap_or_else(|_| panic!("`{length}` is not a length"));
            (length, digest.to_string())
        })
        .collect();
    for n in 0..=129 {
        assert!(
            rows.iter().any(|(length, _)| *length == n),
            "the fixture has no row for length {n}; regenerate it with `bash scripts/sha256_fixture.sh`"
        );
    }
    rows
}

#[test]
fn every_boundary_length_agrees_with_the_system_tool() {
    for (n, digest) in boundaries() {
        assert_eq!(Digest::of(&message(n)).to_string(), digest, "length {n}");
    }
}

#[test]
fn feeding_in_pieces_gives_the_same_digest() {
    // Every split point of a message of four blocks, so a piece ends at each offset of a block.
    let whole = message(200);
    let expected = Digest::of(&whole);
    for split in 0..=whole.len() {
        let mut hasher = Sha256::new();
        hasher.update(&whole[..split]);
        hasher.update(&whole[split..]);
        assert_eq!(hasher.finish(), expected, "split at {split}");
    }
}

#[test]
fn the_written_form_has_one_spelling() {
    let digest = Digest::of(b"abc");
    let written = digest.to_string();
    assert_eq!(
        written,
        "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(Digest::parse(&written), Some(digest));
    for bad in [
        written.to_uppercase(),
        written.replace("sha256:", "SHA256:"),
        written.replace("sha256:", ""),
        format!("{written}0"),
        written[..written.len() - 1].to_string(),
        written.replace('a', "g"),
        written.replace('b', "B"),
    ] {
        assert_eq!(Digest::parse(&bad), None, "`{bad}` parsed");
    }
}
