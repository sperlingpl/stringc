use calamine::{Xlsx, Reader, XlsxError};
use std::io;
use std::io::BufReader;
use std::fs::File;

pub trait EFile {
    fn rows(&self) -> Vec<Vec<String>>;
    fn columns(&self) -> Vec<String>;
}

pub struct ExcelFile {
    workbook: Xlsx<BufReader<File>>
}

impl EFile for ExcelFile {
    fn rows(&self) -> Vec<Vec<String>> {
        todo!()
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