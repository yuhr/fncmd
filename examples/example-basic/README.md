```console
$ example-basic
? 2
error: the following required arguments were not provided:
  <GREETING>

Usage: example-basic <GREETING> [NAME]

For more information, try '--help'.

```

```console
$ example-basic --help
Description of the command line tool

Usage: crate-name [OPTIONS] --foo <FOO>

Options:
  -f, --foo <FOO>  Argument foo
  -b, --bar <BAR>  Argument bar
  -h, --help       Print help
  -V, --version    Print version

```

```console
$ example-basic Hello World
Hello, World.

```

```console
$ example-basic Hello --bang
Hello!

```