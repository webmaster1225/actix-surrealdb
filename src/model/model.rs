use bounce::Atom;
use serde::{ Serialize, Deserialize };
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Number,
    SingleSelect,
    MultipleSelect,
    Date,
    Url,
    Checkbox,
    Attachment,
    Subtask,
}
impl Default for ContentType {
    fn default() -> Self {
        ContentType::Text
    }
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cell {
    pub content: Vec<String>,
    pub content_type: ContentType,
}
impl Default for Cell {
    fn default() -> Self {
        Cell {
            content: vec![],
            content_type: ContentType::default(),
        }
    }
}
#[derive(Atom, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub data: Vec<Vec<Cell>>, // current selected table
}

impl Default for Table {
    fn default() -> Self {
        Table {
            name: "Table 1".to_string(),
            data: vec![
                vec![
                    Cell { content: vec!["Name".to_string()], content_type: ContentType::Text },
                    Cell { content: vec!["Age".to_string()], content_type: ContentType::Number },
                    Cell {
                        content: vec!["Device Type".to_string()],
                        content_type: ContentType::SingleSelect,
                    },
                    Cell {
                        content: vec!["Current Owner".to_string()],
                        content_type: ContentType::MultipleSelect,
                    },
                    Cell { content: vec!["Date".to_string()], content_type: ContentType::Date },
                    Cell { content: vec!["Url".to_string()], content_type: ContentType::Url },
                    Cell {
                        content: vec!["Checkbox".to_string()],
                        content_type: ContentType::Checkbox,
                    },
                    Cell {
                        content: vec!["Photo".to_string()],
                        content_type: ContentType::Attachment,
                    },
                    Cell {
                        content: vec!["Subtask".to_string()],
                        content_type: ContentType::Subtask,
                    }
                ],
                vec![
                    Cell {
                        content: vec!["Row 1, Cell 1".to_string()],
                        content_type: ContentType::Text,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 2".to_string()],
                        content_type: ContentType::Number,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 3".to_string()],
                        content_type: ContentType::SingleSelect,
                    },
                    Cell {
                        content: vec!["Row 1".to_string(), "Cell 4".to_string()],
                        content_type: ContentType::MultipleSelect,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 5".to_string()],
                        content_type: ContentType::Date,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 6".to_string()],
                        content_type: ContentType::Url,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 7".to_string()],
                        content_type: ContentType::Checkbox,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 8".to_string()],
                        content_type: ContentType::Attachment,
                    },
                    Cell {
                        content: vec!["Row 1, Cell 9".to_string()],
                        content_type: ContentType::Subtask,
                    }
                ],
                vec![
                    Cell {
                        content: vec!["Row 2, Cell 1".to_string()],
                        content_type: ContentType::Text,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 2".to_string()],
                        content_type: ContentType::Number,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 3".to_string()],
                        content_type: ContentType::SingleSelect,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 4".to_string()],
                        content_type: ContentType::MultipleSelect,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 5".to_string()],
                        content_type: ContentType::Date,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 6".to_string()],
                        content_type: ContentType::Url,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 7".to_string()],
                        content_type: ContentType::Checkbox,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 8".to_string()],
                        content_type: ContentType::Attachment,
                    },
                    Cell {
                        content: vec!["Row 2, Cell 9".to_string()],
                        content_type: ContentType::Subtask,
                    }
                ]
            ],
        }
    }
}

// #[derive(Atom, PartialEq)]
// pub struct Tables {
//     pub tables: Vec<Table>,
// }

// impl Default for Tables {
//     fn default() -> Self {
//         Tables {
//             tables: vec![
//                 Table {
//                     name: "Table 1".to_string(),
//                     data: vec![
//                         vec![
//                             Cell {
//                                 content: vec!["Name".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Age".to_string()],
//                                 content_type: ContentType::Number,
//                             },
//                             Cell {
//                                 content: vec!["Device Type".to_string()],
//                                 content_type: ContentType::SingleSelect,
//                             },
//                             Cell {
//                                 content: vec!["Current Owner".to_string()],
//                                 content_type: ContentType::MultipleSelect,
//                             },
//                             Cell {
//                                 content: vec!["Date".to_string()],
//                                 content_type: ContentType::Date,
//                             },
//                             Cell {
//                                 content: vec!["Url".to_string()],
//                                 content_type: ContentType::Url,
//                             },
//                             Cell {
//                                 content: vec!["Checkbox".to_string()],
//                                 content_type: ContentType::Checkbox,
//                             },
//                             Cell {
//                                 content: vec!["Photo".to_string()],
//                                 content_type: ContentType::Attachment,
//                             },
//                             Cell {
//                                 content: vec!["Subtask".to_string()],
//                                 content_type: ContentType::Subtask,
//                             }
//                         ],
//                         vec![
//                             Cell {
//                                 content: vec!["Row 1, Cell 1".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 2".to_string()],
//                                 content_type: ContentType::Number,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 3".to_string()],
//                                 content_type: ContentType::SingleSelect,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 4".to_string()],
//                                 content_type: ContentType::MultipleSelect,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 5".to_string()],
//                                 content_type: ContentType::Date,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 6".to_string()],
//                                 content_type: ContentType::Url,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 7".to_string()],
//                                 content_type: ContentType::Checkbox,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 8".to_string()],
//                                 content_type: ContentType::Attachment,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell 9".to_string()],
//                                 content_type: ContentType::Subtask,
//                             }
//                         ],
//                         vec![
//                             Cell {
//                                 content: vec!["Row 2, Cell 1".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 2".to_string()],
//                                 content_type: ContentType::Number,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 3".to_string()],
//                                 content_type: ContentType::SingleSelect,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 4".to_string()],
//                                 content_type: ContentType::MultipleSelect,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 5".to_string()],
//                                 content_type: ContentType::Date,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 6".to_string()],
//                                 content_type: ContentType::Url,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 7".to_string()],
//                                 content_type: ContentType::Checkbox,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 8".to_string()],
//                                 content_type: ContentType::Attachment,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell 9".to_string()],
//                                 content_type: ContentType::Subtask,
//                             }
//                         ]
//                     ],
//                 },
//                 Table {
//                     name: "Table 2".to_string(),
//                     data: vec![
//                         vec![
//                             Cell {
//                                 content: vec!["Header A".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Header B".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Header C".to_string()],
//                                 content_type: ContentType::Text,
//                             }
//                         ],
//                         vec![
//                             Cell {
//                                 content: vec!["Row 1, Cell A".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell B".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Row 1, Cell C".to_string()],
//                                 content_type: ContentType::Text,
//                             }
//                         ],
//                         vec![
//                             Cell {
//                                 content: vec!["Row 2, Cell A".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell B".to_string()],
//                                 content_type: ContentType::Text,
//                             },
//                             Cell {
//                                 content: vec!["Row 2, Cell C".to_string()],
//                                 content_type: ContentType::Text,
//                             }
//                         ]
//                     ],
//                 }
//             ],
//         }
//     }
// }
