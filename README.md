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

# License

Copyright 2022, @hillcoder



Permission is hereby granted, free of charge, to any person obtaining a copy of this rspwd and associated documentation files, to deal in the rspwd without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the rspwd, and to permit persons to whom the rspwd is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the rspwd.

THE rspwd IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE rspwd OR THE USE OR OTHER DEALINGS IN THE rspwd.