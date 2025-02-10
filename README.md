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
