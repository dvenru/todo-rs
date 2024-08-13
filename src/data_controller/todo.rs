use chrono::NaiveDate;

#[derive(Debug)]
pub enum TodoStatus {
    On,
    Done
}

impl TodoStatus {
    pub fn from_string(value: String) -> Result<Self, String> {
        match value.trim() {
            "on" => Ok(TodoStatus::On),
            "done" => Ok(TodoStatus::Done),
            _ => Err("Ошибка ввода статуса! Используйте 'on' или 'done'.".to_string())
        }
    }
}

impl ToString for TodoStatus {
    fn to_string(&self) -> String {
        match self {
            Self::On => String::from("on"),
            Self::Done => String::from("done")
        }
    }
}

#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub description: String,
    pub date: String,
    pub category: String,
    pub status: TodoStatus
}

impl Todo {
    /// Обновляет значения полей задачи
    pub fn update(&mut self, args: &[String]) -> Result<(), String> {
        if let Some(arg) = args.get(0) {
            self.description = arg.clone()
        }
        if let Some(arg) = args.get(1) {
            self.date = arg.clone()
        }
        if let Some(arg) = args.get(2) {
            self.category = arg.clone()
        }
        if let Some(arg) = args.get(3) {
            match TodoStatus::from_string(arg.clone()) {
                Ok(s) => self.status = s,
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }

    /// Выводит в консоль форматированный вариант задачи
    pub fn format_print(&self) {
        println!("{}", self.name);
        println!("{}", self.description);
        println!("Category: {}", self.category);
        println!("Date: {}", self.date);
        println!("Status: {} \n", self.status.to_string());
    }
}

impl Todo {
    pub fn from_string_slice(value: &[String]) -> Result<Self, String> {
        Ok(
            Todo {
                name: value[0].clone(),
                description: value[1].clone(),
                date: match NaiveDate::parse_from_str(&value[2], "%Y-%m-%d") {
                    Ok(_) => value[2].clone(),
                    Err(_) => return Err("Ошибка ввода даты! Формат даты: год-месяц-день".to_string())
                },
                category: value[3].clone(),
                status: match TodoStatus::from_string(value[4].clone()) {
                    Ok(s) => s,
                    Err(e) => return Err(e)
                }
            }
        )
    }
}