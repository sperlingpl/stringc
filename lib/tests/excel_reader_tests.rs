use std::collections::BTreeMap;

use lib::excel_file::{EFile, ExcelFile};
use lib::excel_reader::{imp_excel, InvalidLanguageError};
use lib::json_data::{DataRoot, get_projects, Project, Translation};
use std::panic::resume_unwind;

struct ExcelFileMock {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl EFile for ExcelFileMock {
    fn rows(&self) -> Vec<Vec<String>> {
        self.rows.clone()
    }

    fn columns(&self) -> Vec<String> {
        self.columns.clone()
    }
}

#[test]
fn import_excel_wrong_language_project1_file() {
    let file = ExcelFileMock {
        rows: vec![
            vec!["key".to_string(), "en-US".to_string(), "de-DE".to_string()],
            vec!["new1".to_string(), "added1-en".to_string(), "added1-de".to_string()],
            vec!["app.t2".to_string(), "updated1-en".to_string(), "updated1-de".to_string()]
        ],
        columns: vec![],
    };

    let mut data_root = generate_basic_data();
    let project = &data_root.projects[0];
    let result = imp_excel(&file, &mut data_root.translations, project, false);

    assert_eq!(result.is_err(), true);
}

#[test]
fn import_excel_project2_file() {
    let file = ExcelFileMock {
        rows: vec![
            vec!["key".to_string(), "en-US".to_string(), "de-DE".to_string()],
            vec!["new1".to_string(), "added1-en".to_string(), "added1-de".to_string()],
            vec!["app.t2".to_string(), "updated1-en".to_string(), "updated1-de".to_string()]
        ],
        columns: vec![],
    };

    let mut data_root = generate_basic_data();
    let result = imp_excel(&file, &mut data_root.translations, &data_root.projects[1], false);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(1, result.added.len());
    assert_eq!(1, result.updated.len());
    assert_eq!(0, result.ignored.len());
    assert_eq!(2, data_root.projects.len());
    assert_eq!(5, data_root.translations.len());
    assert_eq!(true, data_root.translations.contains_key("new1"));
    assert_eq!(true, data_root.translations.contains_key("app.t2"));
    assert_eq!("added1-en", data_root.translations.get("new1").unwrap().values.get(&2).unwrap().get("en-US").unwrap());
    assert_eq!("updated1-de", data_root.translations.get("app.t2").unwrap().values.get(&2).unwrap().get("de-DE").unwrap());
}

#[test]
fn import_empty_excel_file() {
    let file = ExcelFileMock { rows: vec![], columns: vec![] };
    let mut data_root = generate_basic_data();

    let result = imp_excel(&file, &mut data_root.translations, &data_root.projects[0], false);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result.added.len());
    assert_eq!(0, result.updated.len());
    assert_eq!(0, result.ignored.len());
    assert_eq!(2, data_root.projects.len());
    assert_eq!(4, data_root.translations.len());
    assert_eq!("Hello", data_root.translations.get("app.hello").unwrap().values.get(&1).unwrap().get("en-US").unwrap())
}

#[test]
fn import_ignore_unknown_excel_file() {
    let file = ExcelFileMock {
        rows: vec![
            vec!["key".to_string(), "en-US".to_string(), "de-DE".to_string()],
            vec!["new1".to_string(), "added1-en".to_string(), "added1-de".to_string()],
            vec!["app.t2".to_string(), "updated1-en".to_string(), "updated1-de".to_string()]
        ],
        columns: vec![],
    };
    let mut data_root = generate_basic_data();

    let result = imp_excel(&file, &mut data_root.translations, &data_root.projects[1], true);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result.added.len());
    assert_eq!(1, result.updated.len());
    assert_eq!(1, result.ignored.len());
    assert_eq!(2, data_root.projects.len());
    assert_eq!(4, data_root.translations.len());
    assert_eq!(false, data_root.translations.contains_key("new1"));
    assert_eq!("updated1-en", data_root.translations.get("app.t2").unwrap().values.get(&2).unwrap().get("en-US").unwrap());
    assert_eq!("updated1-de", data_root.translations.get("app.t2").unwrap().values.get(&2).unwrap().get("de-DE").unwrap());
}

fn generate_basic_data() -> DataRoot {
    let json = r#"
    {
      "projects": [
        {
          "id": 1,
          "name": "TestProject1",
          "langs": [
            "en-US",
            "pl-PL"
          ],
          "defaultLang": "en-US"
        },
        {
          "id": 2,
          "name": "TestProject2",
          "langs": [
            "en-US",
            "de-DE"
          ],
          "defaultLang": "en-US"
        }
      ],
      "translations": {
        "app.hello": {
          "projects": [
            1,
            2
          ],
          "values": {
            "1": {
              "en-US": "Hello",
              "pl-PL": "Witaj"
            },
            "2": {
              "en-US": "Hello",
              "de-DE": "Hallo"
            }
          }
        },
        "app.t1": {
          "projects": [
            1,
            2
          ],
          "values": {
            "1": {
              "en-US": "test1-en-p1",
              "pl-PL": "test1-pl-p1"
            },
            "2": {
              "en-US": "test1-en-proj2",
              "de-DE": "test1-de-proj2"
            }
          }
        },
        "app.t2": {
          "projects": [
            2
          ],
          "values": {
            "2": {
              "en-US": "test2-en-proj2",
              "de-DE": "test2-de-proj2"
            }
          }
        },
        "app.t3": {
          "projects": [
            1
          ],
          "values": {
            "2": {
              "en-US": "test3-en-proj1"
            }
          }
        }
      }
    }
    "#;

    get_projects(json).expect("Error parsing json")
}