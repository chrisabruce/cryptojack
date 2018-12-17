use rusqlite::{Connection, NO_PARAMS};

pub struct Store {
    conn: rusqlite::Connection,
}

impl Store {
    pub fn new(filename: &str) -> Store {
        let conn = Connection::open(filename).unwrap();
        conn.execute(
            "create table if not exists completed_games (
                id integer primary key,
                user text not null,
                did_win integer,
                wager integer,
                created_at integer
            )",
            NO_PARAMS,
        )
        .unwrap();

        conn.execute(
            "create table if not exists accounts (
                id integer primary key,
                user_id integer,
                in_address text,
                out_address text,
                balance,
                created_at integer
                )",
            NO_PARAMS,
        )
        .unwrap();

        Store { conn }
    }
}
