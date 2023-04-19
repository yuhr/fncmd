@_:
	just --list

setup:
	chmod +x .githooks/*
	git config --local core.hooksPath .githooks

bump *ARGS:
	cargo release --workspace --sign-commit --no-tag --no-push --no-publish \
		$(convco version --bump) {{ARGS}}

release *ARGS:
	cargo release --workspace --sign-tag {{ARGS}}