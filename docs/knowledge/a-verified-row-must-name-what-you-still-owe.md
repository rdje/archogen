---
slug: a-verified-row-must-name-what-you-still-owe
answers:
  - "A vendor fixed the integration bug I reported — what do I have to change on my side?"
  - "My register says `verified`. Does that mean there is nothing left to do?"
  - "Half of a remedy landed in my own repository — how do I record that?"
  - "How do I verify a fix whose remedy is split between the vendor's tree and mine?"
type: knowledge
date: 2026-09-27
---

# A split remedy: measure which half carries the fix, and record what you still owe

## The question

You reported an integration defect. The vendor publishes a fix, and re-measuring shows the failure
gone — but only after you also changed something in **your** repository. What does `verified` mean
now, and who owns the half that landed on your side?

## The answer

**Measure the two halves separately, and make the verdict say which one carries it.** Then record
your half as a *standing requirement of this consumer*, in the register and in the file that
implements it — not as a step you happened to take once.

A `verified` row that does not name the consumer's own obligation reads as "nothing left to do",
and the next person to reproduce the setup on a clean clone hits the original failure with a green
register telling them it cannot happen.

## Why

Measured on this project, `2026-09-27`, re-measuring a Cargo workspace collision at a vendor's new
pin. The report had asked for an empty `[workspace]` table in **two** vendored manifests. At the new
revision:

| Manifest the documented route touches | Own boundary | Before the consumer's exclusion | After it |
| --- | --- | --- | --- |
| the integration example (what a consumer builds) | yes | resolves | resolves |
| the nested transitive package (what the vendor's bootstrap builds) | no | **collides**, `rc=101` | resolves |

So the vendor fixed one manifest at the source and answered the other with a documented consumer
step — `exclude = ["vendor/linkedspec"]` in the application's workspace root. Both halves were
needed; neither alone was sufficient; and only a before/after measurement could tell them apart.
Probing costs nothing here (`cargo metadata --no-deps --offline`: no build, no network, no write
into the vendored tree), which is what made separating the halves cheap enough to be worth doing.

⭐ **The part that is easy to lose:** the second manifest is a *transitive* dependency, so the
vendor's refusal to patch it is defensible — it is another project's file. That makes the consumer's
exclusion a permanent part of the supported route, not a workaround to be cleaned up later.

## How to apply

- **Split the measurement.** Run the instrument before and after your own side of the change, and
  freeze both runs. A single green run after the fact cannot show which half did the work.
- **Print the attribution in the verdict.** "Carried by both halves: 1 of 2 vendored manifests
  declare their own boundary, and the application root excludes the vendored tree" is a sentence a
  reader can act on; "PASS" is not.
- **Record your half where it lives and where it is read.** A comment in the manifest that carries
  it (naming the report and the reason), and a row in the register's vendor table, so a reader who
  never opens the manifest still learns the obligation exists.
- **Say what `verified` does not cover.** Resolution is not a build; a build is not a link; a
  documentation fix is not a behaviour fix. One green instrument per claim, and the claim named.
- **Do not patch the vendored tree to make a measurement pass.** If the only way to reproduce the
  fix is to edit a dependency, the vendor's supported route has changed and the historical
  reproducer belongs in the record as history — re-run it as an artifact, never as a remedy.
- Related: [[frozen-reproducers-measure-change-not-repair]] — why the verdict needs a second
  instrument at all, and [[pin-the-vendor-head-and-measure-the-delta]] — how to choose the revision
  you measure at.
