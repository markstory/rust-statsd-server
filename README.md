# Rust Statsd Server

[![Build Status](https://travis-ci.org/markstory/rust-statsd-server.svg?branch=master)](https://travis-ci.org/markstory/rust-statsd-server)

Forked from [markstory/rust-statsd-server](https://github.com/markstory/rust-statsd-server)

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

# Usage

The statsd server has several options to control which ports it runs on:

```
-p, --port=<p>        The UDP port to bind to [default: 8125].
--admin-host=<p>      The host to bind the management server on. [default: 127.0.0.1]
--admin-port=<p>      The port to bind the management server to. [default: 8126]
```

## Changing how frequently metrics are output

```
--flush-interval=<p>  How frequently to flush metrics to the backends in seconds. [default: 10].
```

On each flush interval event, derived metrics for timers are calculated. This
duration is tracked as `statsd.processing_time`. You can use this metric to
track how long statsd is spending generating derived metrics.

## Enabling the console or graphite backends

By default no backends are enabled. In this mode the statsd server doesn't do
that much. To enable one or both backends use the CLI flags:

```
--console             Enable the console backend.
--graphite            Enable the graphite backend.
```

The graphite backend has additional options for defining where graphite/carbon
runs:

```
--graphite-port=<p>   The port graphite/carbon is running on. [default: 2003].
--graphite-host=<p>   The host graphite/carbon is running on. [default: 127.0.0.1]
```

## Internal metrics

This server tracks a few internal metrics:

* `statsd.bad_messages` The number of invalid metrics that have been sent since
  the last flush.
* `statsd.total_messages` The number of messages received including invalid
  messages.
* `statsd.processing_time` How many ms were spent calculating derived metrics
  in the current flush cycle.


## Prior Art

I took a bunch of inspiration in how to implement and structure this
implementation from [erik/rust-statsd](https://github.com/erik/rust-statsd).
