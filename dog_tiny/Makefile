EXE=dog_tiny

run: target/release/$(EXE)
	@./$<

exe: target/release/$(EXE)

target/release/$(EXE):
	@cargo build --release --quiet
	@\cp ./target/release/build/$(EXE)-*/out/$(EXE) $@

.PHONY: run exe
