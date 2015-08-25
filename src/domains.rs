use std::iter::repeat;
use std::ascii::AsciiExt;

use rusqlite::{SqliteConnection, SqliteTransaction, SqliteStatement};
use rusqlite;

use crypto::digest::Digest;
use crypto::md5::Md5;

pub struct Domains<'a> {
	conn: SqliteConnection,
	insert: SqliteStatement<'a>,
	select: SqliteStatement<'a>,
	//selectall: PreparedStatement
}

impl<'a> Domains<'a> {
	fn new(conn: SqliteConnection) -> Domains<'a> {
		let conn = match SqliteConnection::open_with_flags(&"foo", rusqlite::SQLITE_OPEN_READ_WRITE | rusqlite::SQLITE_OPEN_CREATE) {
			Ok(c) => c,
			Err(e) => {
				println!("Error: {}", e);
				panic!("Error creating SQLITE database {}: {}", "foo", e)
			}
		};
		let insert = conn.prepare("INSERT INTO domains(name, hash) VALUES($1, $2)").unwrap();
		let select = conn.prepare("SELECT name FROM domains WHERE hash = $1").unwrap();
		let d = Domains {
			conn: conn,
			insert: insert,
			select: select
		};

		/*
		Domains {
			insert: conn.prepare("INSERT INTO domains(name, hash) VALUES($1, $2)").unwrap(),
			select: conn.prepare("SELECT name FROM domains WHERE hash = $1").unwrap(),
			//selectall: conn.prepare("SELECT name FROM domains").unwrap()
			conn: conn,
		}*/
		d
	}

	pub fn create(filename: &str) -> Domains<'a> {
		let mut conn = match SqliteConnection::open_with_flags(&filename, rusqlite::SQLITE_OPEN_READ_WRITE | rusqlite::SQLITE_OPEN_CREATE) {
			Ok(c) => c,
			Err(e) => {
				println!("Error: {}", e);
				panic!("Error creating SQLITE database {}: {}", filename, e)
			}
		};

		conn.execute_batch("
			PRAGMA synchronous = OFF;
			PRAGMA journal_mode = MEMORY;
			DROP TABLE IF EXISTS domains;
			CREATE TABLE IF NOT EXISTS domains ( name STRING PRIMARY KEY ASC,  hash BINARY );
			CREATE UNIQUE INDEX IF NOT EXISTS idx_domains_hash ON domains(hash);
			").ok().expect("execute_batch");

		Domains::new(conn)
	}

	pub fn open(filename: &str) -> Domains<'a> {
		let mut conn = match SqliteConnection::open_with_flags(&filename, rusqlite::SQLITE_OPEN_READ_WRITE) {
			Ok(c) => c,
			Err(e) => {
				println!("Error: {}", e);
				panic!("Error opening SQLITE database {}: {}", filename, e)
			}
		};

		Domains::new(conn)
	}

	pub fn begin_transaction(&'a mut self) -> SqliteTransaction<'a> {
		self.conn.transaction().unwrap()
	}

	pub fn add(&mut self, domain: &str) -> bool {
		let domain = domain.to_ascii_lowercase();
		let hash = compute_hash(&domain);

		self.insert.execute(&[&domain, &hash]).unwrap();

		true
	}

	pub fn exists(&mut self, domain: &str) -> bool {
		let domain = domain.to_ascii_lowercase();
		let hash = compute_hash(&domain);

		let mut rows = self.select.query(&[&hash]).unwrap();

		rows.count() > 0
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