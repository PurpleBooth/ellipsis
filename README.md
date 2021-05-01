# ellipsis

*ellipsis* is a dotfile manager.

## Usage

``` shell,script(name="help",expected_exit_code=0)
ellipsis --help
```

``` text,verify(script_name="help",stream=stdout)
ellipsis 0.3.0
Billie Thompson <billie@billiecodes.com>
Manage dotfiles

USAGE:
    ellipsis [FLAGS] [OPTIONS]

FLAGS:
        --dry-run    Print what would be done without making any changes
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>    The configuration file for the operations to perform [env: ELLIPSIS=]
                             [default: ellipsis.yml]
        --home <home>        Work on the basis that the home directory is at this path [env:
                             HOME=/home/your-home]
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/ellipsis/releases/latest) we build
for linux and mac (all x86\_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/ellipsis
```
