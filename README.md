# rusefs

A simple [Rust](https://www.rust-lang.org/) [grep](https://www.gnu.org/software/grep/)-like CLI tool for searching your filesystem with [regex](https://docs.rs/regex/1.5.4/regex/#syntax).

## Example

Search for all JavaScript files by their extension (case-insensitively) with `functionName()` in them that are under 5MB while skipping all files and folders named `node_modules`.

```bash
rusefs -f ~/GitHub -n "(?i)\.js" -c "functionName\(\)" -e "node_modules" -s 5
```

## Installation

Download the binary for your architechure from the [releases](https://github.com/Kyza/rusefs/releases), extract it, and place it somewhere in your PATH.

A [`rusefs-config.toml`](https://github.com/Kyza/rusefs/blob/master/rusefs-config.toml) file can be created in the same folder as the binary to include default settings.

## Building

Building this program requires [Rust](https://www.rust-lang.org/).

Once you've installed [Rust](https://www.rust-lang.org/), build the binary with the command below.

```bash
cargo build --release
```
