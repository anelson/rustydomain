// Consumes the domains list in data/domains.txt, training a Markov model in the generation of
// domain names.
extern crate rustydomain;

use std::fs::File;

use rustydomain::domains::Domains;
use rustydomain::markov::Markov;

static ORDER: u32 = 2;

fn main() {
	println!("Loading domains file");
	let domains = Domains::open("data/domains.sqlite3");
	println!("Domains file loaded");

	let mut markov = Markov::new(ORDER);
	println!("Training markov model");
	train_markov(&domains, &mut markov);

	println!("Training complete; persisting markov model");
	persist_markov(&markov);
}

fn train_markov(_domains: &Domains, _markov: &mut Markov) {
	unimplemented!();
	/*
	let mut count: u32 = 0;
	let total = domains.domains.len() as f32;

	for domain in domains.domains.iter() {
		count = count + 1;
		if count % 10000 == 0 {
			let percent: f32 = (count as f32) / total;
			println!("Processing domain {} ({}% complete)", domain, percent * 100.0);
		}

		markov.learn(&domain);
	}
	*/
}

fn persist_markov(markov: &Markov) {
	let mut file = match File::create("data/markov.chain") {
		Ok(handle) => handle,
		Err(e) => {
			panic!("Error creating markov file: {}", e);
		}
	};

	markov.save(&mut file);
}