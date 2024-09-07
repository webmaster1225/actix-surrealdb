use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ColumnType {
    Text,
    Number,
    SingleSelect,
    MultipleSelect,
    Date,
    Url,
    Checkbox,
    FileAttachment,
    Subtask,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subtask {
    pub title: String,
    pub completed: bool,
}