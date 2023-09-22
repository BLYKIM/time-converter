mod argument;

use argument::{datetime_to_nanos, timestamp_to_utc};
use std::{env, process::exit};

fn main() {
    let Some(output) = parse() else {
        eprintln!("Need [timestamp/datetime] [value]");
        exit(1);
    };
    println!("{output}");
}

fn parse() -> Option<String> {
    #[allow(unused_assignments)]
    let mut output = String::new();

    let mut args = env::args();
    args.next()?;

    let arg = args.next()?;
    if arg == "timestamp" {
        let Ok(ts) = args.next()?.parse::<i64>() else {
            eprintln!("It is not a timestamp");
            exit(1);
        };
        output = format!("{}", timestamp_to_utc(ts).format("%Y-%m-%dT%H:%M:%S%.f%:z"));
    } else if arg == "datetime" {
        output = match datetime_to_nanos(&args.next()?) {
            Ok(timestamp) => timestamp.to_string(),
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }
    } else {
        eprintln!("[timestamp/datetime] value");
        exit(1);
    }

    Some(output)
}
