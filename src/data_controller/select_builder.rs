pub struct WhereCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
    pub next: Option<String>
}

impl ToString for WhereCondition {
    fn to_string(&self) -> String {
        let mut res = format!(
            "{} {} '{}'",
            self.field,
            self.operator,
            self.value
        );

        if let Some(n) = &self.next {
            res += n.as_str()
        }

        res
    }
}

impl From<Vec<String>> for WhereCondition {
    fn from(value: Vec<String>) -> Self {
        WhereCondition {
            field: value[0].clone(),
            operator: value[1].clone(),
            value: value[2].clone(),
            next: {
                if value.len() > 3 {
                    Some(value[3].clone())
                } else {
                    None
                }
            }
        }
    }
}