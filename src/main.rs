use std::env;
use std::io::stdin;

use app::{App, tools::SplitWithQuotes};
use data_controller::query::CONNECTING_STRING;
use data_controller::DataBase;

mod app;
mod data_controller;


fn main() {
    DataBase::connect(CONNECTING_STRING).init();

    let mut args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        args.remove(0);
        App::run(args)
        
    } else {
        loop {
            println!("Введите команду: ");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if input.trim() == "exit" || input.trim() == "Выход" { break };
            App::run(input.split_with_quotes());
        }
    }
}
