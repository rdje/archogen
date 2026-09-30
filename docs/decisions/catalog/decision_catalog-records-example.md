# Catalog records: the worked example of the hash grammar

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`). This is §3's worked example of
  [[decision_catalog-records]], moved out of it verbatim on `2026-09-30`: round 7's answers took that record past
  the per-file ceiling `README-ROUTES` holds for `docs/decisions/` (`README_POLICY.md`). The example's text still
  names that record in its `origin` strings, because those bytes are hash inputs and moving them would change
  every digest.

## The fact / decision

The digests below are normative: they are what `decision_catalog-records.md` §3's grammar gives for these bytes,
and `M2.7.3` must reproduce every one. Section numbers are that record's.

**Worked example.** Two records and three files, given here rather than tracked. They exercise every kind of
line except `file` lines of a reached set and `ledger` lines, which need a package or a ledger. They also
exercise an escaped string, a hexadecimal integer, a review line and lock lines. The example's target gives no
`TARGET_KIND`, so it would not load. It fixes bytes, not a catalog. The digests below were computed by an
independent implementation of this grammar, which first reproduced every digest of the previous revision, and
each changed hash input was checked with `shasum -a 256`, `sha256sum` and `openssl dgst -sha256`. `M2.7.3` must
reproduce every one.

The three files, with their exact bytes (each line ends in a line feed):

| Path | Content |
| --- | --- |
| `docs/example/model.txt` | `model` |
| `targets/example-target.env` | `TARGET_ID=example-target` |
| `targets/example-target.eadl` | `(platform)` |

```text
(catalog-record example.base
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
  (timing-model (version "0.1.0") (none "the costs of a machine are its devices'"))
  (review (facet behavior-model)
          (hash "sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813")
          (verdict production) (by independent-context "the worked example") (date "2026-09-30")
          (basis "the model file says one processor")))

(catalog-record example.timed
  (version "1.0.0")
  (catalog algorithms)
  (source (origin "the worked example of decision_catalog-records.md") (license "MIT OR Apache-2.0"))
  (maintainer M2)
  (depends (example.base "0.1"))
  (supersedes)
  (profiles rt-static-up-v1)
  (targets any)
  (preconditions)
  (guarantees "a switch costs what it costs")
  (implementation (version "1.0.0") (none "the example has no code"))
  (behavior-model (version "1.0.0") (sources) (describes) (facts))
  (timing-model (version "1.0.0") (sources) (measured-with) (facts)
    (costs (cost switch (target example-target) (value 0x28) (unit ns) (scope "one switch")
                 (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt)
                 (evidence assumed) (basis "chosen for the example")))))
