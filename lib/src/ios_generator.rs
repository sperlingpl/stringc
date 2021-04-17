use std::fs::File;
use std::io::{LineWriter, Write};
use super::strings_generator::Generator;

pub struct TranslationsIOS {
    pub lang: String,
    pub translations: Vec<TranslationOut>
}

pub struct TranslationOut {
    pub key: String,
    pub value: String
}

impl Generator for TranslationsIOS {
    fn generate(&self) -> std::io::Result<()> {
        let file_name = format!("Localized_{}.strings", self.lang);

        let file = File::create(file_name)?;
        let mut file = LineWriter::new(file);

        for translation in &self.translations {
            let out_value = translation.value
                .replace("%s", "%@")
                .replace("%d", "%@")
                .replace("%c", "%@")
                .replace("\"", "\\\"");

            let out_string = format!("\"{}\" = \"{}\";\n", translation.key, out_value);
            file.write_all(out_string.as_ref())?;
        }

        file.flush()?;

        Ok(())
    }
}
