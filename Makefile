.PHONY: build install lint test check clean

CARGO := cargo
build:
	${CARGO} build

install:
	${CARGO} install --path .

lint:
	${CARGO} clippy

test:
	${CARGO} test -- --nocapture

test-single:
	${CARGO} test ${TARGET} -- --nocapture	

check:
	${CARGO} check

clean:
	rm -rf target
