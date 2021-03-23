.PHONY: all
all:
	# Rust parts
	cargo build

	# Guest_c
	gcc -c -Wall -Werror -fpic guest_c/main.c -o target/debug/guest_c_main.o
	gcc -shared -o target/debug/libguest_c.so target/debug/guest_c_main.o

	# Guest_go
	go build -o target/debug/libguest_go.so -buildmode=c-shared guest_go/main.go

.PHONY: run
run: all
	target/debug/host
