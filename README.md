<h1 align="center">
  rusefs
</h1>

<p align="center">
  RUst SEarch FileSystem
</p>

---

A simple [Rust](https://www.rust-lang.org/) [grep](https://www.gnu.org/software/grep/)-like CLI tool for searching your filesystem with [regex](https://docs.rs/regex/1.5.4/regex/#syntax).

## Example

Search for all JavaScript files by their extension (case-insensitively) with `functionName()` in them that are under 5MB while skipping all files and folders named `node_modules`.

```bash
rusefs -f ~/GitHub -n "(?i)\.js" -c "functionName\(\)" -e "node_modules" -s 5
```

## Installation

Download the binary for your architechure from the [releases](https://github.com/Kyza/rusefs/releases), extract it, and place it somewhere in your PATH. Alternatively you can create an alias for it in your `.bashrc` or `.zshrc`.

A [`rusefs-config.toml`](https://github.com/Kyza/rusefs/blob/master/rusefs-config.toml) file can be created in the same folder as the binary to include default settings. The keys are the same as the long names for the CLI flags, run `rusefs --help` to find them.

## Building

Building this program requires [Rust](https://www.rust-lang.org/).

Once you've installed [Rust](https://www.rust-lang.org/), build the binary with the command below.

```bash
cargo build --release
```
