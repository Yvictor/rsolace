publish:
	cargo publish -p rsolace

publish-sys:
	cargo publish -p rsolace-sys

publish-dry-run:
	cargo publish -p rsolace-sys --dry-run
	cargo publish -p rsolace --dry-run
