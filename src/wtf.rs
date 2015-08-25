	use rusqlite::{SqliteConnection, SqliteTransaction, SqliteStatement};
	use rusqlite;

	pub struct Foo<'a> {
		conn: SqliteConnection,
		statement: Option<SqliteStatement<'a>>
	}

	impl<'a> Foo<'a> {
		pub fn new() -> Foo<'a> {
			let mut foo = Foo {
				conn: SqliteConnection::open(&":memory:").unwrap(),
				statement: None
			};

			foo.statement = Some(foo.conn.prepare("INSERT INTO Foo(name, hash) VALUES($1, $2)").unwrap());

			foo
		}
	}
