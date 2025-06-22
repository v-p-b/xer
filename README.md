# xer

Command line byte encoding swiss army knife. The goal is to be the `iconv` of byte stream encodings.

Have you ever spent precious time converting something like `0xde, 0xad,\r\n0xbe, 0xef` to `\xde\xad\xbe\xef`? If so, then `xer` is for you.

## Usage

```
Usage: xer [OPTIONS]

Options:
  -f, --from <FROM>
          Input format specifier

          Possible values:
          - hex:     2-digit hex encoding, optionally separated by whitespace
          - escaped: \xHH encoding, no separator
          - c:       0xHH encoding, values separated with commas and whitespace
          - java:    0xHH encoding, with optional negative sign, values separated with commas and whitespace
          - bin:     0bBBBBBBBB encoded binary, values separated with commas and whitespace
          - dec:     Decimal
          - s-dec:   Signed Decimal
          - raw:     Raw bytes

  -t, --to <TO>
          Output format specifier

          Possible values:
          - hex:     2-digit hex encoding, optionally separated by whitespace
          - escaped: \xHH encoding, no separator
          - c:       0xHH encoding, values separated with commas and whitespace
          - java:    0xHH encoding, with optional negative sign, values separated with commas and whitespace
          - bin:     0bBBBBBBBB encoded binary, values separated with commas and whitespace
          - dec:     Decimal
          - s-dec:   Signed Decimal
          - raw:     Raw bytes

  -i, --input <INPUT>
          Input file (default: stdin)

  -o, --output <OUTPUT>
          Output file (default: stdout)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

If no `from` is specified the tool tries to auto-detect textual formats. To parse raw bytes you have to explicitly specify `-f raw`.

The default output format is C (e.g. `0xde,0xad,0xbe,0ef`). Output serialization aims to produce output that is easy to post-process:

- C delimiters are a single "," (no whitespace)
- Hex delimiters are a single space (so byte values can be easily split)

## TODO

* ~~Signed values (as you know, three billion devices run Java)~~
* More formats (PR's welcome)
* More tests

