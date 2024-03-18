PROGRAM = cli-tool

run: main
	./${PROGRAM}

build: main

clean:
	rm ${PROGRAM}

main: src/main.zig
	zig build
	mv zig-out/bin/${PROGRAM} .
	rm -rf zig-cache
	rm -rf zig-out
