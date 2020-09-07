
bundle:
	cargo build --release --target x86_64-unknown-linux-musl && zip -j holiday-lambda.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

clean:
	rm holiday-lambda.zip; cargo clean
