TESTFILES = sample/add.txt sample/fundec.txt sample/fib.txt sample/odd-even.txt

test: $(TESTFILES)
	for i in $(TESTFILES); do \
	cargo run -- $$i; \
	done
