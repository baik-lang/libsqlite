.PHONY: build clean
build:
	cargo build

clean:
	rm test.db
	#rm lib.inadl
	#rm -rf target Cargo.lock