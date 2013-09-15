# TMUX min attacher - reattach to numbered tmux sessions

I launch tmux as my login shell from the terminal by calling just
`tmux` - by default, this creates one session per window. This is
usually what I want, except when I accidentally hit Cmd-W and close a
terminal window.

Opening a new window then causes a new session to be created, instead
of the accidentally-detached session to be re-attached. This was
getting pretty annoying to me, so I wrote this program.

## Usage

1. Get Rust 0.8
2. (optionally) check the PATH that
   [this](https://github.com/antifuchs/tmux_min_attacher/blob/master/tmux_min_attacher.rs#L42-L48)
   sets - setting PATH from your shell init files won't cut it here.
3. `make install`
4. Set your terminal to run the `tmux_min_attacher` binary as the
   command for new windows.
5. Open and close tabs/windows and be happy

## Disclaimer

I've tested this with a 0.8pre release of Rust. Interfaces are not
super stable, and this even uses FFI, so things are likely to break in
surprising amusing ways. I hope they don't, though!
