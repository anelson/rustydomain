use std::collections::hash_map::HashMap;
use std::io::{BufReader, BufRead, BufWriter, Read, Write};

pub struct SymbolProbability {
	symbol: String,
	p: f32
}

pub struct Markov {
	order: u32,
	tuples: HashMap<String, Vec<SymbolProbability>>
}

impl Markov {
	pub fn new(order: u32) -> Markov {
		Markov { order: order, tuples: HashMap::new() }
	}

	pub fn load(reader: &Read) {
		unimplemented!();
	}

	pub fn save(&self, writer: &Write) {
		unimplemented!();
	}

	pub fn learn(&self, word: &str) {
		unimplemented!();
	}

	pub fn generate(&self, seed: &str) -> String {
		unimplemented!();
	}
}