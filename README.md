# rusefs

A simple Rust grep-like CLI tool for searching your filesystem with regex.

## Example

Search for all JavaScript files by their extension (case-insensitively) with `functionName()` in them that are under 5MB while skipping all files and folders named `node_modules`.

```bash
rusefs -f ~/GitHub -n "(?i)\.js" -c "functionName\(\)" -e "node_modules" -s 5
```
