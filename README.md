# sysnet

A tool to get system and network info of a Linux box written in Rust.


### Usage

```
λ ~ » sysnet --help
sysnet 0.1.0

USAGE:
    sysnet [OPTIONS]

OPTIONS:
    -a, --all          Print system and network info
    -c, --cpu          Print CPUs info
    -d, --disks        Print disks info
    -h, --help         Print help information
    -n, --network      Print network info
    -p, --processes    List all processes
    -s, --system       Print system info
    -V, --version      Print version information
```

### Installing 

From the git repo :
```
λ ~/dev/sysnet(main*) » cargo install --path .
  Installing sysnet v0.1.0 (/home/mathieu/dev/sysnet)
    Updating crates.io index
   Compiling sysnet v0.1.0 (/home/mathieu/dev/sysnet)
    Finished release [optimized] target(s) in 1.36s
   Replacing /home/mathieu/.cargo/bin/sysnet
    Replaced package `sysnet v0.1.0 (/home/mathieu/dev/sysnet)` with `sysnet v0.1.0 (/home/mathieu/dev/sysnet)` (executable `sysnet`)
```

From crates.io : `cargo install sysnet`.


Make sure `~/.cargo/bin/sysnet` is in your `$PATH`.
