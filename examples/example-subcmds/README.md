```console
$ example-subcmds
Top-level command is called.

```

```console
$ example-subcmds --help
Usage: example-subcmds [COMMAND]

Commands:
  child  
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```console
$ example-subcmds child
One-level-deep subcommand is called.

```

```console
$ example-subcmds child grandchild
Two-level-deep subcommand is called.

```

```console
$ example-subcmds orphan
? 2
error: unrecognized subcommand 'orphan'

Usage: example-subcmds [COMMAND]

For more information, try '--help'.

```

```console
$ example-subcmds-orphan
Top-level orphan command is called.

```