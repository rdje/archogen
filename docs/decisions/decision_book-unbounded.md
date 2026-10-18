# The book grows as needed

- **Type:** `decision`
- **Date:** `2026-10-18`
- **Status:** `active`
- **Owner / source:** the director, `2026-10-18`: "The book is the only user-facing surface so no limit on his size"; implemented under leaf `M3.6.9.6.3.2.2.13.1`.

## The fact / decision

The [mdBook](../book/src/ledger.md#mdbook) has no growth limits: no total or per-file size, chapter count, file count, line count,
line width, or other growth ceiling. It must be as large as the project needs to explain its behavior,
examples, contracts, evidence and limitations fully. This supersedes the book ceilings in
[the layering decision](decision_book-in-layers.md); that decision's reading layers remain binding.

## Why

The book is the director's only view of the project. A documentation ceiling obstructs completeness
and creates unnecessary compaction work whenever implementation needs a fuller account.

## How to apply

`README_POLICY.md` registers every book growth dimension as `unbounded`. `README-ROUTES` requires
that value for book routes, including nested routes, and refuses it for other destinations.
Do not shorten explanations, abbreviate navigation labels, split the book, or defer documentation
to satisfy a growth ceiling. Organize chapters and annexes for readers and the subject matter.

Keep the accuracy, glossary, generated index, anchor, coverage, source, figure and build checks.
Other repository surfaces retain their own policies.
