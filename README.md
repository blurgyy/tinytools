# tinytools

A collection of tools that enhance your experience in shell.

## Installation

Installing with `cargo`:

```shell
$ cargo install tinytools
```

## Utilities

### `gr`

#### Description

Get the nearest git root above current working directory (if it exists).

#### Usage

Sometimes you want to quickly go to nearest git root of current project.  Add
this shell function to your shell's initialization script:

```shell
function cg() {
    local groot=$(tt gr 2>/dev/null)
    if [[ -n $groot ]]; then
        cd $groot
    fi
}
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

### `bak`

#### Description

Append a tilde \(\~\) to the names of given files/directories.

#### Usage

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
