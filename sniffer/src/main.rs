use clap::Parser;
use std::io::{ self, Write };
use std::net::{ IpAddr, TcpStream };
use std::str::FromStr;
use std::sync::mpsc::{ Sender, channel };
use std::thread;

const MAX: u16 = 65535;

#[derive(Debug)]
enum Errors {
    InvalidIPAddress,
}

/// Sniffer CLI Program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// IP Address
    #[clap(short, long, value_parser)]
    ipaddress: String,
    /// Number of threads
    #[clap(short, long, value_parser, default_value_t = 4)]
    threads: u16,
}

fn main() -> Result<(), Errors> {
    let args = Args::parse();
    let ipaddr = match IpAddr::from_str(&args.ipaddress) {
        Ok(s) => s,
        Err(_) => return Err(Errors::InvalidIPAddress)
    };
    let num_threads = args.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, ipaddr, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
    Ok(())
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
