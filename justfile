@_:
	just --list

bump *ARGS:
	cargo release --workspace --no-tag --no-push --no-publish {{ARGS}}

release *ARGS:
	cargo release --workspace --sign-tag {{ARGS}}