```

`E` renders the cost on one line, with `0x28` as `40`:

```text
(cost switch (target example-target) (value 40) (unit ns) (scope "one switch") (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt) (evidence assumed) (basis "chosen for the example"))
```

`example.timed` names `(targets any)`, so its behavioral model has `target` lines for every target under
`targets/`, which in this example is `example-target` alone. Its timing model has them for the target its cost
names. Both models of both records carry a `contract` line. The file hashes:

- `model.txt`: `sha256:98ad61a25e3683b6adf2474b01bbe1c27de6aad2ce3a80ff4140fe473c14e691`;
- `.env`: `sha256:0061dd5cabefd87f90429ee144bc8b8d0824108bb668f86fc7b655311ebe08c0`;
- `.eadl`: `sha256:16cc827ff198a7e91963562dce366b4ad1c56a2f4c3d4d81d5d7b3da7f05a2c0`.

| Hash | `example.base` | `example.timed` |
| --- | --- | --- |
| own contract | `sha256:7d0e0beeb4a60b63ec582923f85b705d783f6a23a9981e1376d31bbc7cdaafe0` | `sha256:27d97e2d076776372efefbd88b7e9da223cdc9a1dde381bb0471dc1654f2f6de` |
| bound contract | `sha256:067674b7244594b9bd0f363b8b1eda35e23efd2b968356a317c41f75f9c6bee7` | `sha256:d12c172c744fa512aea123b9b5bcbf872230f25f369a12bd493a4c609c4da6ab` |
| own implementation | `sha256:8b294520470fcc0abd962849a5ece4b7ae461f499a96dc7e47c40cf5d57cc025` | `sha256:55bb9d366caa730ac832bcaa77718d63e3e63c3d5cead08df507b04a3be7fa05` |
| bound implementation | `sha256:00e05895b46d115c43659f54d862f60eec69289643db3fa8e7ed54082b860cfe` | `sha256:ea8fa872738441b53d900655865fc27aa1464beb2c99f21ef949755aeb78f304` |
| own behavior-model | `sha256:65dc42732194435388d0d7b717a23ad13dc4cc17dbdb77fca6ff117e5f394c29` | `sha256:f0c1fb43fa476b20c71de169a42d4531180c44a947dceb1051f899881e48ead4` |
| bound behavior-model | `sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813` | `sha256:63f5b5125e0c2170973a7a979cab7d8b169840095ae47c48a834b03d12de7e6d` |
| own timing-model | `sha256:7a181dae6939f076f6a2fdf1f664517607b46b41d0b571f204d8542920d17867` | `sha256:b21769048b5a0a484669bde510a487f610013fb2aa2c0c039734875dd7133460` |
| bound timing-model | `sha256:56d69b33f75c5d7e9373004a69a7a382d5033c73b181ef4d52ef30d4c2a1da50` | `sha256:f674a199d995429c5c03f748b80eab021c8870e8d096709a9e05894ef1245e53` |
| record | `sha256:66ca4a999f157f4efde58048ee4bca9a0518ce571a3538cf87289e30b5fa17ab` | `sha256:48a323e7f59e248413585129801d7a6eb68bb68b78848175629f0f4f40213b6a` |

For instance, the input of `example.timed`'s bound timing model is:

```text
archogen-catalog/1
bound timing-model example.timed
own sha256:b21769048b5a0a484669bde510a487f610013fb2aa2c0c039734875dd7133460
contract example.timed sha256:d12c172c744fa512aea123b9b5bcbf872230f25f369a12bd493a4c609c4da6ab
implementation example.base sha256:00e05895b46d115c43659f54d862f60eec69289643db3fa8e7ed54082b860cfe
implementation example.timed sha256:ea8fa872738441b53d900655865fc27aa1464beb2c99f21ef949755aeb78f304
target targets/example-target.eadl sha256:16cc827ff198a7e91963562dce366b4ad1c56a2f4c3d4d81d5d7b3da7f05a2c0
target targets/example-target.env sha256:0061dd5cabefd87f90429ee144bc8b8d0824108bb668f86fc7b655311ebe08c0
timing-model example.base sha256:56d69b33f75c5d7e9373004a69a7a382d5033c73b181ef4d52ef30d4c2a1da50
```

`example.base`'s lock lines, its review's included, are:

```text
example.base contract 0.1.0 sha256:7d0e0beeb4a60b63ec582923f85b705d783f6a23a9981e1376d31bbc7cdaafe0
example.base implementation 0.1.0 sha256:8b294520470fcc0abd962849a5ece4b7ae461f499a96dc7e47c40cf5d57cc025
example.base behavior-model 0.1.0 sha256:65dc42732194435388d0d7b717a23ad13dc4cc17dbdb77fca6ff117e5f394c29
example.base timing-model 0.1.0 sha256:7a181dae6939f076f6a2fdf1f664517607b46b41d0b571f204d8542920d17867
example.base review sha256:dd436a7114e51f9b4327c42f0114240a4bafdcfaede3415420aff27fee85c603 behavior-model production sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813
```

The review's `E`, which follows `archogen-catalog/1` and `review example.base` in its ledger hash's input, is one
line, with the hash in full:

```text
(review (facet behavior-model) (hash "sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813") (verdict production) (by independent-context "the worked example") (date "2026-09-30") (basis "the model file says one processor"))
```

`example.base`'s timing model has a forms hash (§5) of
`sha256:ee76ffe0061191590403b973d6360a4c9fc5d9d5de5f64561b95e705f2e1f2d3`, over:

```text
archogen-catalog/1
forms timing-model
(timing-model (none "the costs of a machine are its devices'"))
```

## Why

A worked example is the one part of a hash grammar that an implementation can be checked against without trusting
the grammar's prose. It is kept whole, and apart, so the design record can grow with its reviews without the
example's bytes being touched.

## How to apply

- A change to §3 that moves a hash input recomputes the example here, by an independent implementation and by at
  least two command-line tools, in the same change.
- `M2.7.3`'s tests reproduce every digest here before any other catalog test runs.
