mod birthday;

use chrono::{Datelike, NaiveDate, Utc};
use clap::{Arg, Command};
use ics::ICalendar;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::birthday::Birthday;

fn main() {
    let matches = Command::new("bd-ical-creator")
        .version("0.1.0")
        .author("<kontakt@der-fetzer.de>")
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Sets the output file to use")
                .required(true),
        )
        .get_matches();

    let input_file = File::open(matches.get_one::<String>("INPUT").unwrap()).unwrap();
    let lines = BufReader::new(input_file).lines();

    let now = Utc::now();
    let current_year = now.year();

    let mut calendar = ICalendar::new("2.0", "ics-rs");

    lines
        .map(|line| {
            let line = line.unwrap();
            let split_line: Vec<&str> = line.split(',').collect();
            if split_line.len() != 2 {
                panic!("Could not parse line: {}", line);
            }
            let date = NaiveDate::parse_from_str(split_line[1], "%d.%m.%Y")
                .or_else(|_| NaiveDate::parse_from_str(&format!("{}0", split_line[1]), "%d.%m.%Y"))
                .unwrap_or_else(|_| panic!("Could not parse line: {}", line));
            Birthday {
                name: split_line[0].trim().to_string(),
                date,
            }
        })
        .for_each(|birthday| {
            for year in current_year..current_year + 5 {
                let mut current_birthday = birthday.clone();
                current_birthday.date = current_birthday.date.with_year(year).unwrap();
                calendar.add_event(
                    birthday
                        .clone()
                        .into_event(year, now.format("%Y%m%dT%H%M%SZ").to_string()),
                );
            }
        });

    calendar
        .save_file(matches.get_one::<String>("OUTPUT").unwrap())
        .unwrap();
}
