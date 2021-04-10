pub mod json_data;
pub mod ios_generator;
pub mod strings_generator;
pub mod excel_writer;
pub mod excel_reader;
pub mod excel_file;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
