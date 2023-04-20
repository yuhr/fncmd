```console
$ example-subcmds
Top-level command is called.

```

```console
$ example-subcmds --help
example-subcmds 0.0.0

USAGE:
    example-subcmds [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    child    
    help     Print this message or the help of the given subcommand(s)

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
error: Found argument 'orphan' which wasn't expected, or isn't valid in this context

USAGE:
    example-subcmds [SUBCOMMAND]

For more information try --help

```

```console
$ example-subcmds-orphan
Top-level orphan command is called.

```