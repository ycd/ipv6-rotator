mod rotator;

use std::process::exit;

use clap::{App, Arg};
use rotator::rotator::Rotator;

fn main() {
    // Set level filter to minimum to log everything.
    log::LevelFilter::to_level(&log::LevelFilter::Trace);

    let matches = App::new("rotator")
        .version("0.1")
        .author("Yagiz Degirmenci. <yagizcanilbey1903@gmail.com>")
        .about("Rotate IPv6 addresses")
        .arg(
            Arg::new("interface")
                .short('i')
                .long("interface")
                .value_name("INTERFACE")
                .about("Set a network device interface (Example: eth0, ens33)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("network")
                .short('n')
                .long("network")
                .value_name("ADDRESS")
                .about("Address prefix to rotate over. (Example: 2001:db8:0000:0000)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .about("Number of the addresses to be routed")
                .default_value("5")
                .takes_value(true),
        )
        .arg(
            Arg::new("block")
                .short('b')
                .long("block")
                .value_name("CIDR block")
                .about("block range to rotate (Example: 32, 48, 64)")
                .default_value("64")
                .takes_value(true),
        )
        .arg(
            Arg::new("time")
                .about("Rotate to a new IP in x seconds")
                .takes_value(true)
                .default_value("10"),
        )
        .get_matches();

    // interface is a device actually, used as device "internally".
    let device = matches.value_of("interface").unwrap();
    let time = matches
        .value_of("time")
        .unwrap()
        .parse::<u16>()
        .expect("time is not a valid integer");
    let count = matches
        .value_of("count")
        .unwrap()
        .parse::<u16>()
        .expect("count is not a valid integer");
    let block = match matches.value_of("block").unwrap().parse::<u8>() {
        Ok(b) => match b {
            32 => rotator::rotator::IpBlock::CIDR32,
            48 => rotator::rotator::IpBlock::CIDR48,
            64 => rotator::rotator::IpBlock::CIDR64,
            _ => {
                eprintln!("[ERROR] {} is an invalid CIDR block", b);
                exit(1)
            }
        },
        Err(why) => panic!("invalid CIDR block: {:?}", why),
    };
    let network = matches.value_of("network").unwrap();

    let mut rotator = Rotator::builder()
        .device(device)
        .network(network)
        .count(count)
        .block(block)
        .sleep_time(time)
        .build();

    println!("{:#?}", &rotator);
    println!("{:?}", &rotator.create_unique_ip());
    println!("{:?}", &rotator.create_unique_ip());
    println!("{:?}", &rotator.create_unique_ip());
}
