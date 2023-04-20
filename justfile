@_:
	just --list

setup:
	chmod +x .githooks/*
	git config --local core.hooksPath .githooks

test *ARGS:
	for project in `ls examples`; do \
		RUST_BACKTRACE=1 cargo test --manifest-path examples/$project/Cargo.toml --workspace {{ARGS}} -- --nocapture; \
	done

bump *ARGS:
	cargo release --workspace --sign-commit --no-tag --no-push --no-publish \
		$(convco version --bump) {{ARGS}}

release *ARGS:
	cargo release --workspace --sign-tag {{ARGS}}