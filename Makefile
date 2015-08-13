.PHONY: test test_unit test_integration

ENV=env
PIP=$(ENV)/bin/pip
PYTEST=$(ENV)/bin/py.test

target/debug:
	mkdir -p target/debug

target/debug/statsd: target/debug src/*.rs
	cargo build

$(ENV):
	virtualenv $(ENV)
	$(PIP) install -r tests/requirements.txt


test: unit_test test_integration

test_unit:
	cargo test

test_integration: target/debug/statsd $(ENV)
	$(PYTEST) tests/
