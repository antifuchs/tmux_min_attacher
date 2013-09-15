tmux_min_attacher: tmux_min_attacher.rs
	rustc tmux_min_attacher.rs

run:
	rust run tmux_min_attacher.rs

test:
	rust test tmux_min_attacher.rs
