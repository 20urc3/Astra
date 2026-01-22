# ASTRA

**Astra** is an **educational fuzzer written in Rust**, built to explore how modern Unix-compatible fuzzers are designed internally (AFL++-style architecture, scheduling, coverage tracking, crash triage, and parallel workers).

This project focuses on **clarity and hackability** over completeness: it’s meant to be read, modified, and extended.

## Goals

- Provide a clean, understandable fuzzing architecture in **pure Rust**
- Support a **multi-worker** design (scales with cores)
- Implement the core mechanics:
  - corpus management
  - mutation loop
  - execution + timeouts
  - crash / hang detection
  - lightweight stats reporting

## Non-goals

- Being a drop-in replacement for AFL++ / libFuzzer
- Supporting every obscure target mode or platform edge-case
- Ultra-polished UX (yet)

If you want battle-tested fuzzing in production, use AFL++ / LibAFL.
If you want to understand how fuzzers tick and build your own, this repo is for you.

## Repository layout

The project is organized as a workspace:

```
.
├── crates/        # The actual fuzzer crates (engine, worker, shared types, etc.)
├── docs/          # Notes / design docs / write-ups
├── utils/         # Helper scripts and dev utilities
├── Cargo.toml     # Workspace manifest
└── Cargo.lock
```

> Most of the logic lives in `crates/`.

## High-level architecture

Astra is built around a **controller + workers** model:

### 1) Controller (main process)
Responsible for:
- loading seeds and managing the corpus
- distributing work to workers
- collecting results (coverage updates, crashes, hangs)
- printing statistics / progress

### 2) Workers (parallel fuzzing loop)
Each worker repeatedly:
1. picks a seed from the corpus
2. mutates it
3. executes the target
4. reports:
   - new coverage (interesting inputs)
   - crashes
   - timeouts / hangs

### 3) Shared state
Workers coordinate through lightweight shared state such as:
- a global coverage map / edge bitmap
- counters (execs/sec, total execs, etc.)
- channels/queues for events (crash/hang/interesting inputs)


## Quickstart
### Prerequisites

- Rust stable (`rustup` recommended)
- A UNIX environment (or WSL2)
### Build
```bash
cargo build --release
````


## Running Astra
Astra fuzzes an external target binary (or harness) by repeatedly providing mutated inputs.
A typical run looks like this:
```bash
# Example (replace with your real target)
./target/release/astra \
  --target ./path/to/target_binary \
  --input  ./corpus/seeds \
  --output ./out \
  --jobs   8
```

### Recommended directory structure

```bash
mkdir -p corpus/seeds
mkdir -p out
```
* `corpus/seeds/` contains your initial seed files
* `out/` will contain:

  * findings (crashes / hangs)
  * the evolving corpus
  * logs/stats (depending on build)

## Output

Astra typically produces:

* `out/crashes/` → unique crashing inputs
* `out/hangs/`   → timeout inputs
* `out/corpus/`  → minimized / evolved corpus
* live console stats (exec/s, edges, uptime, etc.)


## What “interesting” means (in Astra)
A mutated input is considered **interesting** when it increases global novelty, e.g.:
* discovers new edges / coverage bytes
* increases raw edge count or hits unseen transitions
* (optionally) triggers a new signal like a new path bucket
The exact policy is meant to stay readable and easy to tweak.

### Use a toy target

Start with something like:
* a file parser you control
* a tiny harness with a few branches
* a known-buggy sample project

## Documentation / write-ups

Check `docs/` for design notes and explanations.
This project is meant to be read alongside the code.

## Credits

Author: Salim LARGO (2ourc3)

Astra is inspired by decades of fuzzing research and tooling:
* AFL / AFL++
* libFuzzer
* LibAFL
If you’re new to fuzzing internals: read those projects’ docs, then come back here and implement the missing pieces yourself :)
