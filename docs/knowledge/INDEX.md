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
| [`verify-the-mutation-applied.md`](verify-the-mutation-applied.md) | I broke the code to prove the test catches it and it still passed — what now? · My before→after census shows no difference — did nothing change, or did I record too little? |
| [`a-gate-is-only-as-sharp-as-its-fixtures.md`](a-gate-is-only-as-sharp-as-its-fixtures.md) | My acceptance gate is green — what class of error could it still be blind to? |
| [`make-the-rule-a-constructor-precondition.md`](make-the-rule-a-constructor-precondition.md) | A specification says "must not do X" — where does that belong in the code? |
| [`a-rule-only-in-the-prompt-is-enforced-nowhere.md`](a-rule-only-in-the-prompt-is-enforced-nowhere.md) | The operator's instructions forbid X — where does that rule belong so it survives the session? |
| [`pin-the-vendor-head-and-measure-the-delta.md`](pin-the-vendor-head-and-measure-the-delta.md) | A vendor's notice names a specific publication — do I pin that commit or their latest head? |
| [`frozen-reproducers-measure-change-not-repair.md`](frozen-reproducers-measure-change-not-repair.md) | Upstream says the bug I reported is fixed — how do I verify that instead of accepting it? |
| [`a-verified-row-must-name-what-you-still-owe.md`](a-verified-row-must-name-what-you-still-owe.md) | A vendor fixed the integration bug I reported — what do I have to change on my side? |
| [`prove-the-artifact-was-regenerated-not-just-present.md`](prove-the-artifact-was-regenerated-not-just-present.md) | I re-ran the build at a new pin and it passed — how do I know it did not reuse the old artifacts? |
| [`a-fix-that-adds-a-route-does-not-retire-the-old-one.md`](a-fix-that-adds-a-route-does-not-retire-the-old-one.md) | The vendor says it is fixed but my reproducer still reproduces — who is right? |
| [`a-moved-measurement-needs-a-census-of-its-copies.md`](a-moved-measurement-needs-a-census-of-its-copies.md) | I changed a number my tests measure — what else do I have to update? |
| [`a-leafs-claims-about-the-repository-are-hypotheses.md`](a-leafs-claims-about-the-repository-are-hypotheses.md) | My task leaf says the mechanism / arms / figure already exists — do I trust it? |
| [`enumerate-the-population-from-the-specification.md`](enumerate-the-population-from-the-specification.md) | My two implementations agree on every input I have — which input is neither of them ever given? |
| [`an-existence-census-cannot-see-a-discarded-result.md`](an-existence-census-cannot-see-a-discarded-result.md) | My gate proves every diagnostic code is stated and emitted — what could still be wrong? · The help text says the command does X and a library does X — how do I know the command calls it? |
