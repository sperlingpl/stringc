use simple_excel_writer::*;

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

    pub fn generate(&self, file_name: &str) {
        let mut wb = Workbook::create(file_name);
        let mut sheet = wb.create_sheet("Localizations");

        // Key column.
        sheet.add_column(Column { width: 30.0 });

        // Values columns.
        for _ in &self.langs {
            sheet.add_column(Column { width: 50.0 });
        }

        let lang_count = self.langs.len();

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;

            self.build_headers(lang_count, sw);
            self.build_values(lang_count, sw);

            Ok(())
        }).expect("Cannot write Excel file!");

        wb.close().expect("Cannot close Excel file!");
    }

    fn build_values(&self, lang_count: usize, sw: &mut SheetWriter) {
        for value in &self.values {
            let mut cells: Vec<Cell> = vec![];
            let key_cell = Cell { value: CellValue::String(value.key.to_string()), column_index: 1 };
            cells.push(key_cell);

            for i in 0..lang_count {
                let cell = Cell { value: CellValue::String(value.values[i].to_string()), column_index: i + 2 };
                cells.push(cell);
            }

            let mut row = Row::new();
            row.cells = cells;
            sw.append_row(row);
        }
    }

    fn build_headers(&self, lang_count: usize, sw: &mut SheetWriter) {
        let mut cells: Vec<Cell> = vec![];
        let key_cell = Cell { value: CellValue::String("Key".to_string()), column_index: 1 };
        cells.push(key_cell);

        for i in 0..lang_count {
            let cell = Cell { value: CellValue::String(self.langs[i].to_string()), column_index: i + 2 };
            cells.push(cell);
        }

        let mut row = Row::new();
        row.cells = cells;
        sw.append_row(row);
    }
}

impl ExcelTranslation {
    pub fn new(key: String, values: Vec<String>) -> ExcelTranslation {
        ExcelTranslation { key, values }
    }
}