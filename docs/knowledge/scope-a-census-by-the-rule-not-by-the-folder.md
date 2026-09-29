---
slug: scope-a-census-by-the-rule-not-by-the-folder
answers:
  - "I grepped for every site that breaks a rule — how do I know I found them all?"
  - "My census excluded the documentation to drop prose — what else did it drop?"
  - "A leaf's census sized the defect — is that the population I fix?"
type: knowledge
date: 2026-09-29
---

# Scope a census by what the rule governs, not by the folder you expect the breach in

## The question

You grep for every place that breaks a rule, fix what you find, and write the count into the leaf. How do you
know the count is the population?

## The answer

**Scope the census by what the rule applies to, then filter by evidence, never by location.** The rule
"scratch stays on this volume" applies to every script this repository runs. The census that filed the leaf
searched `scripts/`, because that is where gates live, and found **8** sites in 6 files. A second census made
while working the leaf searched every tracked file, then dropped `docs/` to get rid of prose that mentions the
command, and still found only those 8. The gate written for the leaf read every tracked shell script, by
extension and wherever it lives, and found **18 in 15**. The ten it added were all under `docs/`: eight
vendor-feedback reproducers, and two probe scripts kept beside the leaves that wrote them.

Both censuses were plausible, and both excluded the missing sites for the same reason: the filter removed a
**place**, and code lives in places you file under "documentation".

## How to apply

- **Name the population from the rule.** Before running the grep, write down what the rule governs: "every
  tracked shell script and Rust source", not "the gates". Select that population by what the files are (their
  extension, `git ls-files`), not by where you expect them.
- **Filter noise by shape, not by path.** To drop prose, drop lines that are not code, such as full-line
  comments or Markdown files. A path filter drops whatever else lives on that path.
- **Treat a leaf's count as a hypothesis** ([[a-leafs-claims-about-the-repository-are-hypotheses]]). The
  number that sized the defect was taken before anyone looked hard. When the gate's own census disagrees,
  the gate is right, and the leaf records both numbers.
- **Make the census the gate.** A population derived on every run (`git ls-files -- '*.sh' '.githooks/*'`)
  also covers the file added tomorrow, which a count in a leaf never does
  ([[enumerate-the-population-from-the-specification]]).
