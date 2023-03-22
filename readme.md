# Findall

Recursively search for files which contain all listed patterns.

```
findall 'PATTERN...' [PATH]
```

If no PATH is provided, defaults to the current working directory.
Patterns are a single space separated argument (so if you have multiple patterns, you'll need to surround them by quotes, eg `findall 'foo bar baz' some_directory`
