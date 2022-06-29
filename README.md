# perg
`perg` - a toy Rust implementation of `grep`

## Usage
```
    perg [OPTIONS] <PATTERN> [PATH]

ARGS:
    <PATTERN>    A regular expression used for the search
    <PATH>       A path to a file or directory to search for. If none is given, perg will search
                 for the <PATTERN> recursively in current directory

OPTIONS:
    -h, --help    Print help information
    -i            Matches case insensitively. (Disabled by default)
    -l            Print only the names of the files matching the provided criteria. (Disabled by
                  default)
    -S            Follow symbolic links when traversing the directories. (Disabled by default)
    -w            Only match whole words. (Disabled by default)
```


## Examples

### Basics

Find all occurrences of `foo` in `bar` file/directory:
```bash
$ perg foo bar
```

Use any regex for search, i.e. find all 4-digit numbers in current directory (`'.'` path is optional)
```bash
$ perg  "\d{4}"
```

### Handling piped input

`perg` also can detect whether it should handle data from stdin when piped to.
For example, this command:
```bash
$ echo "Hello world\nhello" | perg -i hello # case insensitive
```
will output:
```bash
<stdin>
Hello world
hello
```

