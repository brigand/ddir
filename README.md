A simple tool for sorting files by arbitrary and flexible matching of dates/times in the
file name.

## Installation

If you have rust installed, you can install directly from cargo.

```
cargo install --force ddir
```

Other installation options may be added in the future.

## Usage

If you run `ddir` with no arguments, it defaults to the "latest" command, returning
the file name with the largest date/time/datetime.

For example, if you have a directory with log files, you can print the latest one like this:

```
cat "$(ddir)"
```

The commands "asc" and "desc" return a matching file name on each line, in ascending or
descending order.

The "debug" command prints the parsed dates along with the file name, to help identify
issues if not behaving as expected.

