# Words this book uses

Embedded software is full of abbreviations, and this book uses its share. If a word stops you, it should be
here: every acronym the book uses, spelled out and explained in a sentence, then the ordinary-looking words that
mean something precise in this project. Each entry points to the chapter that treats the idea properly.

This page is kept true by a check, not by care. `scripts/check_book_glossary.sh` (the `BOOK-GLOSSARY` gate)
refuses a change whose chapters use an acronym this page does not define, or that leaves an entry here that no
chapter uses any more. Code, file names and the project's own identifiers — milestones such as `M2`, fixtures such
as `F26`, task-tree leaves — are names, not words, and are not listed.

## Acronyms and abbreviations

- **ABI** — Application Binary Interface: how compiled code calls other code at the machine level — which registers
  hold arguments, how the stack is laid out. See [The boundary](boundary.md).
- **ACLINT** — the RISC-V Advanced Core Local Interruptor: the specification for a hart's timer and software
  interrupts. See [What this project relies on from outside](ledger.md).
- **API** — Application Programming Interface: the functions a program offers other programs. archogen's is the
  engine API. See [The engine API](engine-api.md).
- **AUTOSAR** — AUTomotive Open System ARchitecture: the car industry's standard software architecture, whose
  operating-system part is quoted here as a precedent. See [its ledger entry](ledger.md#autosar-os).
- **BSD** — Berkeley Software Distribution: here, the flavour of the standard command-line tools macOS ships, which
  differ in small ways from the GNU ones. See [Verifying the toolchain](verification.md).
- **CI** — Continuous Integration: the automatic build and test run on every change, here GitHub's. See [Verifying
  the toolchain](verification.md).
- **CLI** — Command-Line Interface: a program driven by typed commands; here, `archogen` itself. See [The `archogen`
  command line](cli.md).
- **CLINT** — Core-Local Interruptor: the block that provides a RISC-V hart's timer and software interrupts on the
  emulated machine archogen targets first. See [Where generated systems run](targets.md).
- **CPU** — Central Processing Unit: the processor. See [What this project relies on from outside](ledger.md).
- **CSR** — Control and Status Register: a RISC-V register that configures or reports the processor's state, such as
  whether interrupts are enabled. See [What this project relies on from outside](ledger.md).
- **DMA** — Direct Memory Access: a device moving data to or from memory without the processor; excluded from the
  first profile. See [The supported profile](profile.md).
- **DSP** — Digital Signal Processor: a processor specialised for signal arithmetic. See [What this project relies
  on from outside](ledger.md).
- **eADL** — the description language archogen reads. The name comes from the project's charter, and this
  repository never spells it out. See [The boundary](boundary.md) and [Reading a description](reading.md).
- **EBNF** — Extended Backus–Naur Form: a notation for writing down a language's grammar. See [Verifying the
  toolchain](verification.md).
- **EIP** — External Interrupt Pending: the [PLIC specification](ledger.md#riscv-plic)'s name for the pending bit it
  raises at a hart.
- **GB**, **KB**, **MB** — gigabyte, kilobyte, megabyte, in this book's prose; the language's own units are the
  binary `KiB` and `MiB`. See [Quantities and units](quantities.md).
- **GNU** — the GNU project ("GNU's Not Unix"): here, the flavour of the standard command-line tools a Linux runner
  has. See [Verifying the toolchain](verification.md).
- **GPG** — GNU Privacy Guard: the tool that checks a download's signature. See [What this project relies on from
  outside](ledger.md).
- **HTTP**, **HTTPS** — Hypertext Transfer Protocol, and its secure form: how a browser fetches a page. See [The
  engine API](engine-api.md).
- **ID** — identifier: a name that picks out exactly one thing. See [Where the engine's knowledge comes from: the
  catalog](catalog.md).
- **IPC** — Inter-Process Communication: programs exchanging messages; the first profile has none. See [The
  supported profile](profile.md).
- **ISA** — Instruction Set Architecture: the instructions a processor family understands, such as RISC-V. See [What
  this project relies on from outside](ledger.md).
- **ISR** — Interrupt Service Routine: the code that runs when an interrupt arrives; here usually called a service.
  See [What the scheduling checker establishes](analysis.md).
- **JSON** — JavaScript Object Notation: a plain-text format for structured data. See [The engine API](engine-api.md).
- **LCOFI** — Local Counter Overflow Interrupt: a RISC-V interrupt raised when a performance counter overflows. See
  [What this project relies on from outside](ledger.md).
- **MCP** — Model Context Protocol: the protocol through which an AI agent calls a tool; archogen's server speaks it.
  See [The engine API](engine-api.md) and [the specification's ledger entry](ledger.md#mcp-specification).
- **MEI**, **MSI**, **MTI** — Machine External, Software and Timer Interrupt: RISC-V's interrupts for its most
  privileged mode, where archogen's systems run. See [What this project relies on from outside](ledger.md).
- **MMIO** — Memory-Mapped Input/Output: a device controlled by reading and writing memory addresses. See [Where
  generated systems run](targets.md).
- **MTIP** — Machine Timer Interrupt Pending: the bit that says a RISC-V hart's timer interrupt is waiting. See [What
  this project relies on from outside](ledger.md).
