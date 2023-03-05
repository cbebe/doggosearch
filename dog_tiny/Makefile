EXE=doggosearch

run: target/release/$(EXE)
	@./$<

exe: target/release/$(EXE)

target/release/$(EXE):
	@cargo build --release --quiet
	@\cp ./target/release/build/$(EXE)-*/out/$(EXE) $@

clean:
	@cargo clean

.PHONY: run exe clean
