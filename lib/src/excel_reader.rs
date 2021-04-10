use calamine::{Xlsx, Reader};
use crate::json_data::{DataRoot, Translation};
use std::collections::BTreeMap;
use crate::excel_file::EFile;

pub fn imp_excel(file: &EFile, data_root: &mut DataRoot, project_id: u16, ignore_unknown: bool) {
    let mut lang_list: Vec<String> = vec![];

    for (idx, row) in file.rows().into_iter().enumerate() {
        if 0.eq(&idx) {
            for column in 1..row.len() {
                lang_list.push(row[column].to_string());
            }
            continue;
        }

        let key = row[0].to_string();
        if key.is_empty() {
            continue;
        }

        for (idx, lang) in lang_list.iter().enumerate() {
            let value = row[idx + 1].to_string();

            if data_root.translations.contains_key(&key) {
                update_key_value(data_root, project_id, &key, lang, &value);
            } else if !ignore_unknown {
                add_new_key(data_root, project_id, key.to_string(), lang, value);
            } else { }
        }
    }
}

pub fn import_excel(file: &str, data_root: &mut DataRoot, project_id: u16, ignore_unknown: bool) {
    let mut workbook: Xlsx<_> = calamine::open_workbook(file)
        .expect("Cannot open file");

    let worksheet = workbook.worksheets()
        .first()
        .expect("Cannot read sheet")
        .clone();

    let mut lang_list: Vec<String> = vec![];

    for (idx, row) in worksheet.1.rows().into_iter().enumerate() {
        if 0.eq(&idx) {
            for column in 1..row.len() {
                lang_list.push(row[column].to_string());
            }
            continue;
        }

        let key = row[0].to_string();
        if key.is_empty() {
            continue;
        }

        for (idx, lang) in lang_list.iter().enumerate() {
            let value = row[idx + 1].to_string();

            if data_root.translations.contains_key(&key) {
                update_key_value(data_root, project_id, &key, lang, &value);
            } else if !ignore_unknown {
                add_new_key(data_root, project_id, key.to_string(), lang, value);
            } else { }
        }
    }
}

fn update_key_value(data_root: &mut DataRoot, project_id: u16, key: &String, lang: &String, value: &String) {
    let translation_data = data_root.translations.get_mut(key)
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

fn add_new_key(data_root: &mut DataRoot, project_id: u16, key: String, lang: &String, value: String) {
    println!("Adding new key: {}", key);

    let mut values_lang_map = BTreeMap::new();
    values_lang_map.insert(lang.to_string(), value);

    let mut values_map = BTreeMap::new();
    values_map.insert(project_id, values_lang_map);

    let value_node = Translation { projects: vec![1], values: values_map };
    data_root.translations.insert(key.to_string(), value_node);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_key() {
        let mut data_root = generate_basic_data();

        add_new_key(&mut data_root, 1, "t2".to_string(), &"en".to_string(), "test1".to_string());

        assert_eq!(2, data_root.translations.len());
        assert_eq!(true, data_root.translations.contains_key("t2"));
    }

    fn generate_basic_data() -> DataRoot {
        let mut value_map = BTreeMap::new();
        value_map.insert("en".to_string(), "Hello".to_string());

        let mut values_map = BTreeMap::new();
        values_map.insert(1, value_map);

        let mut translation_map = BTreeMap::new();
        let translation = Translation { projects: vec![1], values: values_map };
        translation_map.insert("t1".to_string(), translation);

        let mut data_root = DataRoot { projects: vec![], translations: translation_map };
        data_root
    }
}