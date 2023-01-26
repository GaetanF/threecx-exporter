.PHONY: all
all: clean build

package:
	apt update
	apt install -y pkg-config dh-cargo debhelper build-essential devscripts libssl-dev
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | RUSTUP_INIT_SKIP_PATH_CHECK=yes sh
	dpkg-buildpackage -us -uc

build:
	/root/.cargo/bin/cargo build --release
	mkdir -p debian/threecx-exporter/opt/threecx-exporter
	cp target/release/threecx-exporter debian/threecx-exporter/opt/threecx-exporter/

clean:
	/root/.cargo/bin/cargo clean