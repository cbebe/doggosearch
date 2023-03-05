default:
	cd dog_bin && cargo build --release --quiet && cp ./target/release/dog_bin ../doggosearch

tiny:
	cd dog_tiny && $(MAKE) exe && cp ./target/release/dog_tiny ../doggosearch_tiny

clean:
	\rm -f doggosearch doggosearch_tiny
	cd dog_tiny && cargo clean
	cd dog_bin && cargo clean
	cd dog_lib && cargo clean

.PHONY: default tiny clean
