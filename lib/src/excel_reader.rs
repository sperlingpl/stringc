use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::excel_file::EFile;
use crate::json_data::{Translation, Project, DataRootTranslations};

pub struct ImportResult {
    pub added: Vec<String>,
    pub updated: Vec<String>,
    pub ignored: Vec<String>
}

#[derive(Debug, Clone)]
pub struct InvalidLanguageError;

impl fmt::Display for InvalidLanguageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid language in file")
    }
}

impl Error for InvalidLanguageError {}

struct Lang {
    name: String,
    column: u16
}

type Result<T> = std::result::Result<T, InvalidLanguageError>;

pub fn import_excel(file: &mut dyn EFile, data_root: &mut DataRootTranslations, project: &Project, ignore_unknown: bool)
                    -> Result<ImportResult> {
    let mut lang_list: Vec<Lang> = vec![];
    let mut result = ImportResult { added: vec![], updated: vec![], ignored: vec![] };

    for (idx, row) in file.rows().into_iter().enumerate() {
        if 0.eq(&idx) {
            for column in 1..row.len() {
                let lang = Lang { name: row[column].to_string(), column: column as u16 };

                if project.langs.contains(&lang.name) {
                    lang_list.push(lang);
                } else if !ignore_unknown {
                    return Err(InvalidLanguageError);
                }
            }
            continue;
        }

        let key = row[0].to_string();
        if key.is_empty() {
            continue;
        }

        for (idx, lang) in lang_list.iter().enumerate() {
            let value = row[idx + 1].to_string();

            if data_root.contains_key(&key) {
                update_key_value(data_root, project.id, &key, &lang.name, &value);

                if !result.added.contains(&key) && !result.updated.contains(&key) {
                    result.updated.push(key.to_string());
                }
            } else if !ignore_unknown {
                add_new_key(data_root, project.id, key.to_string(), &lang.name, value);
                add_result(key.to_string(), &mut result.added);
            } else {
                add_result(key.to_string(), &mut result.ignored);
            }
        }
    }

    Ok(result)
}

fn add_result(key: String, list: &mut Vec<String>) {
    if !list.contains(&key) {
        list.push(key);
    }
}

fn update_key_value(data_root: &mut DataRootTranslations, project_id: u16, key: &String, lang: &String, value: &String) {
    let translation_data = data_root.get_mut(key)
        .unwrap();

    if !translation_data.projects.contains(&project_id) {
        translation_data.projects.push(project_id);
    }

    translation_data.values.entry(project_id)
        .or_insert(BTreeMap::new());

    translation_data.values.get_mut(&project_id)
        .unwrap()
        .insert(lang.to_string(), value.to_string());
}

fn add_new_key(data_root: &mut DataRootTranslations, project_id: u16, key: String, lang: &String, value: String) {
    println!("Adding new key: {}", key);

    let mut values_lang_map = BTreeMap::new();
    values_lang_map.insert(lang.to_string(), value);

    let mut values_map = BTreeMap::new();
    values_map.insert(project_id, values_lang_map);

    let value_node = Translation { projects: vec![1], values: values_map };
    data_root.insert(key.to_string(), value_node);
}
