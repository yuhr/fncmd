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
Prints greeting message

Usage: example-basic [OPTIONS] <GREETING> [NAME]

Arguments:
  <GREETING>  Greeting message
  [NAME]      Name of someone to greet

Options:
      --bang     Whether to use “!” instead of “.” at the end of the message
  -h, --help     Print help
  -V, --version  Print version

```

```console
$ example-basic Hello World
Hello, World.

```

```console
$ example-basic Hello --bang
Hello!

```