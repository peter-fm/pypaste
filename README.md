# pypaste
A command line tool for formatting python code enabling trouble free pasting into a python interpreter.

![Screenshot](/img/screenshot.png?raw=true)

When you paste code into a python terminal, particularly code containing classes, it can complain about unexpected indents. This tool fixes that problem by parsing python code and returning nicely formatted code that will make your interpreter happy.


## Helix Python REPL example
The tool is designed to be used in conjunction with your favorite text editor providing trouble free support for a python REPL. The following is an example using [Helix](https://helix-editor.com). For more information about installing helix see [Installing Helix](https://docs.helix-editor.com/install.html)

### Installation

1. Add the following line to your ~/.bashrc, ~/.bash_profile or ~/.zshrc file. (You'll need `tmux` if you don't already have it installed.)

```
alias hxpython='tmux new-session -d -s helix-target; tmux send-keys -t helix-target ". ./.venv/bin/activate" C-m;tmux send-keys -t helix-target "python" C-m; tmux attach-session -t helix-target'
```

You will need to refresh your session.

`source ~/.bash_profile`


The `hxpython` alias creates a python prompt using the virtual environment of the directory you are in.  (You'll need to replace the `. ./.venv/bin/activate` part with `conda activate <environment-name>` if you use [anaconda](https://www.anaconda.com)).

2. Clone the respository and build the optimized release target. If you don't already have rust installed get cargo from [here](https://rustup.rs).
```
cd ~
git clone https://github.com/peter-fm/pypaste.git
cd pypaste
cargo build --release
```

3. Create a helix keyboard shortcut by adding the following to your `~/.config/helix/config.toml` file

```
[keys.normal."\\"]
space = [ ":pipe-to /home/<username>/pypaste/target/release/pypaste | tmux load-buffer - \\; paste-buffer -t helix-target:0"] 
```

### Usage

Using your favorite terminal (I use [wezTerm](https://wezfurlong.org/wezterm/index.html)). Open up two panes side-by-side and navigate both to your python project. In the right hand side type in `hxpython`. This should open up a python prompt. Then in the left pane open up a python file. 

To send lines from the file to your python REPL select them (e.g. with `x`) then type `\` + `space` and the code will be sent to your REPL having been correctly formatted by pypaste! (Note you must be in normal mode so press `;` first to get back to normal mode if selecting via visual mode).


### macOS Issues

In macOS, the above works for small amounts of code but sometimes you can face some weird buffer overflow issues if trying to send a large amount of code in one go. To overcome this you can use pypaste to break the code into smaller chunks and send them directly to the tmux target. For this you can specify the target (`-t`), the buffer size in bytes (`-b`) and the delay between chunks in milliseconds (`-d`). Below is an example of sending the code in 1024 byte chunks with a small 10 millisecond delay between chunks. You may need to try different values until it works.

```
[keys.normal."\\"]
space = [":pipe-to /Users/<username>/pypaste/target/release/pypaste -t helix-target:0 -b 1024 -d 10" ]
```
If piping is causing you issues as well, you can avoid it altogether by using the clipboard mode via the `-c` flag and changing the shortcut as follows:

```
[keys.normal."\\"]
space = [":clipboard-yank", ":run-shell-command /Users/<username>/pypaste/target/release/pypaste -t helix-target:0 -b 1024 -d 10 -c" ]
```

See `pypaste -help` for more information.
