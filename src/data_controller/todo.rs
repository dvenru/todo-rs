#[derive(Debug)]
pub enum TodoStatus {
    On,
    Done
}

impl From<String> for TodoStatus {
    fn from(value: String) -> Self {
        match value.trim() {
            "on" => TodoStatus::On,
            "done" => TodoStatus::Done,
            _ => panic!("Неправильное значение статуса! Используйте 'on' или 'done'")
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
    pub fn update(&mut self, args: &[String]) {
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
            self.status = TodoStatus::from(arg.clone())
        }
    }

    pub fn format_print(&self) {

    }
}

impl From<&[String]> for Todo {
    fn from(value: &[String]) -> Self {
        Todo {
            name: value[0].clone(),
            description: value[1].clone(),
            date: value[2].clone(),
            category: value[3].clone(),
            status: TodoStatus::from(value[4].clone())
        }
    }
}