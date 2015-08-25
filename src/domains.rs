extern crate sqlite3;
extern crate crypto;

use std::collections::hash_set::HashSet;
use std::io::{BufReader, BufRead, Read};
use std::fs::File;
use std::ascii::AsciiExt;

use sqlite3::{DatabaseConnection, PreparedStatement};

use crypto::digest::Digest;
use crypto::Md5;

pub struct Domains {
	conn: DatabaseConnection,
	insert: PreparedStatement,
	select: PreparedStatement
}

impl Domains {
	fn new(conn: mut DatabaseConnection) -> Domains {
		Domains {
			conn: conn,
			insert: try!(conn.prepare("INSERT INTO domains(name, hash) VALUES($1, $2)")),
			select: try!(conn.prepare("SELECT name FROM domains WHERE hash = $1")),
			selectall: try!(conn.prepare("SELECT name FROM domains"))
		}
	}

	pub fn create(filename: String) {
		let mut conn = try!(sqlite::access::open_file(filename));

		try!(conn.exec("DROP TABLE IF EXISTS domains"));
		try!(conn.exec("CREATE TABLE IF NOT EXISTS domains ( name STRING PRIMARY KEY ASC,  hash BINARY )"));
		try!(conn.exec("CREATE UNIQUE INDEX IF NOT EXISTS idx_domains_hash ON domains(hash)"));

		new(conn)
	}

	pub fn open(filename: String) -> Domains {
		let mut conn = try!(sqlite::access::open_file(filename));

		new(conn)
	}

	pub fn add(&mut self, domain: &str) -> bool {
		let hash = compute_hash(domain);
		try!(self.insert.clear_bindings());
		try!(self.insert.bind_text(1, domain));
		try!(self.insert.bind_blob(2, hash));
		let results = try!(self.insert.execute());
		drop(results);

		true;
	}

	pub fn exists(&mut self, domain: &str) -> bool {
		let hash = compute_hash(domain);
		let results = try!(self.select.execute());

		match results.step() {
			Some(result) => true,
			None => false
		}
	}

	pub fn foreach(&mut self) {
		unimplemented!();
	}

	fn compute_hash(domain: &str) -> [u8] {
		let mut hasher = Md5::new();

		hasher.input_str(domain);
		let mut buf: Vec<u8> = repeat(0).take((hasher.output_bits()+7)/8).collect();
    hasher.result(&mut buf);

    buf
	}
}

impl Drop for Domains {
	fn drop(&mut self) {
		drop(self.conn)
		drop(self.insert)
		drop(self.select)
	}
}