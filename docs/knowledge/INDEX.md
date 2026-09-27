# Knowledge — the retrievable layer, keyed by question

A lesson learned is only worth the effort if a future reader can *find* it. `DEV_NOTES.md`
is chronological and unsearchable by question; this directory is the promoted form: one
file per durable lesson, whose front matter names the **questions it answers**.

New entry: create `docs/knowledge/<slug>.md` with front matter at line 1 carrying an
`answers:` list, then add its row below. The `LESSON-PROMOTION` doctrine requires every new
dated lesson in `DEV_NOTES.md` to be promoted here (or explicitly declined in its leaf).

| Entry | Answers |
| --- | --- |
| [`doctrine-seams-vs-forking-a-check.md`](doctrine-seams-vs-forking-a-check.md) | A portable gate misfires on this repo's layout — do I edit the gate? |
| [`cross-tree-lockstep-and-commit-scope.md`](cross-tree-lockstep-and-commit-scope.md) | TASK-ACCEPTANCE refuses a tree I only added a note to — what do I do? |
| [`presence-checks-cannot-see-an-extra-key.md`](presence-checks-cannot-see-an-extra-key.md) | The suite is green and the output is wrong — what kind of assertion did I write? |
| [`an-oracle-is-independent-by-construction.md`](an-oracle-is-independent-by-construction.md) | How do I write an oracle that is independent of what it judges — and that nobody can quietly adjust? |
| [`closing-a-leaf-whose-work-landed-elsewhere.md`](closing-a-leaf-whose-work-landed-elsewhere.md) | Another tree already built what my leaf describes — do I just delete the leaf? |
| [`prose-beside-data-goes-unenforced.md`](prose-beside-data-goes-unenforced.md) | Half my config is enforced and half is prose — how do I stop the prose rotting? |
| [`verify-the-mutation-applied.md`](verify-the-mutation-applied.md) | I broke the code to prove the test catches it and it still passed — what now? |
| [`a-gate-is-only-as-sharp-as-its-fixtures.md`](a-gate-is-only-as-sharp-as-its-fixtures.md) | My acceptance gate is green — what class of error could it still be blind to? |
| [`make-the-rule-a-constructor-precondition.md`](make-the-rule-a-constructor-precondition.md) | A specification says "must not do X" — where does that belong in the code? |
| [`a-rule-only-in-the-prompt-is-enforced-nowhere.md`](a-rule-only-in-the-prompt-is-enforced-nowhere.md) | The operator's instructions forbid X — where does that rule belong so it survives the session? |
| [`pin-the-vendor-head-and-measure-the-delta.md`](pin-the-vendor-head-and-measure-the-delta.md) | A vendor's notice names a specific publication — do I pin that commit or their latest head? |
