use chrono::{Utc, TimeZone, Local, DateTime};
use clap::{arg, Command};

const TIMESTAMP_NANO_LEN: usize = 19;
const TIMESTAMP_SECOND_LEN: usize = 10;

fn main() {
    let matches = Command::new("time-converter")
        .arg(arg!([TIMESTAMP])).get_matches();

    match matches.value_of("TIMESTAMP") {
        Some(n) => {
            parse_number(n);
        },
        None => {
            println!("pass timestamp only");
        }
    }
}

// timestamp nano 1000000000000000555
// unix timestamp 1653405628XXXXXXXXX
// RFC 3339 format string should be printed
fn parse_number(n: &str) {

    // Inferring the input using length will work until ~ 2285 year
    let (secs, nsecs) = match n {
        t if n.len() == TIMESTAMP_NANO_LEN => {
            // TODO(young): handle error
            let t = t.parse::<i64>().unwrap();
            let secs = t / 1_000_000_000;
            let nsecs = t - secs;

            (secs, nsecs)
        },
        t if n.len() == TIMESTAMP_SECOND_LEN => {
            let t = t.parse::<i64>().unwrap();

            (t, 0)
        }
        _ => {
            println!("input is not length of {} or {}", TIMESTAMP_NANO_LEN, TIMESTAMP_SECOND_LEN);
            return;
        }
    };

    let t_utc = Utc.timestamp(secs, nsecs as u32);
    let t_local: DateTime<Local> = t_utc.into();

    println!("{}", t_utc.to_rfc3339());
    println!("{}", t_local.to_rfc3339());   
}
