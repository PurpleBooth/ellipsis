# Copy

With all operations source and destination are relative to current
working directory, and you may use "\~" as a shorthand for the location
of your home directory.

Copy operations can be used to copy files from your dotfiles to another
location.

## Into Existing directories

Here we copy a file from one location to another. To do this we first
need a file to copy

``` shell,script(name="copy-step-create-hello-world",expected_exit_code=0)
echo "Hello, world!" > source.txt
```

Next we need to create the `ellipsis.yml` settings

``` yaml,file(path="ellipsis.yml")
---
todo:
- copy:
    from: source.txt
    to: ~/destination.txt
```

Then we run ellipsis

``` shell,script(name="copy-step-elipsis",expected_exit_code=0)
ellipsis
```

``` shell,script(name="copy-step-see-new-file",expected_exit_code=0)
cat "$HOME/destination.txt" 
```

``` text,verify(script_name="copy-step-see-new-file",stream=stdout)
Hello, world!
```

## Overwriting existing files

If we subsequently make changes to this file, and rerun ellipsis

``` shell,script(name="copy-step-change-to-destination",expected_exit_code=0)
echo "Some New Content" > "$HOME/destination.txt" 
ellipsis
```

Those changes will be lost

``` shell,script(name="copy-step-see-new-file",expected_exit_code=0)
cat "$HOME/destination.txt" 
```

``` text,verify(script_name="copy-step-see-new-file",stream=stdout)
Hello, world!
```

## Missing containing directories

If the directory the file is to be placed within doesn't exist, we will
create it

``` yaml,file(path="ellipsis.yml")
---
todo:
- copy:
    from: source.txt
    to: ~/some/deep/file/location/destination.txt
```

``` shell,script(name="copy-step-input-deep-file",expected_exit_code=0)
echo "Hello, world!" > source.txt
```

``` shell,script(name="copy-step-deep-file-run",expected_exit_code=0)
ellipsis
```

``` shell,script(name="copy-step-deep-link",expected_exit_code=0)
cat "$HOME/some/deep/file/location/destination.txt" 
```

``` text,verify(script_name="copy-step-deep-link",stream=stdout)
Hello, world!
```
