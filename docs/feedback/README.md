# Outbound feedback to upstream projects

Reports archogen sends to projects it depends on. Each subdirectory is one recipient and holds
the report plus everything needed to reproduce it on the recipient's side — reproducer scripts,
input corpora, and frozen expected output.

The rule for anything in here: **no claim that was not executed.** A report that leaves this
repository carries archogen's name, so every observation in it is measured on a named revision
with a runnable reproducer, and anything we got wrong on first reading is corrected in the
report itself rather than quietly dropped.

| Recipient | Report | Subject |
| --- | --- | --- |
| LinkedSpec | [`linkedspec/FEEDBACK.md`](linkedspec/FEEDBACK.md) | the Rust backend and the shipped Lispish grammar |
