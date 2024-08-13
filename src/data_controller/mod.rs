use rusqlite::{self, Connection, Error};

pub mod query;
pub mod todo;
pub mod select_builder;

use todo::*;
use query::*;
use select_builder::*;

pub struct DataBase<'a> {
    conn_string: &'a str
}

impl<'a> DataBase<'a> {
    pub fn connect(conn_string: &'a str) -> Self {
        DataBase { conn_string }
    }

    /// Инициализация базы данных
    pub fn init(&mut self) {
        let conn = Connection::open(CONNECTING_STRING).unwrap();

        match conn.execute(
            CREATE_TABLE,
            ()
        ) {
            Ok(_) => (),
            Err(e) => println!("Ошибка: {}", e)
        }
    }

    /// Добавляет новую запись в таблицу
    pub fn add(&mut self, data: Todo) {
        let conn = Connection::open(self.conn_string).unwrap();

        match conn.execute(
            APPEND_QUERY,
            (&data.name, &data.description, &data.date, &data.category, &data.status.to_string())
        ) {
            Ok(_) => println!("Задача добавлена!"),
            Err(_) => println!("Неправильный ввод полей! Повторите ввод.")
        }
    }

    /// Помечает запись как выполненную
    pub fn done(&mut self, name: &str) {
        let conn = Connection::open(self.conn_string).unwrap();

        match conn.execute(
            DONE_QUERY,
            [name]
        ) {
            Ok(u) => {
                if u > 0 {
                    println!("Задача {} отмечена как выполненная!", name)
                } else {
                    println!("Задачи не найдено!")
                }
            },
            Err(_) => println!("")
        }
    }

    /// Обновляет запись
    pub fn update(&mut self, data: Todo) {
        let conn = Connection::open(self.conn_string).unwrap();

        match conn.execute(
            UPDATE_QUERY,
            (&data.description, &data.date, &data.category, &data.status.to_string(), &data.name)
        ) {
            Ok(u) => {
                if u > 0 {
                    println!("Задача {} обновлена!", &data.name)
                } else {
                    println!("Задачи не найдено!")
                }
            },
            Err(_) => println!("")
        }
    }

    /// Удаляет запись
    pub fn delete(&mut self, name: &str) {
        let conn = Connection::open(self.conn_string).unwrap();

        match conn.execute(
            DELETE_QUERY,
            [name]
        ) {
            Ok(u) => {
                if u > 0 {
                    println!("Задача {} удалена!", name)
                } else {
                    println!("Задачи не найдено!")
                }
            },
            Err(_) => println!("")
        }
    }

    /// Выбирает и возвращает записи
    pub fn select(&mut self, param: Option<Vec<WhereCondition>>) -> Result<Vec<Todo>, Error> {
        let conn = Connection::open(self.conn_string).unwrap();
        let mut stmt = conn.prepare(&DataBase::format_select_query(param)).unwrap();

        let query_res = stmt.query_map((),|row| {
            Ok(
                Todo {
                    name: row.get(0).unwrap(),
                    description: row.get(1).unwrap(),
                    date: row.get(2).unwrap(),
                    category: row.get(3).unwrap(),
                    status: TodoStatus::from(row.get::<usize, String>(4).unwrap())
                }
            )
        }).unwrap()
            .map(|t| t.unwrap())
            .collect::<Vec<Todo>>();

        Ok(query_res)
    }

    /// Форматирует SELECT запрос
    fn format_select_query(param: Option<Vec<WhereCondition>>) -> String {
        let mut select_query = String::from(SELECT_QUERY);

        if let Some(param) = param {
            if param.len() > 0 {
                select_query += " WHERE ";
            }
    
            for cond in param.iter() {
                select_query += &cond.to_string()
            }
        }

        select_query
    }
}