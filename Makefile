REPO=/opt/repo/pop

danglings:
	cargo watch -x check -x 'run --release -p gen -- danglings ${REPO} -o danglings.log'

ab:
	RUST_LOG=debug cargo watch -c -x check -x 'run --release -p gen -- assetbundle ${REPO}'

tui:
	RUST_LOG=debug cargo watch -c -x check -x 'run --release -p unitytui -- ${REPO}'
