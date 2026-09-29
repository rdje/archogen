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
| [`presence-checks-cannot-see-an-extra-key.md`](presence-checks-cannot-see-an-extra-key.md) | The suite is green and the output is wrong — what kind of assertion did I write? · My test asserts the diagnostic code is there — could the author still be reading a second, false one? |
| [`an-oracle-is-independent-by-construction.md`](an-oracle-is-independent-by-construction.md) | How do I write an oracle that is independent of what it judges — and that nobody can quietly adjust? |
| [`closing-a-leaf-whose-work-landed-elsewhere.md`](closing-a-leaf-whose-work-landed-elsewhere.md) | Another tree already built what my leaf describes — do I just delete the leaf? |
| [`prose-beside-data-goes-unenforced.md`](prose-beside-data-goes-unenforced.md) | Half my config is enforced and half is prose — how do I stop the prose rotting? |
| [`verify-the-mutation-applied.md`](verify-the-mutation-applied.md) | I broke the code to prove the test catches it and it still passed — what now? · My before→after census shows no difference — did nothing change, or did I record too little? · My census sized a defect — how do I know it measured the defect and not something containing it? · My harness says a mutation did not apply — is the file really untouched, and who restores it? |
| [`a-gate-is-only-as-sharp-as-its-fixtures.md`](a-gate-is-only-as-sharp-as-its-fixtures.md) | My acceptance gate is green — what class of error could it still be blind to? · Every RED arm of my gate passes — could the gate still be unable to fail on the real tree? · My new rule's fixtures went green — would they have gone green without the rule? · I moved a self-test's scratch directory and it still passes — is it still testing anything? · My test is named for the property it guards — does it contain a case where the property could fail? · My property run passes in a release build — could it be blind to an overflow? |
| [`make-the-rule-a-constructor-precondition.md`](make-the-rule-a-constructor-precondition.md) | A specification says "must not do X" — where does that belong in the code? |
| [`a-rule-only-in-the-prompt-is-enforced-nowhere.md`](a-rule-only-in-the-prompt-is-enforced-nowhere.md) | The operator's instructions forbid X — where does that rule belong so it survives the session? |
| [`pin-the-vendor-head-and-measure-the-delta.md`](pin-the-vendor-head-and-measure-the-delta.md) | A vendor's notice names a specific publication — do I pin that commit or their latest head? |
| [`frozen-reproducers-measure-change-not-repair.md`](frozen-reproducers-measure-change-not-repair.md) | Upstream says the bug I reported is fixed — how do I verify that instead of accepting it? |
| [`a-verified-row-must-name-what-you-still-owe.md`](a-verified-row-must-name-what-you-still-owe.md) | A vendor fixed the integration bug I reported — what do I have to change on my side? |
| [`prove-the-artifact-was-regenerated-not-just-present.md`](prove-the-artifact-was-regenerated-not-just-present.md) | I re-ran the build at a new pin and it passed — how do I know it did not reuse the old artifacts? |
| [`a-fix-that-adds-a-route-does-not-retire-the-old-one.md`](a-fix-that-adds-a-route-does-not-retire-the-old-one.md) | The vendor says it is fixed but my reproducer still reproduces — who is right? |
| [`a-moved-measurement-needs-a-census-of-its-copies.md`](a-moved-measurement-needs-a-census-of-its-copies.md) | I changed a number my tests measure — what else do I have to update? |
| [`a-leafs-claims-about-the-repository-are-hypotheses.md`](a-leafs-claims-about-the-repository-are-hypotheses.md) | My task leaf says the mechanism / arms / figure already exists — do I trust it? · I am about to exclude something from a check because it obviously cannot run there — do I measure first? |
| [`enumerate-the-population-from-the-specification.md`](enumerate-the-population-from-the-specification.md) | My two implementations agree on every input I have — which input is neither of them ever given? |
| [`an-existence-census-cannot-see-a-discarded-result.md`](an-existence-census-cannot-see-a-discarded-result.md) | My gate proves every diagnostic code is stated and emitted — what could still be wrong? · The help text says the command does X and a library does X — how do I know the command calls it? |
| [`scope-a-census-by-the-rule-not-by-the-folder.md`](scope-a-census-by-the-rule-not-by-the-folder.md) | I grepped for every site that breaks a rule — how do I know I found them all? · My census excluded the documentation to drop prose — what else did it drop? · A leaf's census sized the defect — is that the population I fix? |
| [`a-byte-offset-is-not-a-character.md`](a-byte-offset-is-not-a-character.md) | Where can my reader or renderer panic on text that is not ASCII? · My scanner advances one byte at a time — when is that safe? · Every test passes and the reader still crashes on a user's file — what did the tests never contain? |
