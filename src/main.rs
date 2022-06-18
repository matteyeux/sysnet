//! # sysnet
//!
//! Rewrite of sysnet, one of my old projects written in C, but this time in Rust.

use clap::Parser;
use sysnet::Device;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Print system info
    #[clap(short, long)]
    system: bool,

    /// Print network info
    #[clap(short, long)]
    network: bool,

    /// Print disks info
    #[clap(short, long)]
    disks: bool,

    /// Print CPUs info
    #[clap(short, long)]
    cpu: bool,

    /// Print system and network info
    #[clap(short, long)]
    all: bool,

    /// List all processes
    #[clap(short, long)]
    processes: bool,
}

fn main() {
    let args = Args::parse();
    let mut dev = Device::new();

    // TODO : do something better, it's currently not possible to print help
    // when args.len < 1
    if args.system {
        dev.print_system_info(false);
    }

    if args.network {
        dev.print_network_info();
    }

    if args.disks {
        dev.print_disks_info();
    }

    if args.cpu {
        dev.print_cpus_info();
    }

    if args.all {
        dev.print_all_info();
    }

    if args.processes {
        dev.print_processes();
    }
}
