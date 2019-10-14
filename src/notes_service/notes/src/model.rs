use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use crate::schema::notes;

#[derive(Deserialize, Serialize, Debug, Queryable, Insertable)]
#[table_name = "notes"]
pub struct NewNote {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Queryable)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn insert(note: NewNote, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(notes::table)
            .values(&note)
            .execute(conn)
    }
}