use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct DataRoot {
    pub projects: Vec<Project>,
    pub translations: BTreeMap<String, Translation>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u16,
    pub name: String,
    pub langs: Vec<String>,
    pub default_lang: String
}

#[derive(Serialize, Deserialize)]
pub struct Translation {
    pub projects: Vec<u16>,
    pub values: BTreeMap<u16, BTreeMap<String, String>>
}

pub fn get_projects(data: &str) -> Result<DataRoot> {
    let project: DataRoot = serde_json::from_str(data)?;
    Ok(project)
}

pub fn generate_template(file_name: &str) -> std::io::Result<()> {
    let project = Project {
        id: 1,
        langs: vec!["en-US".to_string(), "pl-PL".to_string()],
        name: "TestProject".to_string(),
        default_lang: "en-US".to_string()
    };

    let mut values_map = BTreeMap::new();
    let mut values_lang_map = BTreeMap::new();
    values_lang_map.entry("en-US".to_string()).or_insert("Hello World!".to_string());
    values_lang_map.entry("pl-PL".to_string()).or_insert("Witaj świecie!".to_string());
    values_map.entry(1).or_insert(values_lang_map);

    let translation = Translation {
        projects: vec![1],
        values: values_map
    };

    let mut keys_map = BTreeMap::new();
    keys_map.entry("app.title".to_string()).or_insert(translation);

    let data_root = DataRoot { projects: vec![project], translations: keys_map };
    save(file_name, &data_root)
}

pub fn save(file_name: &str, data_root: &DataRoot) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&data_root)
        .expect("Cannot write data");
    let file = File::create(file_name)?;
    let mut file = LineWriter::new(file);

    file.write_all(json.as_ref())?;
    file.flush()?;

    Ok(())
}