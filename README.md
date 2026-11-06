# ASTRA
If you want battle-tested fuzzing in production, use AFL++ / LibAFL. If you want to understand how fuzzers work, tick and build your own, this repo is for you.

## Building and Installing
### Prerequisites
- Rust stable (`rustup` recommended)
- A UNIX environment (or WSL2)
### Build and Install
An install script will compile and deploy the compiler wrapper on your machine simply run the command:
- `$ ./utils/install.sh`

(sudo privileges are required)

## Running Astra
Astra fuzzes an external target binary (or harness) by repeatedly providing mutated inputs.
A typical run looks like this:
```bash
astra --target ./path/to/target_binary --input  ./corpus --output ./out -c 8 -- [args] @@ [args]
```

## Credits
Author: Salim LARGO (2ourc3)

## License
Licensed under Apache License, Version 2.0 
