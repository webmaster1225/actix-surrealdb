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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cell {
    pub content: Vec<String>,
    pub content_type: ContentType,
}

// pub struct TableDB;

// impl TodoBMC {
//     pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
//         let ast = "SELECT * FROM todo;";

//         let res = db.ds.execute(ast, &db.ses, None, true).await?;

//         let first_res = res.into_iter().next().expect("Did not get a response");

//         let array: Array = W(first_res.result?).try_into()?;

//         array
//             .into_iter()
//             .map(|value| W(value).try_into())
//             .collect()
//     }

//     pub async fn create<T: Creatable>(
//         db: Data<SurrealDBRepo>,
//         tb: &str,
//         data: T
//     ) -> Result<Object, Error> {
//         let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

//         let data: Object = W(data.into()).try_into()?;

//         let vars: BTreeMap<
//             String,
//             Value
//         > = map![
// 			"tb".into() => tb.into(),
// 			"data".into() => Value::from(data)];

//         let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

//         let first_val = ress
//             .into_iter()
//             .next()
//             .map(|r| r.result)
//             .expect("id not returned")?;

//         W(first_val.first()).try_into()
//     }

//     pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
//         let sql = "SELECT * FROM $th";

//         let tid = format!("todo:{}", tid);

//         let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];

//         let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

//         let first_res = ress.into_iter().next().expect("Did not get a response");

//         W(first_res.result?.first()).try_into()
//     }
//     pub async fn update<T: Patchable>(
//         db: Data<SurrealDBRepo>,
//         tid: &str,
//         data: T
//     ) -> Result<Object, Error> {
//         let sql = "UPDATE $th MERGE $data RETURN *";

//         let tid = format!("todo:{}", tid);

//         let vars =
//             map![
//         "th".into() => thing(&tid)?.into(),
//         "data".into() => data.into()];

//         let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

//         let first_res = ress.into_iter().next().expect("id not returned");

//         let result = first_res.result?;

//         // Handle the Option case and extract the Value
//         if let Some(value) = result.first() {
//             // Attempt the conversion
//             W(value).try_into()
//         } else {
//             // Handle the case where result.first() is None
//             Err(Error::from("No result found"))
//         }
//     }

//     pub async fn delete(db: Data<SurrealDBRepo>, tid: &str) -> Result<String, Error> {
//         let sql = "DELETE $th RETURN *";

//         let tid = format!("todo:{}", tid);

//         let vars = map!["th".into() => thing(&tid)?.into()];

//         let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

//         let first_res = ress.into_iter().next().expect("id not returned");

//         first_res.result?;

//         Ok(tid)
//     }
// }
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
