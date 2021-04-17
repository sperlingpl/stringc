use std::fs::File;
use std::io::{LineWriter, Write};
use crate::strings_generator::{Generator, TranslationOut};

pub struct TranslationsIOS {
    pub lang: String,
    pub translations: Vec<TranslationOut>
}

impl Generator for TranslationsIOS {
    fn generate(&self) -> std::io::Result<()> {
        let file_name = format!("Localized_{}.strings", self.lang);

        let file = File::create(file_name)?;
        let mut file = LineWriter::new(file);

        for translation in &self.translations {
            let out_value = TranslationsIOS::escape(&translation.value);

            let out_string = format!("\"{}\" = \"{}\";\n", translation.key, out_value);
            file.write_all(out_string.as_ref())?;
        }

        file.flush()?;

        Ok(())
    }
}

impl TranslationsIOS {
    fn escape(translation: &str) -> String {
        let out_value = translation
            .replace("%s", "%@")
            .replace("%d", "%@")
            .replace("%c", "%@")
            .replace("\"", "\\\"");
        out_value
    }
}

#[cfg(test)]
mod tests {
    use crate::ios_generator::TranslationsIOS;

    #[test]
    fn escape_ios_string() {
        let escaped = TranslationsIOS::escape("Hello %s, \"%d,\" %c%@");
        assert_eq!(escaped, "Hello %@, \\\"%@,\\\" %@%@");
    }
}