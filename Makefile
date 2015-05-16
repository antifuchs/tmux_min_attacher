target/debug/tmux_min_attacher: src/main.rs
	cargo build

target/release/tmux_min_attacher: src/main.rs
	cargo build --release

run: tmux_min_attacher
	cargo run

test:
	cargo test

clean:
	git clean -fdx

install: target/release/tmux_min_attacher
	cp $< ~/bin
