//! Provides the CLI option parser
//!
//! Used to parse the argv/config file into a struct that
//! the server can consume and use as configuration data.
extern crate docopt;

use docopt::Docopt;

static DEFAULT_UDP_PORT: u16 = 8125;
static DEFAULT_TCP_PORT: u16 = 8126;

static USAGE: &'static str = "
Usage: statsd [options]
       statsd --help

Options:
  -h, --help       Print help information.
  -p, --port       The UDP port to bind to. Defaults to 8125.
  -a, --admin-port The TCP port for the admin interface. Defaults to 8126.
  --console        Enable the console backend.
";

/// Holds the parsed command line arguments
struct Args {
    flag_port: u8,
    flag_admin_port: u8,
    flag_console: bool,
    flag_help: bool,
}

fn parse_args() -> Args {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit);
    args
}
