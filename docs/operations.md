# Operations

With all operations source and destination are relative to current
working directory, and you may use "\~" as a shorthand for the location
of your home directory.

## Copy

Copy operations can be used to copy files from your dotfiles to another
location.

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

## Link

Link operations can be used to create soft links files between the
location you desire.

Here we link a file from one location to another. To do this we first
need a file to link

``` shell,script(name="link-step-create-hello-world",expected_exit_code=0)
echo "Hello, world!" > source.txt
```

Next we need to create the `ellipsis.yml` settings

``` yaml,file(path="ellipsis.yml")
---
todo:
- link:
    from: source.txt
    to: ~/symlink.txt
```

Then we run ellipsis

``` shell,script(name="link-step-elipsis",expected_exit_code=0)
ellipsis
```

``` shell,script(name="link-step-see-new-file",expected_exit_code=0)
cat "$HOME/symlink.txt" 
```

``` text,verify(script_name="link-step-see-new-file",stream=stdout)
Hello, world!
```

``` shell,script(name="link-step-see-link",expected_exit_code=0)
if [[ -L "$HOME/symlink.txt" ]]; then
  echo "It's a link!"
fi
```

``` text,verify(script_name="link-step-see-link",stream=stdout)
It's a link!
```

If the link is a file already

``` shell,script(name="link-step-real-file",expected_exit_code=0)
rm "$HOME/symlink.txt"
echo "I am a real file" > "$HOME/symlink.txt"
```

``` shell,script(name="link-step-real-file-error",expected_exit_code=1)
ellipsis
```

then we error
