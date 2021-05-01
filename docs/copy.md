# Copy

Copy operations can be used to copy files from your dotfiles to another
location. The source and destination are relative to current working
directory, and you may use "\~" as a shorthand for the location of your
home directory.

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

``` text,verify(script_name="copy-step-elipsis",stream=stdout)
Done!
```

``` shell,script(name="copy-step-see-new-file",expected_exit_code=0)
cat "$HOME/destination.txt" 
```

``` text,verify(script_name="copy-step-see-new-file",stream=stdout)
Hello, world!
```
