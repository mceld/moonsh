# moonsh

funny shell

`cargo run` to build and run

`cargo build` to build

Add `moonsh` executable path to `/etc/shells` and `chsh` on Linux if you are brave.

### Config

Allocate some number of slices that each contain a stack of directories (built up as the user interacts with the shell).

`cd /somewhere/far/away/` will place this on the stack above the initial directory in that slice.

#### Config file format

```
slice /path/to/place/
slice ...
...
```

`slice` alone is also acceptable and will default to the `$HOME` environment variable.

The config file will be run as a sequence of commands to `moonsh`, you can provide commands other than `slice`.

### Interaction

Up key brings up last command from current slice (storing this many histories may be memory intensive?)

Down key moves down the history stack (if possible)

Ctrl-Up moves up the directory stack for the current slice

Ctrl-Down moves down the directory stack for the current slice

Ctrl-Left moves backward in the list of slices (with wrap-around)

Ctrl-Right moves forward in the list of slice (with wrap-around)

### New builtins

`slice [dir]` - Create a new slice with `dir` at the bottom.

`slices` - displays the contents of each slice for the current `moonsh` instance.

`reload <config_file>` - tear down and reload slices with those in `config_file` (provided the file exists).

`pop <n>` - pop `n` directories from the current slice's stack

- `pop a` will remove everything but the very bottom directory.

`pop`ping to an empty stack will fill in the bottom of the stack with the directory specified by `$HOME`.

### Basic wildcarding

Accepted wildcards

```
* - Kleene (anything for any number of characters, including 0)
? - Single (anything just once)
[abc] - Group (match any single unique character [interpreted literally] from 'abc')
```

Other characters are considered `Normal`.
