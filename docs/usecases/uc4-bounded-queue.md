# `uc4-bounded-queue` — the refusal fixture

**Status:** rejection fixture. It must stay refused for as long as `rt-static-up-v1` is the
profile. **First gate:** M1.

A system whose tasks communicate through a bounded queue. The profile excludes it
(`general-ipc`), so the toolchain must refuse it — by name, with the reason, and with the
obligation admitting it would add.

## The required answer

```console
$ osgen check examples/bounded-queue/system.eadl --profile rt-static-up-v1
osgen: unsupported-profile: `general-ipc` is not admitted by profile `rt-static-up-v1`
  hint: general inter-task communication, including task-to-task queues, needs queue capacity,
        overflow semantics, and their response-time effects; a later profile amendment
```

Exit code `12`. Not a build with the queue silently replaced by a shared variable. Not a build
that succeeds and a report that quietly omits the blocking term.

## Why this case is not optional

§3.1 makes a promise: a request outside the profile "returns an unsupported-profile diagnostic
rather than silently reducing the requested guarantee". A promise nothing exercises is a
comment. This is the fixture that runs the refusal path.

It also guards a specific decay mode. The pressure to admit a queue will be real — it is the
first item in §12 M8+'s dependency order, and it will arrive attached to a use case someone
wants. The correct response is a **new named profile with its own analysis obligations**, at
which point this fixture is re-pointed at the old profile and a new refusal case is written for
the new one's boundary. The incorrect response is to widen `rt-static-up-v1` and keep the
evidence that was produced under its old assumptions.

## What a later profile owes

§12 M8+ names the obligations that come with bounded IPC: queue capacity, overflow semantics,
priority inversion, blocking and response-time effects. Until every one of them has an
analysis and a fixture, this case stays red by design.

## The related refusals

The same shape applies to every other profile exclusion — `application-mutexes`,
`dynamic-task-creation`, `multicore`, and the rest of the eighteen in
[`docs/profiles/rt-static-up-v1.md`](../profiles/rt-static-up-v1.md). `uc4` is the worked one;
the profile's exclusion table is the population it samples.
