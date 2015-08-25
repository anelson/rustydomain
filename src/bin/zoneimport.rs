// Simple executable which reads all of the .zone files in the data/ directory, and builds
// a list of unique domain names (not including the TLD).  Domain names are written to data/domains.txt
//
// Output is not sorted, but is guaranteed to contain one unique domain name per line
extern crate regex;
extern crate glob;
extern crate rustydomain;

use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use regex::Regex;

use glob::glob;

use rustydomain::domains::Domains;

fn main() {
	let (tx, rx) = mpsc::sync_channel::<Arc<String>>(1024);

	let writer = thread::spawn(move || {
		domain_writer(rx);
	});

	let readers: Vec<_> = glob("data/*.zone").unwrap().into_iter().map(|p| {
		let tx = tx.clone();
		thread::spawn(move || {
			file_reader(&p.unwrap().as_path().to_str().unwrap(), tx);
		})
	}).collect();

	println!("Waiting for file reader threads to complete");
	for reader in readers {
		reader.join().unwrap();
	}

	println!("File reader threads complete; waiting for writer thread to complete");
	drop(tx);

	writer.join().unwrap();

	println!("Writer thread complete");
}

fn file_reader(p: &str, tx: mpsc::SyncSender<Arc<String>>) {
	let re = Regex::new(r"^([A-Z0-9][A-Z0-9-]*)\s+NS\s+.+$").unwrap();
	let f = match File::open(p) {
		Ok(handle) => handle,
		Err(e) => {
			println!("Error opening file {}: {}", p, e);
			return;
		}
	};

	let reader = BufReader::new(f);
	let mut line_number: usize = 0;
	let mut last_domain: String = String::new();

	for line in reader.lines() {
		line_number += 1;
		if line_number % 100000 == 0 {
			print!("Processing {} line {}\r", p, line_number);
			std::io::stdout().flush().ok().expect("flush");
		}

		match line {
			Ok(line) => {
				match re.captures(&line) {
					Some(captures) => {
						let domain = captures.at(1).unwrap();
						if domain != last_domain {
							let domain = String::from(domain);
							last_domain = domain.clone();
							tx.send(Arc::new(domain)).ok().expect("send");
						}
					},
					None => {
						println!("Non-standard line at {}:{} {}", p, line_number, line);
					}
				}
			},
			Err(e) => {
				println!("Failed to read line from {}: {}", p, e);
				break;
			}
		}
	}
}

fn domain_writer(rx: mpsc::Receiver<Arc<String>>) {
	let mut domains = Domains::create("data/domains.sqlite3");
	let mut unique_domains = 0;
	let mut tx = domains.begin_transaction();

	for domain in rx.iter() {
		if domains.add(&domain) {
			unique_domains += 1;

			if unique_domains % 10000 == 0 {
				tx.commit();
				let tx = domains.begin_transaction();
			}
		}
	}

	tx.commit();

	println!("Found {} unique domains", unique_domains);
}