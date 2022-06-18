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


### Examples

Print system info :
```
λ ~ » sysnet -s
Hosname: blackbird
OS : Ubuntu 21.10
kernel : 5.13.0-44-generic
Cores : 8
RAM
   Total RAM: 8299MB
   Free  RAM: 5277MB
   Used  RAM: 986MB
Swap
   Total Swap: 2147MB
   Free  Swap: 2147MB
   Used  Swap: 0MB
```

Print network info :
```
λ ~ » sysnet -n
lo
   127.0.0.1/8
   00:00:00:00:00:00

   ::1/128
   00:00:00:00:00:00
ens33
   192.168.121.128/24
   00:0c:31:b8:a9:f9

   fe80::e57c:b022:b151:4b8a/64
   00:af:29:bf:f9:c9
virbr0
   192.168.122.1/24
   51:52:00:f7:6b:cd
docker0
   172.17.0.1/16
   06:ff:91:ff:89:21
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
