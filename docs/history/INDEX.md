# docs/history/INDEX.md — the sealed segments of the rolling ledgers

Each segment below is a run of a live ledger's oldest entries, moved here byte for byte by
`bash scripts/check_history_ledgers.sh --seal` and never edited again (`docs/decisions/decision_history-ledgers.md`).
A row records the segment's entry count, its newest and oldest entries, its lines, bytes and sha256, and the day it
was sealed. The rows are append-only, and `HISTORY-LEDGERS` checks every segment against its row on every commit.

To read a ledger whole, read its live file, then its segments from the highest number down: the concatenation is
the ledger as it would stand unsealed. To prove a segment, compare `sha256sum docs/history/<ledger>/<NNNN>.md` with
its row.

## `changelog`

| Segment | Entries | Newest | Oldest | Lines | Bytes | sha256 | Sealed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0001` | 20 | ARCHOGEN-S0-0023 | ARCHOGEN-PROGRAM-0002 | 534 | 37750 | `7a00002bca6abfffae6833cd539755395f4e9fa5e71a51aeec3534c34ed529d5` | `2026-09-30` |
| `0002` | 20 | ARCHOGEN-LINKEDSPEC-0053 | ARCHOGEN-M1-0024 | 627 | 49032 | `784f56d088d43a44a0fde8b5e6a341ffd35ee25779930847c01a3f119694318b` | `2026-09-30` |
| `0003` | 20 | ARCHOGEN-API-0078 | ARCHOGEN-LINKEDSPEC-0054 | 850 | 72112 | `7a459c541b8ee3389e1fa5f5b4157ca26aac86a09d7fa1fb390a33eb5f4b99cb` | `2026-09-30` |
| `0004` | 20 | ARCHOGEN-M1-0109 | ARCHOGEN-M1-0080 | 918 | 82280 | `68ca2ee9f1157e7b90bb15dc7df74a7b1d49ff182be10ced3019304aefd8d587` | `2026-09-30` |
| `0005` | 20 | ARCHOGEN-PROGRAM-0136 | ARCHOGEN-PROGRAM-0111 | 320 | 23774 | `a7a5e3d0aa897c985b592fa868ff7d5c25bc08144148e5caa02511dbed554687` | `2026-09-30` |
| `0006` | 20 | ARCHOGEN-API-0158 | ARCHOGEN-PROGRAM-0137 | 251 | 16088 | `33efd9d9ae7c38340fb97bafb82576643db394c8c3ec2aa0650f39705aa58736` | `2026-09-30` |
| `0007` | 20 | ARCHOGEN-M2-0186 | ARCHOGEN-M1-0159 | 236 | 14811 | `318c8e0911f2b79dc5bc1c2065047086297284394bcc113bc76b206ff22d7f2a` | `2026-09-30` |
| `0008` | 20 | ARCHOGEN-PROGRAM-0207 | ARCHOGEN-M2-0187 | 304 | 20852 | `884604cbbbac3cbe6e92ad39b72ac5499352ab9f6ec4475b27c55532e9dadebe` | `2026-09-30` |
| `0009` | 20 | ARCHOGEN-M2-0227 | archogen — removing the project template's references is f | 289 | 19565 | `ee49731606175949515ac26ddb69f66bc4620e1ae38b422180f99d95f9350773` | `2026-09-30` |
| `0010` | 20 | ARCHOGEN-M2-0250 | ARCHOGEN-PROGRAM-0228 | 290 | 20386 | `aafcab4e456ff82f5ef573da698e4e2d95cc05ecb0323adc3db43083a77fd874` | `2026-10-01` |

## `dev-notes`

| Segment | Entries | Newest | Oldest | Lines | Bytes | sha256 | Sealed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0001` | 10 | 2026-09-13 | 2026-09-13 | 162 | 11571 | `5389a5a78e424788467eab02e6db034d1ca0ca0e4978a73c92246083c82cb9a3` | `2026-09-30` |
| `0002` | 10 | 2026-09-27 | 2026-09-13 | 235 | 18835 | `7df1b95f0adc2f5826b54ec87861a6ad89c2d42aac20e54e24b5c39114b2b7f4` | `2026-09-30` |
| `0003` | 10 | 2026-09-27 | 2026-09-27 | 339 | 28638 | `ec4711680c571aae8c1fd948febe78ffe4ee3fc6ac99875ab001860eb287b67c` | `2026-09-30` |
| `0004` | 10 | 2026-09-29 | 2026-09-27 | 369 | 32716 | `56e78ea3ade53e9bdc472033e9eae9a3bd4c192625e07b2d826995a1b84970bf` | `2026-09-30` |
