use crate::json_data::{DataRootTranslations, Project};
use std::collections::BTreeMap;
use crate::ios_generator::TranslationsIOS;

pub trait Generator {
    fn generate(&self) -> std::io::Result<()>;
}

#[derive(Clone)]
pub enum StringsGeneratorType {
    Ios,
    Android
}

pub struct TranslationOut {
    pub key: String,
    pub value: String
}

pub fn generate_strings(export_type: StringsGeneratorType, data: &DataRootTranslations, project: &Project) {
    for lang in &project.langs {
        let strings = prepare_strings(export_type.clone(), &data, &project.id, lang.to_string());

        match export_type {
            StringsGeneratorType::Ios => {
                let ios_generator = TranslationsIOS { lang: lang.to_string(), translations: strings };
                ios_generator.generate();
            }
            StringsGeneratorType::Android => {}
        }
    }
}

fn prepare_strings(export_type: StringsGeneratorType, data: &DataRootTranslations, project_id: &u16, lang: String)
    -> Vec<TranslationOut> {
    let mut translations: Vec<TranslationOut> = vec![];

    let items: BTreeMap<_, _> = data.iter()
        .filter(|&p| p.1.projects.contains(&project_id))
        .collect();

    for item in items {
        let mut value: &str = item.0.as_str();

        if let Some(k) = item.1.values.get(&project_id) {
            if let Some(t) = k.get(&lang) {
                value = t.as_str();
            }
        }

        let translation = TranslationOut { key: item.0.to_string(), value: value.to_string()};
        translations.push(translation);
    }

    translations
}

#[cfg(test)]
mod tests {
    use crate::json_data::{get_projects, DataRoot};
    use crate::strings_generator::prepare_strings;
    use crate::strings_generator::StringsGeneratorType::Ios;

    #[test]
    fn export_ios() {
        let data = generate_basic_data();
        let strings = prepare_strings(Ios, &data.translations, &data.projects[0].id, "en-US".to_string());

        assert_eq!(strings.len(), 3);
        assert_eq!(strings[0].key, "app.hello");
        assert_eq!(strings[1].value, "test1-en-p1");
        assert_eq!(strings[2].value, "app.t3");
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
}
