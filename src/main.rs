//! This is the core server of store.rs

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(custom_derive, plugin)]

#![plugin(serde_macros)]

extern crate docopt;

use docopt::Docopt;

use std::str::FromStr;
use std::process::exit;
use std::net::SocketAddr;

fn version() {
    println!("store-bin 0.1.0");
}

const USAGE: &'static str = "
In-memory tree based key value database

Usage:
  store-bin <ip> <port>
  store-bin (-h | --help)
  store-bin --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    if args.get_bool("--version") {
        version();
        exit(0);
    }

    let host = args.get_str("<ip>");
    let port = args.get_str("<port>");
    let addr = format!("{}:{}", host, port);
    let sock_addr = SocketAddr::from_str(&addr[..]).unwrap();

    println!("Listening on {}", sock_addr);
}
