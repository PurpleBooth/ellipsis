# Link

With all operations source and destination are relative to current
working directory, and you may use "\~" as a shorthand for the location
of your home directory.

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

then we get an error, unless we set the overwrite flag

``` yaml,file(path="ellipsis.yml")
---
todo:
- link:
    from: source.txt
    to: ~/symlink.txt
    overwrite: true
```

Then we can run ellipsis

``` shell,script(name="link-step-overwrite",expected_exit_code=0)
ellipsis
```

``` shell,script(name="link-step-check-file-after-overwrite",expected_exit_code=0)
cat "$HOME/symlink.txt" 
```

``` text,verify(script_name="link-step-check-file-after-overwrite",stream=stdout)
Hello, world!
```

``` shell,script(name="link-step-see-link-after-overwrite",expected_exit_code=0)
if [[ -L "$HOME/symlink.txt" ]]; then
  echo "It's a link!"
else
  echo "It's not a link"
fi
```

``` text,verify(script_name="link-step-see-link-after-overwrite",stream=stdout)
It's a link!
```

If the directory the link is within doesn't exist, we will create it

``` yaml,file(path="ellipsis.yml")
---
todo:
- link:
    from: source.txt
    to: ~/some/deep/file/location/symlink.txt
```

``` shell,script(name="link-step-input-deep-file",expected_exit_code=0)
echo "Hello, world!" > source.txt
```

``` shell,script(name="link-step-deep-file-run",expected_exit_code=0)
ellipsis
```

``` shell,script(name="link-step-deep-link",expected_exit_code=0)
cat "$HOME/some/deep/file/location/symlink.txt" 
```

``` text,verify(script_name="link-step-deep-link",stream=stdout)
Hello, world!
```

``` shell,script(name="link-step-deep-link-see-link",expected_exit_code=0)
if [[ -L "$HOME/some/deep/file/location/symlink.txt" ]]; then
  echo "It's a link!"
else
  echo "It's not a link"
fi
```

``` text,verify(script_name="link-step-deep-link-see-link",stream=stdout)
It's a link!
```
