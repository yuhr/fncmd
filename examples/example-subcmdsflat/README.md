```console
$ example-subcmdsflat
Top-level command is called.

```

```console
$ example-subcmdsflat --help
Usage: example-subcmdsflat[EXE] [COMMAND]

Commands:
  child  
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```console
$ example-subcmdsflat child
One-level-deep subcommand is called.

```

```console
$ example-subcmdsflat child grandchild
Two-level-deep subcommand is called.

```

```console
$ example-subcmdsflat orphan
? 2
error: unrecognized subcommand 'orphan'

Usage: example-subcmdsflat[EXE] [COMMAND]

For more information, try '--help'.

```

```console
$ example-subcmdsflat-orphan
Top-level orphan command is called.

```