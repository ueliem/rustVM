build:
	mkdir -p build
	rustc src/main.rs -o build/rustVM
clean:
	rm -rf build/*
	rmdir build/
run:
	./build/rustVM
.PHONY: build
