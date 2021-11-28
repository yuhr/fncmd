@_:
	just --list

bump *ARGS:
	cargo release --workspace --sign-commit --no-tag --no-push --no-publish {{ARGS}}

release *ARGS:
	cargo release --workspace --sign-tag {{ARGS}}