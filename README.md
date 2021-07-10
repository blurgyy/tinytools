# tinytools

A collection of tools that enhance your experience in shell.  This provides an
executable named `tt`, which in turn provides the following utilities (more to
be added):

- [`bak`][bak]: Append a tilde \(\~\) to the names of given files/directories.
- [`debak`][debak]: Pop a tilde \(\~\) from the names of given files/directories.
- [`gr`][gr]: Get the nearest git root above current working directory (if it exists).

## Installation

Installing with `cargo`:

```shell
$ cargo install tinytools
```

## Utilities

### `bak`

Sometimes you want to quickly backup some file, this is typically done by
suffixing the file with a tilde \(\~\).  When there are multiple files to be
renamed, `bak` helps you to do this at ease:

```shell
$ alias bak="tt bak"
$ touch testfile
$ mkdir testdir
$ ls
testdir/  testfile
$ bak testdir testfile
renamed "<absolute path>/testfile" -> "<absolute path>/testfile~"
renamed "<absolute path>/testdir" -> "<absolute path>/testdir~"
```

`bak` aborts if the path after appending a tilde exists in the filesystem.

### `debak`

`debak` is the inverse of `bak` (see: [`bak`][bak]).  It does nothing (and
aborts) when any of the given path does not end with a tilde.

### `gr`

Sometimes you want to quickly go to nearest git root of current project.  Add
this shell alias to your shell's initialization script:

```shell
$ alias cg='cd ${$(tt gr 2>/dev/null):-$PWD}'
```

Then, call `cg` to quickly jump from your cwd to git root:

```shell
$ pwd
/home/r2d2/repos/tinytools/src/modules
$ cg
$ pwd
/home/r2d2/repos/tinytools
$ cd -
$ pwd
/home/r2d2/repos/tinytools/src/modules
```

[bak]: #user-content-bak
[debak]: #user-content-debak
[gr]: #user-content-gr
