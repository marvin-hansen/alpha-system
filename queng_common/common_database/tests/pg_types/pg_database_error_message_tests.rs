use common_database::prelude::DatabaseErrorMessage;
#[test]
fn test_database_error_message_new() {
    let message = "Error occurred";
    let table_name = "users";
    let error_message = DatabaseErrorMessage::new(message, table_name);
    assert_eq!(error_message.message(), message);
    assert_eq!(error_message.table_name(), table_name);
}

#[test]
fn test_database_error_message_message() {
    let message = "Error occurred";
    let table_name = "users";
    let error_message = DatabaseErrorMessage::new(message, table_name);
    assert_eq!(error_message.message(), message);
}

#[test]
fn test_database_error_message_table_name() {
    let message = "Error occurred";
    let table_name = "users";
    let error_message = DatabaseErrorMessage::new(message, table_name);
    assert_eq!(error_message.table_name(), table_name);
}
