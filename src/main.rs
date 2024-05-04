use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

const DEFAULT_PORT: u16 = 443;
const DEFAULT_TIMEOUT: u64 = 3;
const DEFAULT_COUNT: u64 = 16;

struct Args {
    host: String,
    port: u16,
    timeout: u64,
    count: u64,
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: tcping <host> [port] [timeout] [count]");
        std::process::exit(1);
    }

    let host = args[0].clone();
    let port = if args.len() > 1 {
        args[1].parse().unwrap_or(DEFAULT_PORT)
    } else {
        DEFAULT_PORT
    };
    let timeout = if args.len() > 2 {
        args[2].parse().unwrap_or(DEFAULT_TIMEOUT)
    } else {
        DEFAULT_TIMEOUT
    };
    let count = if args.len() > 3 {
        args[3].parse().unwrap_or(DEFAULT_COUNT)
    } else {
        DEFAULT_COUNT
    };

    Args {
        host,
        port,
        timeout,
        count,
    }
}

fn main() {
    let args = parse_args();
    let timeout = Duration::from_secs(args.timeout);
    let mut rtts = Vec::with_capacity(args.count as usize);
    let mut seq = 0;

    for _ in 0..args.count {
        seq += 1;
        let start = Instant::now();
        let addr = (args.host.as_str(), args.port).to_socket_addrs().unwrap().next().unwrap();
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(_stream) => {
                let duration = start.elapsed();
                let rtt = duration.as_millis() as u64;
                rtts.push(rtt);
                println!("{:20} seq={:2} rtt={} ms", addr, seq, rtt);
            }
            Err(_) => {
                println!("FAIL");
                break;
            }
        }
    }

    if !rtts.is_empty() {
        let min_rtt = rtts.iter().min().unwrap();
        let max_rtt = rtts.iter().max().unwrap();
        let avg_rtt = rtts.iter().sum::<u64>() as f64 / rtts.len() as f64;
        let mut variance = 0.0;
        for rtt in &rtts {
            variance += (*rtt as f64 - avg_rtt).powi(2);
        }
        variance /= rtts.len() as f64;

        println!("Min : {:.2} ms", min_rtt);
        println!("Max : {:.2} ms", max_rtt);
        println!("Avg : {:.2} ms", avg_rtt);
        println!("Mdev: {:.2} ms", variance);
    }
}
