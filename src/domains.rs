use std::iter::repeat;
use std::ascii::AsciiExt;

use sqlite3::{DatabaseConnection, PreparedStatement};
use sqlite3::access;

use crypto::digest::Digest;
use crypto::md5::Md5;

pub struct Domains {
	conn: DatabaseConnection,
	insert: PreparedStatement,
	select: PreparedStatement,
	begin: PreparedStatement,
	commit: PreparedStatement,
	//selectall: PreparedStatement
}

impl Domains {
	fn new(conn: DatabaseConnection) -> Domains {
		Domains {
			insert: conn.prepare("INSERT INTO domains(name, hash) VALUES($1, $2)").unwrap(),
			select: conn.prepare("SELECT name FROM domains WHERE hash = $1").unwrap(),
			begin: conn.prepare("BEGIN TRANSACTION").unwrap(),
			commit: conn.prepare("COMMIT").unwrap(),
			//selectall: conn.prepare("SELECT name FROM domains").unwrap()
			conn: conn,
		}
	}

	pub fn create(filename: &str) -> Domains {
		//let mut conn = try!(sqlite3::access::open(filename));
		let mut conn = match DatabaseConnection::new(access::ByFilename { filename: filename, flags: access::flags::OPEN_CREATE | access::flags::OPEN_READWRITE }) {
			Ok(c) => c,
			Err(e) => {
				println!("Error: {}", e);
				panic!("Error creating SQLITE database {}: {}", filename, e)
			}
		};

		conn.exec("PRAGMA synchronous = OFF").unwrap();
		conn.exec("PRAGMA journal_mode = MEMORY").unwrap();
		conn.exec("DROP TABLE IF EXISTS domains").unwrap();
		conn.exec("CREATE TABLE IF NOT EXISTS domains ( name STRING PRIMARY KEY ASC,  hash BINARY )").unwrap();
		conn.exec("CREATE UNIQUE INDEX IF NOT EXISTS idx_domains_hash ON domains(hash)").unwrap();

		Domains::new(conn)
	}

	pub fn open(filename: &str) -> Domains {
		let conn = DatabaseConnection::new(access::ByFilename { filename: filename, flags: access::flags::OPEN_READWRITE }).ok().expect("sqlite open");

		Domains::new(conn)
	}

	pub fn begin_transaction(&mut self) {
		self.begin.execute().step().unwrap();
	}

	pub fn commit_transaction(&mut self) {
		self.commit.execute().step().unwrap();
	}

	pub fn add(&mut self, domain: &str) -> bool {
		let domain = domain.to_ascii_lowercase();
		let hash = compute_hash(&domain);


		self.insert.clear_bindings();
		self.insert.bind_text(1, &domain).unwrap();
		self.insert.bind_blob(2, &hash).unwrap();

		let results = self.insert.execute();
		drop(results);

		true
	}

	pub fn exists(&mut self, domain: &str) -> bool {
		let domain = domain.to_ascii_lowercase();
		let hash = compute_hash(&domain);

		self.insert.clear_bindings();
		self.select.bind_blob(1, &hash).unwrap();

		let mut results = self.select.execute();

		match results.step().unwrap() {
			Some(_) => true,
			None => false
		}
	}

	pub fn foreach(&mut self) {
		unimplemented!();
	}
}

fn compute_hash(domain: &str) -> Vec<u8> {
	let mut hasher = Md5::new();

	hasher.input_str(domain);
	let mut buf: Vec<u8> = repeat(0).take((hasher.output_bits()+7)/8).collect();
  hasher.result(&mut buf);

  buf
}