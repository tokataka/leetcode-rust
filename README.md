# Leetcode Commandline Fetcher for Rust

This repository is a modified version of [Aylei’s original Leetcode Rust repository](https://github.com/aylei/leetcode-rust) with some modifications.

## Overview

This tool fetches Leetcode problems along with their provided test cases, allowing you to quickly start solving problems in Rust without manual setup.

## Usage

`.cargo/config.toml` defines `cargo leetcode` command for easy use.

- `cargo leetcode fetch <PROBLEM_ID>...`: Download one or more specific problems.
- `cargo leetcode archive <PROBLEM_ID>...`: Move problems to `solutions/src/archived`.
- `cargo leetcode daily`: Download today's daily problem.
- `cargo leetcode random`: Download randomly selected problem.

## Debug

### Using VSCode

- Launch the **Cargo debug test current problem** configuration by pressing `F5` in VSCode.
- This runs the following test command internally:
  `cargo test --lib -p solutions -- [attempting|archived]:{CURRENT_PROBLEM_FILE_NAME}`

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.78.0 and up.
However, `rust-toolchain.toml` overrides `toolchain.channel="1.79.0"` since Leetcode environment uses this specific version.[^1]

## Compatibility Issue with VSCode rust-analyzer

The built-in rust-analyzer binary bundled with the extension has dropped support for Rust toolchain version 1.79.0, which may lead to unexpected behavior when running this toolchain version.

### Workaround

To resolve this issue, you can install rust-analyzer locally and configure VSCode to use it:

1. **Install rust-analyzer locally:**

   Open your terminal and run:

   ```
   rustup component add rust-analyzer
   ```

2. **Update VSCode Settings:**

   Add the following configuration to your project's `.vscode/settings.json` file to point the extension to the locally installed rust-analyzer:

   ```jsonc
   {
     // ...
     "rust-analyzer.server.path": "${userHome}/.cargo/bin/rust-analyzer"
     // ...
   }
   ```

By following these steps, VSCode will use your locally installed rust-analyzer, bypassing the compatibility issue with Rust toolchain version 1.79.0.

[^1]: https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages
