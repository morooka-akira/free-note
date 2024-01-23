use chrono::{DateTime, Local, TimeZone};
use clap::{Arg, Command};
#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use std::mem::zeroed;

        use libc::{settimeofday, suseconds_t, time_t, timeval, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

fn main() {
    let command = Command::new("clock")
        .version("0.1")
        .about("Gets and (aspirationally) sets the time")
        .arg(
            Arg::new("action")
                .value_parser(["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::new("std")
                .short('s')
                .long("standard")
                .value_parser(["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(
            Arg::new("datetime").help("When <action> is 'set', apply <datetime> Otherwise, ignore"),
        );
    let args = command.get_matches();

    let action = args.get_one::<String>("action").unwrap();
    let std = args.get_one::<String>("std").unwrap();

    if action == "set" {
        let t_ = args.get_one::<String>("datetime").unwrap();

        let parser = match std.as_str() {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };
        let err_msg = format!("Unable to parse: {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);

        Clock::set(t);
    }
    let now = Clock::get();
    match std.as_str() {
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        "timestamp" => println!("{}", now.timestamp()),
        _ => unreachable!(),
    }
}
