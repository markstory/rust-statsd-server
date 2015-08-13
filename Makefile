.PHONY: test

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


test:
	cargo test

test_integration: $(ENV)
	cargo build
	$(PYTEST) tests/
