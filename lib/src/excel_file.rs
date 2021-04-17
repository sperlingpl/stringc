use calamine::{Xlsx, Reader, XlsxError};
use std::io::BufReader;
use std::fs::File;

pub trait EFile {
    fn rows(&mut self) -> Vec<Vec<String>>;
    fn columns(&self) -> Vec<String>;
}

pub struct ExcelFile {
    workbook: Xlsx<BufReader<File>>
}

impl EFile for ExcelFile {
    fn rows(&mut self) -> Vec<Vec<String>> {
        let worksheet = self.workbook.worksheets()
            .first()
            .expect("cannot get worksheet")
            .clone();

        let mut rows: Vec<Vec<String>> = vec![];
        for xlsx_row in worksheet.1.rows() {
            let mut row = vec![];

            for column in 0..xlsx_row.len() {
                row.push(xlsx_row[column].to_string());
            }

            rows.push(row);
        }

        return rows;
    }

    fn columns(&self) -> Vec<String> {
        todo!()
    }
}

impl ExcelFile {
    pub fn new(file_name: &str) -> Result<ExcelFile, XlsxError> {
        let workbook = calamine::open_workbook(file_name)?;
        let excel_file = ExcelFile { workbook };
        Ok(excel_file)
    }
}