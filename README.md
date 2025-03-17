<h1 align="center">
  <code>p-memo</code>
</h1>
<p align="center">
  <img width="400" alt="p-memo" src="https://github.com/user-attachments/assets/892da91c-71e8-4ed9-b3cc-b0b97f29ac2e" />
</p>
<p align="center">
  A <code>pinocchio</code>-based Memo program.
</p>

## Overview

A re-implementation of SPL Memo program using [`pinocchio`](https://github.com/anza-xyz/pinocchio) inspired by Cavey's [ASMEMO](https://x.com/cavemanloverboy/status/1898416863056384402) program.

There are two "version" included:
* Without additional logs (`logging` feature disabled)
* Same output as SPL Memo (`logging` feature enabled). CUs consumption increases in this case since the signer pubkeys are logged.

## Features

Program size: `1280` bytes

CU comsumption:

| \# signers |  CU |  CU (`+logging`) |
| ---------- | --- | ---------------- |
| 0          | 109 | 632              |
| 1          | 125 | 2320             |
| 2          | 141 | 3992             |

## Building

To build the programs from the root directory of the repository:
```bash
cargo build-sbf
```

## Testing

To run the tests:
```bash
cargo test-sbf
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