- **NUL** — the character whose code is zero. See [Reading a description](reading.md).
- **OS** — Operating System. See [The boundary](boundary.md).
- **OSEK**, **VDX** — OSEK/VDX, a German automotive standard for small real-time operating systems ("Offene Systeme
  und deren Schnittstellen für die Elektronik in Kraftfahrzeugen"), joined by VDX, "Vehicle Distributed eXecutive",
  from French carmakers; quoted here as a precedent. See [its ledger entry](ledger.md#osek-os).
- **PDF** — Portable Document Format. See [What this project relies on from outside](ledger.md).
- **PGEN** — a parser generator that [LinkedSpec](ledger.md#linkedspec)'s bootstrap uses: a name, not an acronym. See
  [its ledger entry](ledger.md#pgen).
- **PLIC** — Platform-Level Interrupt Controller: the RISC-V block that routes devices' interrupts to a hart. See
  [its specification's ledger entry](ledger.md#riscv-plic).
- **POSIX** — Portable Operating System Interface: the standard Unix-like interface; not the first profile's. See
  [The supported profile](profile.md).
- **QEMU** — the open-source machine emulator ("Quick EMUlator") that runs archogen's first target. See [Where
  generated systems run](targets.md) and [its ledger entry](ledger.md#qemu).
- **R24-11** — AUTOSAR's release of November 2024: a version, not an acronym. See [What this project relies on from
  outside](ledger.md).
- **RAM** — Random-Access Memory. See [Where generated systems run](targets.md).
- **README** — a repository's introductory file, meant to be read first. See [Verifying the toolchain](verification.md).
- **RED** — in "RED arm": a deliberately broken case a check must refuse, proving the check can fail (red as in a
  failing test). See [Verifying the toolchain](verification.md).
- **RGX** — a regular-expression engine nested in LinkedSpec: a name, not an acronym. See [its ledger
  entry](ledger.md#rgx).
- **RISC-V** — an open instruction set architecture, the fifth Reduced Instruction Set Computer design from
  Berkeley; archogen's first target. See [Where generated systems run](targets.md).
- **RV64** — 64-bit RISC-V. See [Reading a description](reading.md).
- **SEI**, **SSI**, **STI** — Supervisor External, Software and Timer Interrupt: RISC-V's interrupts for its
  supervisor mode, which archogen's first systems do not use. See [What this project relies on from
  outside](ledger.md).
- **SHA-256** — Secure Hash Algorithm with a 256-bit result: a fingerprint of a file, which changes if one byte
  does. See [What a report may claim](evidence.md).
- **SI** — the International System of Units (Système international). See [Quantities and units](quantities.md).
- **TL16C550C** — a serial-port chip of the 16550 family, the kind QEMU's machine emulates: a part number, not an
  acronym. See [What this project relies on from outside](ledger.md).
- **UART** — Universal Asynchronous Receiver-Transmitter: a serial port, the simplest way a small board talks. See
  [Where generated systems run](targets.md).
- **WCET** — Worst-Case Execution Time: the longest a piece of code can take, the number every timing promise rests
  on. See [Describing a workload](workload.md).
- **xRET** — RISC-V's return-from-trap instructions, `MRET` and `SRET`, the `x` standing for the mode. See [What this
  project relies on from outside](ledger.md).
- **YAML** — "YAML Ain't Markup Language": the text format CI workflows are written in. See [Verifying the
  toolchain](verification.md).

## Words with a meaning of their own here

- **catalog** — the reviewed building blocks a generated system is assembled from, each pinned by a hash of its
  content. See [Where the engine's knowledge comes from: the catalog](catalog.md).
- **deadline** — how long after its release a task's work must be finished. See [Describing a workload](workload.md).
- **description** — a text in eADL saying what a system must do, never how. See [Reading a description](reading.md).
- **fault** — something gone wrong that the runtime must answer: an overrun, a stack overflow, an unexpected trap or
  a failed check. See [The runtime](runtime.md).
- **halt** — stopping the whole system on a fault it cannot contain, keeping a record of the first one. See [The
  runtime](runtime.md).
- **interrupt** — a signal from a device or the timer that makes the processor drop what it is doing and run other
  code for a moment. See [The runtime](runtime.md).
- **job** — one round of a task's work, from one release to its completion. See [The runtime](runtime.md).
- **overrun** — a task released again while its previous job is still unfinished. See [The runtime](runtime.md).
- **period** — the time between a periodic task's releases. See [Describing a workload](workload.md).
- **port** — the part of a system that does the chip-specific work — saving registers, switching stacks — while the
  runtime decides. See [The runtime](runtime.md).
- **preemption** — a more urgent task taking the processor from a less urgent one, which resumes later. See [The
  runtime](runtime.md).
- **priority** — how urgent a task is; `1` is the most urgent. See [Describing a workload](workload.md).
- **profile** — the set of features and limits a description is checked against, such as `rt-static-up-v1`. See [The
  supported profile](profile.md).
- **release** — the moment a task's next round of work becomes due. See [Describing a workload](workload.md).
- **runtime** — the code inside every generated system that decides which task runs next and what to do when
  something goes wrong. See [The runtime](runtime.md).
- **task** — one piece of work a system does over and over, such as reading a sensor every 10 ms. See [Describing a
  workload](workload.md).
- **trap** — the processor stopping its current code to run the system's own, because of an interrupt or an
  exception such as a bad instruction. See [The runtime](runtime.md).
