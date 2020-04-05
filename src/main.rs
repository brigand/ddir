use chrono::NaiveDateTime;
use std::cmp::Ordering;
use std::env;
use std::fs::read_dir;

mod parser;

use parser::parse_file_name;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Entry {
    name: String,
    date: NaiveDateTime,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date
            .cmp(&other.date)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_listing() -> Vec<Entry> {
    read_dir("./")
        .expect("Must be able to read directory")
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let date = parse_file_name(&name).ok()?;
            Some(Entry { name, date })
        })
        .collect()
}

fn main() {
    let cmd = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "latest".into());

    match cmd.as_str() {
        "debug" => {
            let mut list = get_listing();
            list.sort();

            for item in list {
                println!("date: {}, name: {}", item.date, item.name);
            }
        }
        "latest" => {
            let mut list = get_listing();
            list.sort();

            match list.into_iter().last() {
                Some(entry) => {
                    print!("{}", entry.name);
                }
                None => {
                    eprintln!("No files matched.");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command \"{}\".", cmd);
            eprintln!("Try \"latest\" (default)");
        }
    }
}
