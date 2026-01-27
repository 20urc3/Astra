# ASTRA

**Astra** is a fuzzer written in Rust, built to explore how modern Unix-compatible fuzzers are designed internally (AFL++-style architecture, scheduling, coverage tracking, crash triage, and parallel workers). This project focuses on **clarity and hackability** over completeness: it’s meant to be read, modified, and extended.

## Goals

- Provide a clean, understandable fuzzing architecture in **pure Rust**
- Support a **multi-worker** design (scales with cores)
- Implement the core mechanics:
  - corpus management
  - mutation loop
  - execution + timeouts
  - crash / hang detection
  - lightweight stats reporting

If you want battle-tested fuzzing in production, use AFL++ / LibAFL. If you want to understand how fuzzers tick and build your own, this repo is for you.

## Building and Installing
### Prerequisites
- Rust stable (`rustup` recommended)
- A UNIX environment (or WSL2)
### Build and Install
An install script will compile and deploy the compiler wrapper on your machine simply run the command:
- `$ ./utils/install.sh`
(sudo privilege are required)

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


## What “interesting” means in Astra
A mutated input is considered **interesting** when it increases global novelty, e.g.:
* discovers new edges / coverage bytes
* increases raw edge count or hits unseen transitions
* (optionally) triggers a new signal like a new path bucket
The exact policy is meant to stay readable and easy to tweak.
  
## Credits
Author: Salim LARGO (2ourc3)
