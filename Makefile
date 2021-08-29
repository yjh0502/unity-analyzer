danglings:
	cargo watch -x check -x 'run --release -p gen -- danglings /home/ubuntu/pop-tmp -o danglings.log'

tui:
	RUST_LOG=debug cargo watch -c -x check -x 'run --release -p unitytui -- /home/ubuntu/pop-tmp'
