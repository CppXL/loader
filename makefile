gnuwindows:
	cargo build --target=x86_64-pc-windows-gnu

gnulinux:
	cargo build --target=x86_64-unknown-linux-gnu

release:
	cargo build --release --target=x86_64-pc-windows-gnu

musl:
	cargo build --target=x86_64-unknown-linux-musl