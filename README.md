<p align="center">
    <img alt="ellipsis" src="./logo/logo.png">
</p>

*ellipsis* is a dotfile manager.

## Usage

``` shell,script(name="help",expected_exit_code=0)
ellipsis --help
```

``` text,verify(script_name="help",stream=stdout)
ellipsis 0.6.24
Billie Thompson <billie@billiecodes.com>
Manage dotfiles

USAGE:
    ellipsis [OPTIONS]

OPTIONS:
    -c, --config <config>    The configuration file for the operations to perform [env: ELLIPSIS=]
                             [default: ellipsis.yml]
        --dry-run            Print what would be done without making any changes
    -h, --help               Print help information
        --home <home>        Work on the basis that the home directory is at this path [env:
                             HOME=/home/your-home]
    -V, --version            Print version information
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/ellipsis/releases/latest) we build
for linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/ellipsis
```

## Docs

### Execution modes

These are things you can do to change how ellipsis will run

-   [--dry-run](./docs/dry-run.md)

### Operations

These are types you can put in your "todo" list

-   [copy](./docs/copy.md)
-   [link](./docs/link.md)
-   [exec](./docs/exec.md)
