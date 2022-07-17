build-debug:
	@`flags cargo debug`

build-macos: target.nosync/x86_64-apple-darwin/release/visualizer target.nosync/aarch64-apple-darwin/release/visualizer
	@lipo -create -output binaries/bin-macos $^

build-linux-amd64: target.nosync/x86_64-unknown-linux-gnu/release/visualizer
	@mv $^ binaries/bin-linux-amd64

build-linux-arm64: target.nosync/aarch64-unknown-linux-gnu/release/visualizer
	@mv $^ binaries/bin-linux-arm64

build-linux-armhf: target.nosync/armv7-unknown-linux-gnueabihf/release/visualizer
	@mv $^ binaries/bin-linux-armhf

target.nosync/x86_64-apple-darwin/release/visualizer: Cargo.toml src/*.rs
	@`flags cargo macos x86_64`

target.nosync/aarch64-apple-darwin/release/visualizer: Cargo.toml src/*.rs
	@`flags cargo macos arm64`

target.nosync/x86_64-unknown-linux-gnu/release/visualizer: Cargo.toml src/*.rs
	@`flags cargo linux x86_64`

target.nosync/aarch64-unknown-linux-gnu/release/visualizer: Cargo.toml src/*.rs
	@`flags cargo linux arm64`

target.nosync/armv7-unknown-linux-gnueabihf/release/visualizer: Cargo.toml src/*.rs
	@`flags cargo linux armhf`