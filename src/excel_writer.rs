pub struct ExcelTranslations {
    langs: Vec<String>,
    values: Vec<ExcelTranslation>
}

pub struct ExcelTranslation {
    key: String,
    values: Vec<String>
}

impl ExcelTranslations {
    pub fn new(langs: Vec<String>, values: Vec<ExcelTranslation>) -> ExcelTranslations {
        ExcelTranslations { langs, values }
    }
}

impl ExcelTranslation {
    pub fn new(key: String, values: Vec<String>) -> ExcelTranslation {
        ExcelTranslation { key, values }
    }
}