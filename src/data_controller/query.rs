pub const CONNECTING_STRING: &str = "data/todo.db";

pub const CREATE_TABLE: &str = "CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    date TEXT NOT NULL,
    category TEXT NOT NULL,
    status TEXT NOT NULL
)";

pub const APPEND_QUERY: &str = "INSERT INTO todo (name, description, date, category, status) VALUES (?1, ?2, ?3, ?4, ?5)";

pub const DONE_QUERY: &str = "UPDATE todo SET status = 'done' WHERE name = ?1";

pub const UPDATE_QUERY: &str = "UPDATE todo SET description = ?1, date = ?2, category = ?3, status =?4 WHERE name = ?5";

pub const DELETE_QUERY: &str = "DELETE FROM todo WHERE name = ?1";

pub const SELECT_QUERY: &str = "SELECT name, description, date, category, status FROM todo";
