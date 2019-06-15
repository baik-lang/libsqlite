.PHONY: build clean run
build: 
	gcc -fpic -c lib.c
	#gcc -pie -rdynamic -o test test.o
	gcc -rdynamic -Wall -std=c99 -I/usr/local/Cellar/sqlite/3.28.0/include/ -L/usr/local/Cellar/sqlite/3.28.0/lib/  -lsqlite3 -lpthread -ldl  -o lib.inadl lib.o 
	rm lib.o
clean:
	rm lib.inadl test.db
run:
	../../../build/baik lib.ina