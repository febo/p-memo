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

There are three "version" included:
1. no program output, feature `xs` (extra small).
   ```
   Program PMemo11111111111111111111111111111111111111 invoke [1]
   Program PMemo11111111111111111111111111111111111111 consumed 22 of 1400000 compute units
   Program PMemo11111111111111111111111111111111111111 success
   ```
2. logs the memo message only, same as ASMEMO.
   ```
   Program PMemo11111111111111111111111111111111111111 invoke [1]
   Program log: why does spl memo use 36000 cus to print len 60 msg of ascii
   Program PMemo11111111111111111111111111111111111111 consumed 125 of 1400000 compute units
   Program PMemo11111111111111111111111111111111111111 success
   ```
3. same output as SPL Memo, feature `xl` (extra large).
   ```
   Program PMemo11111111111111111111111111111111111111 invoke [1]
   Program log: Signed by 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM
   Program log: Memo (len 60): "why does spl memo use 36000 cus to print len 60 msg of ascii"
   Program PMemo11111111111111111111111111111111111111 consumed 2320 of 1400000 compute units
   Program PMemo11111111111111111111111111111111111111 success
   ```

## Features

Program size: `1280` bytes

CU comsumption:

| \# signers | CU (`xs`) |  CU        | CU (`xl`)       | CU (SPL Memo) |
| ---------- | --------- | ---------- | --------------- | --------------|
| 0          | 4         | 108        | 426             | 4685          |
| 1          | 21        | 123        | 1957            | 16213         |
| 2          | 36        | 136        | 3476            | 28133         |

> [!NOTE]
> Using Solana CLI `v2.2.13`.

## Building

To build the programs from the root directory of the repository:
```bash
cargo build-sbf
```

You can enable the features `xs` or `xl` by using `--features <FEATURE>` in the build command.

## Testing

To run the tests:
```bash
cargo test-sbf
```

You can enable the features `xs` or `xl` by using `--features <FEATURE>` in the test command.

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
