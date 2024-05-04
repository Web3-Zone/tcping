# Tcping


## Install

```
cargo build --release
```

## Usage

```
./target/release/tcping
Usage: tcping <host> [port] [timeout] [count]
```

## Demo

```
$ ./target/release/tcping google.com
64.233.180.113:443   seq= 1 rtt=3 ms
64.233.180.138:443   seq= 2 rtt=2 ms
64.233.180.113:443   seq= 3 rtt=1 ms
64.233.180.139:443   seq= 4 rtt=1 ms
64.233.180.102:443   seq= 5 rtt=2 ms
64.233.180.101:443   seq= 6 rtt=2 ms
64.233.180.100:443   seq= 7 rtt=2 ms
64.233.180.138:443   seq= 8 rtt=1 ms
64.233.180.113:443   seq= 9 rtt=2 ms
64.233.180.139:443   seq=10 rtt=1 ms
64.233.180.102:443   seq=11 rtt=2 ms
64.233.180.101:443   seq=12 rtt=1 ms
64.233.180.100:443   seq=13 rtt=2 ms
64.233.180.138:443   seq=14 rtt=1 ms
64.233.180.113:443   seq=15 rtt=1 ms
64.233.180.139:443   seq=16 rtt=1 ms
Min : 1 ms
Max : 3 ms
Avg : 1.56 ms
```
