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
| [`an-oracle-is-independent-by-construction.md`](an-oracle-is-independent-by-construction.md) | How do I write an expected-output oracle that is actually independent of what it judges? |
| [`closing-a-leaf-whose-work-landed-elsewhere.md`](closing-a-leaf-whose-work-landed-elsewhere.md) | Another tree already built what my leaf describes — do I just delete the leaf? |
| [`prose-beside-data-goes-unenforced.md`](prose-beside-data-goes-unenforced.md) | Half my config is enforced and half is prose — how do I stop the prose rotting? |
