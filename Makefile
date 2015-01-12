target/tmux_min_attacher: src/main.rs
	cargo build

run: tmux_min_attacher
	cargo run

test:
	cargo test

clean:
	git clean -fdx

install: target/tmux_min_attacher
	cp target/tmux_min_attacher ~/bin
