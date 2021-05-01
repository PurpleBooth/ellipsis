# Dry Run

You can see what would happen with your current config by passing the dry run flag

Next we need to create the `ellipsis.yml` settings

``` yaml,file(path="ellipsis.yml")
---
todo:
- copy:
    from: source.txt
    to: ~/destination.txt
- link:
    from: source.txt
    to: ~/link.txt
```

Then we run ellipsis

``` shell,script(name="copy-step-elipsis",expected_exit_code=0)
HOME="/home/your-home" ellipsis --dry-run | sed "s#: .*/demo#: \"/demo#g"
```

``` text,verify(script_name="copy-step-elipsis",stream=stdout)
copy: "/demo/source.txt" -> "/home/your-home/destination.txt"
link: "/demo/source.txt" -> "/home/your-home/link.txt"
```

And no actual changes will be made