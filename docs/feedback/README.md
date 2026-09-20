# Outbound feedback to upstream projects

Reports archogen sends to projects it depends on. Each subdirectory is a small, self-contained
**issue tracker** for one recipient: a status board, one file per bug with its own ID and state,
reproducer scripts, input corpora, and frozen expected output. The recipient needs nothing from
archogen to reproduce, and can respond by editing the state and history in the issue file.

The rule for anything in here: **no claim that was not executed.** A report that leaves this
repository carries archogen's name, so every observation in it is measured on a named revision
with a runnable reproducer, and anything we got wrong on first reading is corrected in the
report itself rather than quietly dropped.

| Recipient | Report | Subject |
| --- | --- | --- |
| LinkedSpec | [`linkedspec/`](linkedspec/README.md) | the Rust backend and the shipped Lispish grammar — 7 issues, `LS-001` … `LS-007` |
