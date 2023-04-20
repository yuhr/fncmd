```console
$ example-basic
? 2
error: The following required arguments were not provided:
    <GREETING>

USAGE:
    example-basic [OPTIONS] <GREETING> [NAME]

For more information try --help

```

```console
$ example-basic --help
example-basic 0.0.0
Prints greeting message

USAGE:
    example-basic [OPTIONS] <GREETING> [NAME]

ARGS:
    <GREETING>    Greeting message
    <NAME>        Name of someone to greet

OPTIONS:
        --bang       Whether to use “!” instead of “.” at the end of the message
    -h, --help       Print help information
    -V, --version    Print version information

```

```console
$ example-basic Hello World
Hello, World.

```

```console
$ example-basic Hello --bang
Hello!

```