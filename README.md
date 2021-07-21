# mondae

An inverse daemon runner, for running things in the foreground.

When running a command in a separate terminal, that window often just
closes if the command terminates.  Running through `mondae` if will
instead ask if you want to restart the command or quit, and wait for
your input.  Since you may have lots of tabs/windows, it will also
prepend a big red cross to the window title on failure (or a check on
normal termination).

## Example:

```
gnome-terminal --tab -- mondae --name "my thing" -- my_thing --with args
```
