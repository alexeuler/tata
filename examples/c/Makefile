CC=gcc
CFLAGS=-I.

make: main.c
	$(CC) -o target/main main.c ../../target/debug/libtata_core.a -lz -framework Security
	./target/main

