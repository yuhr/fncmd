@_:
	just --list

setup:
	chmod +x .githooks/*
	git config --local core.hooksPath .githooks

test *ARGS:
	#!/usr/bin/env bash
	set -euxo pipefail
	RUST_BACKTRACE=1
	STATUS=0
	cargo test --workspace {{ARGS}} || STATUS=1
	for project in examples/example-*; do \
		cargo test --workspace --manifest-path $project/Cargo.toml --target-dir target {{ARGS}} -- --nocapture || STATUS=1; \
	done
	exit $STATUS

bump *ARGS:
	cargo release --workspace --sign-commit --no-tag --no-push --no-publish \
		$(convco version --bump) {{ARGS}}

release *ARGS:
	cargo release --workspace --sign-tag {{ARGS}}