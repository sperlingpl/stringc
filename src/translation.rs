use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct TranslationsProject {
    pub projects: Vec<Project>,
    pub translations: Vec<Translations>
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
pub struct Translations {
    pub key: String,
    pub projects: Vec<u16>,
    pub values: Vec<Translation>
}

#[derive(Serialize, Deserialize)]
pub struct Translation {
    pub value: String,
    pub lang: String,
    pub project: u16
}

pub fn get_projects(data: &String) -> Result<TranslationsProject> {
    let project: TranslationsProject = serde_json::from_str(data)?;
    Ok(project)
}