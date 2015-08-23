# Rust Statsd Server

This is a statsd server implementation written in rust. It aims to be as
compatible as possible with etsy/statsd.


# Building the application

1. Clone the repository.
2. Run `make install`.
3. The generated binary will be in `dist/statsd`.


# Running tests

After cloning the repository you can run either the unit tests, and or the
integration tests. The integration tests require python 2.7 and that port 8125
be free on your system.

```
make test
```

Will run both the unit and integration tests.
