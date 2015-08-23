extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::BTreeSet;

use regex::Regex;

fn main() {
	let re = Regex::new(r"^(\S+)\s+NS\s+\S+$").unwrap();
	let f = File::open("data/net.zone").unwrap();
	let mut domains: BTreeSet<String> = BTreeSet::new();
	let reader = BufReader::new(f);
	let mut line_number: usize = 0;

	for line in reader.lines() {
		line_number += 1;
		if line_number % 1000 == 0 {
			println!("Processing zonefile line {}", line_number);
		}

		match line {
			Ok(line) => {
				match re.captures(&line) {
					Some(captures) => {
						let domain = captures.at(1).unwrap();
						if !domains.insert(domain.to_string()) {
							//println!("Domain {} already in set", domain);
						}
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

	println!("Found {} unique domains in zonefile", domains.len());
}
