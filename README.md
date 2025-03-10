<h1 align="center">
  <code>p-memo</code>
</h1>
<p align="center">
  <img width="400" alt="p-memo" src="https://github.com/user-attachments/assets/ba1c5f0d-db2f-457d-8f7e-e62fd564e5e7" />
</p>
<p align="center">
  A <code>pinocchio</code>-based Memo program.
</p>

# Overview

A re-implementation of SPL Memo program using [`pinocchio`](https://github.com/anza-xyz/pinocchio) inspired by Cavey's [ASMEMO](https://x.com/cavemanloverboy/status/1898416863056384402) program.

> [!NOTE]
> While the program provides the same functionality as SPL Memo, the output is simplified to reduce CUs by omitting some of the log messages.

# Features

Program size: `1280` bytes

CU comsumption:

| \# signers |  CU |
| ---------- | --- |
| 0          | 109 |
| 1          | 125 |
| 2          | 141 |

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
