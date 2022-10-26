RSPWD
=====

Simple password generaton in Rust.

# Usage

```
Usage: rspwd [OPTIONS]

Commad line password generation.

Options:
  -u, --upper           use upper case
  -l, --lower           use lower case
  -n, --number          use number
  -s, --symbol          use symbol
  -p, --password-len    The number of chars. (default: 10)
  -h, --help            Show this help message.
```

Example:

```
./rspwd -u -l -n -p 40
mNQlAOFCjRgQWaQCbm6l3yNgKrtKluFsxEbO2Dez
```
