extern crate regex;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use regex::Regex;

fn main() {
	let re = Regex::new(r"^(\S+)\s+NS\s+\S+$").unwrap();
	let f = File::open("data/net.zone").unwrap();
	let mut reader = BufReader::new(f);
	for line in reader.lines() {
		match line {
			Ok(line) => {
				//println!("Got line: {}", line);
				match re.captures(&line) {
					Some(captures) => {
						//println!("Found entry for domain {}", captures.at(1).unwrap());
					},
					None => {
						println!("Non-standard line: {}", line);
					}
				}
			},
			Err(e) => {
				println!("Failed to read line: {}", e);
				break;
			}
		}
	}
}