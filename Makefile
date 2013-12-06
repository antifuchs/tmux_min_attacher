tmux_min_attacher: tmux_min_attacher.rs
	rustc tmux_min_attacher.rs

run: tmux_min_attacher
	./tmux_min_attacher

test: tmux_min_attacher-test
	./tmux_min_attacher-test
	
tmux_min_attacher-test: tmux_min_attacher.rs
	rustc --test -o tmux_min_attacher-test tmux_min_attacher.rs

clean:
	git clean -fdx
	
install: tmux_min_attacher
	cp tmux_min_attacher ~/bin