# pypaste
A command line tool for formatting python code enabling trouble free pasting into a python interpreter.

![Screenshot](/img/screenshot.png?raw=true)

When you paste code into a python terminal, particularly code containing classes, it can complain about unexpected indents. This tool fixes that problem by parsing python code and returning nicely formatted code that will make your interpreter happy.


## UV example usage
The tool is designed to be used in conjunction with your favorite text editor providing trouble free support for a python REPL. The following is an example using [Helix](https://helix-editor.com). For more information about installing helix see [Installing Helix](https://docs.helix-editor.com/install.html)

Suppose you have a uv python project which has a .venv python environment in your current working directory.

1. Add the following line to your ~/.bashrc, ~/.bash_profile or ~/.zshrc file. (You'll need `tmux` if you don't already have it installed.)

```
alias hxpython='tmux split-window -v; tmux rename-window helix-target; tmux send-keys -t helix-target ". ./.venv/bin/activate" C-m;tmux send-keys -t helix-target "python" C-m; tmux select-pane -t 0; activate;'
```

You will need to refresh your session.

`source ~/.bashrc`

The `hxpython` alias creates a python prompt using the virtual environment of the directory you are in.  (You'll need to replace the `. ./.venv/bin/activate` part with `conda activate <environment-name>` if you use [anaconda](https://www.anaconda.com)).

2. Build the optimized release target. If you don't already have rust installed get cargo from [here](https://rustup.rs).
```
cargo install --git https://github.com/peter-fm/pypaste
```
(make sure `~/.cargo/bin` is on your path)

3. Create a helix keyboard shortcut by adding the following to your `~/.config/helix/config.toml` file

```
[keys.normal."\\"]
space = [":pipe-to pypaste -t helix-target:0 -b 900 -d 10" ]
```

### How to use it

Using your favorite terminal (I use [ghostty](https://ghostty.org)).

1. Start a tmux session `> tmux`
2. Start your ide! `> hypython` 
3. In the top pane open a python file `> hx script.py`
4. To send lines from the file to your python REPL select them (e.g. with `x`) then type `\` + `space` and the code will be sent to your REPL having been correctly formatted by pypaste! (Note you must be in normal mode so press `;` first to get back to normal mode if selecting via visual mode).


### macOS Issues

In macOS, the above works for small amounts of code but sometimes you can face some weird buffer overflow issues if trying to send a large amount of code in one go. To overcome this you can use pypaste to break the code into smaller chunks and send them directly to the tmux target. For this you can specify the target (`-t`), the buffer size in bytes (`-b`) and the delay between chunks in milliseconds (`-d`). Below is an example of sending the code in 1024 byte chunks with a small 10 millisecond delay between chunks. You may need to try different values until it works.

```
[keys.normal."\\"]
space = [":pipe-to pypaste -t helix-target:0 -b 1024 -d 10" ]
```
If piping is causing you issues as well, you can avoid it altogether by using the clipboard mode via the `-c` flag and changing the shortcut as follows:

```
[keys.normal."\\"]
space = [":clipboard-yank", ":run-shell-command pypaste -t helix-target:0 -b 1024 -d 10 -c" ]
```

See `pypaste -help` for more information.
