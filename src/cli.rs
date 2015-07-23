//! Provides the CLI option parser
//!
//! Used to parse the argv/config file into a struct that
//! the server can consume and use as configuration data.

use docopt::Docopt;

static DEFAULT_UDP_PORT: u16 = 8125;
static DEFAULT_TCP_PORT: u16 = 8126;

static USAGE: &'static str = "
Usage: statsd [options]
       statsd --help

Options:
  -h, --help            Print help information.
  -p, --port=<p>        The UDP port to bind to [default: 8125].
  -a, --admin-port=<p>  The TCP port for the admin interface [default: 8126].
  --console             Enable the console backend.
";

/// Holds the parsed command line arguments
#[derive(RustcDecodable, Debug)]
pub struct Args {
    flag_port: u16,
    flag_admin_port: u16,
    flag_console: bool,
    flag_help: bool,
}

pub fn parse_args() -> Args {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    args
}
