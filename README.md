# tinytools

A collection of tools that enhance your experience in shell.

## Installation

With `cargo`:

```shell
$ cargo install tinytools
```

## Utilities

### `gr`

Get the nearest git root above current working directory (if it exists).

You want to quickly go to nearest git root of current project.  Use this shell
function:

```shell
function cg() {
    local groot=$(tt gr 2>/dev/null)
    if [[ -n $groot ]]; then
        cd $groot
    fi
}
```

### `bak`

Append a tilde (~) to the names of given files/directories.

```shell
$ alias bak="tt bak"
$
$ touch testfile
$ mkdir testdir
$ ls
testdir/  testfile
$ bak testdir testfile
renamed "<absolute path>/testfile" -> "<absolute path>/testfile~"
renamed "<absolute path>/testdir" -> "<absolute path>/testdir~"
```
