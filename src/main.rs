mod argument;

use argument::{datetime_to_nanos, format_datetime, timestamp_to_utc};
use chrono::Utc;
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

    match arg.as_str() {
        "timestamp" => {
            let Ok(ts) = args.next()?.parse::<i64>() else {
                eprintln!("It is not a timestamp");
                exit(1);
            };
            output = timestamp_to_utc(ts);
        }
        "datetime" => {
            output = match datetime_to_nanos(&args.next()?) {
                Ok(timestamp) => timestamp.to_string(),
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            }
        }
        "now" => output = format_datetime(Utc::now()),
        _ => {
            eprintln!("[timestamp/datetime/now] value");
            exit(1);
        }
    }
    // if arg == "timestamp" {
    //     let Ok(ts) = args.next()?.parse::<i64>() else {
    //         eprintln!("It is not a timestamp");
    //         exit(1);
    //     };
    //     output = timestamp_to_utc(ts);
    // } else if arg == "datetime" {
    //     output = match datetime_to_nanos(&args.next()?) {
    //         Ok(timestamp) => timestamp.to_string(),
    //         Err(e) => {
    //             eprintln!("{e}");
    //             exit(1);
    //         }
    //     }
    // } else {
    //     eprintln!("[timestamp/datetime/now] value");
    //     exit(1);
    // }

    Some(output)
}
