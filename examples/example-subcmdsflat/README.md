```console
$ example-subcmdsflat
Top-level command is called.

```

```console
$ example-subcmdsflat --help
example-subcmdsflat 0.0.0

USAGE:
    example-subcmdsflat [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    child    
    help     Print this message or the help of the given subcommand(s)

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
error: Found argument 'orphan' which wasn't expected, or isn't valid in this context

USAGE:
    example-subcmdsflat [SUBCOMMAND]

For more information try --help

```

```console
$ example-subcmdsflat-orphan
Top-level orphan command is called.

```