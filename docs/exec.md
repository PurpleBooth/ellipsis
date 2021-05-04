# Exec

This allows you to use other programs in your todo list to setup some of
your files

This might be `apt`, `brew` or something more esoteric.

## Simple command

Next we need to create the `ellipsis.yml` settings

``` yaml,file(path="ellipsis.yml")
---
todo:
- exec:
    command: bash
    args:
        - "-c"
        - 'echo "Hello, world!" > output.txt'
```

Then we run ellipsis

``` shell,script(name="exec-step-elipsis",expected_exit_code=0)
ellipsis
```

Now we can see that the link exists

``` shell,script(name="exec-step-see-new-file",expected_exit_code=0)
cat "output.txt"
```

``` text,verify(script_name="exec-step-see-new-file",stream=stdout)
Hello, world!
```
