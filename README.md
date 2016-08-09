# TMUX min attacher - reattach to numbered tmux sessions

I launch tmux as my login shell from the terminal by calling just
`tmux` - by default, this creates one session per window. This is
usually what I want, except when I accidentally hit Cmd-W and close a
terminal window.

Opening a new window then causes a new session to be created, instead
of the accidentally-detached session to be re-attached. This was
getting pretty annoying to me, so I wrote this program.

## Usage

1. Get Rust 1.10
2. (optionally) check the PATH that
   [this](https://github.com/antifuchs/tmux_min_attacher/blob/master/tmux_min_attacher.rs#L42-L48)
   sets - setting PATH from your shell init files won't cut it here.
2. (optionally) `make test`
3. `make install`
4. Set your terminal to run the `tmux_min_attacher` binary as the
   command for new windows.
5. Open and close tabs/windows and be happy

## But but but... rust!?

"You could do this from the shell!" - Yep, and I did, for the longest
time. Sadly, however, my shell startup is slow enough that this
significantly delayed startup of new shells, and what's worse, caused
stuff I'd typed ahead of the shell startup to be missed. Hence,
Rust. This is fast enough, let me tell you. (No missed typeahead, at
least!)

## Disclaimer

I've tested this with the 1.10 release of Rust. It uses
[nix](https://github.com/nix-rust/nix) to get access to
stable&safe-ish unix interfaces for `exec` and such, so should be
pretty stable - I've been using this code for a few months now. Let me
know in the
[Issues](https://github.com/antifuchs/tmux_min_attacher/issues) if
anything breaks for you. Or, wow, even better, submit a pull request!
(:
