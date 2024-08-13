use chrono::prelude::*;
use crate::data_controller::{query::CONNECTING_STRING, select_builder::WhereCondition, todo::Todo, DataBase};

pub mod tools;

use tools::*;

pub struct App;

impl App {
    /// Добавляет новую задачу.
    /// Формат вызова: add <name> <description> <date> <category> <status>
    fn add(args: &[String]) {
        if args.len() >= 4 {
            let data = Todo::from(args);

            match NaiveDate::parse_from_str(&data.date, "%Y-%m-%d") {
                Ok(_) => (),
                Err(_) => {
                    println!("Ошибка ввода даты! Формат даты: год-месяц-день");
                    return;
                }
            }

            DataBase::connect(CONNECTING_STRING)
                .add(data)
        } else {
            println!("Недостаточно аргументов!")
        }
    }

    /// Обновляет задачу.
    /// Формат вызова: update <name> [description] [date] [category] [status]
    fn update(args: &[String]) {
        if args.len() >= 2 {
            let mut db = DataBase::connect(CONNECTING_STRING);
            let mut data = db.select(
                Some(vec![
                    WhereCondition {
                        field: "name".to_string(),
                        operator: "=".to_string(),
                        value: args[0].clone(),
                        next: None
                    }
                ])
            ).unwrap()
                .pop()
                .unwrap();
            
            data.update(&args[1..]);
            db.update(data);

        } else {
            println!("Недостаточно аргументов!")
        }
    }

    /// Отмечает задачу как выполненную.
    /// Формат вызова: done <name>
    fn done(args: &[String]) {
        if args.len() >= 1 {
            DataBase::connect(CONNECTING_STRING)
                .done(args[0].as_str())
        } else {
            println!("Недостаточно аргументов!")
        }
    }

    /// Возвращает задачи по параметрам.
    /// Формат вызова: select * (Вовращает все задачи)
    /// Формат вызова: select * where <condition> [AND] ... (Возвращает задачи по условиям)
    /// Condition => <field> <operator> <value>
    fn select(args: &[String]) {
        if args.contains(&"*".to_string()) && args.len() < 3 {
            let mut db = DataBase::connect(CONNECTING_STRING);

        } else if args.contains(&"*".to_string()) && args.len() >= 3 {
            let mut db = DataBase::connect(CONNECTING_STRING);
            let mut params = Vec::new();
            let mut wh_arg = Vec::new();
            let where_args = &args[2..];

            for (i, arg) in where_args.iter().enumerate() {
                if ((i + 1) % 4) == 0 && arg == "AND" {
                    if let Some(_) = where_args.get(i + 1) {
                        wh_arg.push(arg.clone());
                    }
                    wh_arg.clear();
                } else if i == (where_args.len() - 1) {
                    wh_arg.push(arg.clone());
                    params.push(WhereCondition::from(wh_arg.clone()));
                    wh_arg.clear();
                } else {
                    wh_arg.push(arg.clone());
                }
            }

            println!("{:?}", db.select(Some(params)).unwrap())
        } else {
            println!("Недостаточно аргументов!")
        }
    }

    /// Удаляет задачу.
    /// Формат вызова: delete <name>
    fn delete(args: &[String]) {
        if args.len() >= 1 {
            DataBase::connect(CONNECTING_STRING)
               .delete(args[0].as_str())
       } else {
           println!("Недостаточно аргументов!")
       }
    }

    /// Входная функция. Выполняет маршрутизацию, в зависимости от введенной команды.
    pub fn run(args: Vec<String>) {
        let arguments = &args[1..];

        match args[0].as_str() {
            "add" => App::add(&arguments),

            "done" => App::done(&arguments),

            "update" => App::update(&arguments),

            "delete" => App::delete(&arguments),

            "select" => App::select(&arguments),

            _ => println!("Команда не распознана!")
        }
    }
}