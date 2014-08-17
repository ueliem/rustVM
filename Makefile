build:
	mkdir -p build
	rustc src/rustvm.rs --out-dir=build/ --crate-type=lib
clean:
	rm -rf build/*
	rmdir build/
run:
	./build/rustVM
.PHONY: build
