build:
	cargo build --release --target x86_64-unknown-linux-musl
	sam build

build-HelloWorldFunction: target/x86_64-unknown-linux-musl/release
	cp ./target/x86_64-unknown-linux-musl/release/hello-world $(ARTIFACTS_DIR)/bootstrap

build-GoodbyeFunction: target/x86_64-unknown-linux-musl/release
	cp ./target/x86_64-unknown-linux-musl/release/goodbye $(ARTIFACTS_DIR)/bootstrap