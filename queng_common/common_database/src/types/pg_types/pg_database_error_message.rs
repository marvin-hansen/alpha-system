use diesel::result::DatabaseErrorInformation;

#[derive(Debug)]
pub struct DatabaseErrorMessage {
    message: String,
    table_name: String,
}

impl DatabaseErrorMessage {
    pub fn new(message: &str, table_name: &str) -> Self {
        Self {
            message: String::from(message),
            table_name: String::from(table_name),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}

impl DatabaseErrorInformation for DatabaseErrorMessage {
    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn details(&self) -> Option<&str> {
        None
    }

    fn hint(&self) -> Option<&str> {
        None
    }

    fn table_name(&self) -> Option<&str> {
        Some(self.table_name.as_str())
    }

    fn column_name(&self) -> Option<&str> {
        None
    }

    fn constraint_name(&self) -> Option<&str> {
        None
    }

    fn statement_position(&self) -> Option<i32> {
        None
    }
}
