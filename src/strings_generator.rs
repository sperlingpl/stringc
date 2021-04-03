pub trait Generator {
    fn generate(&self) -> std::io::Result<()>;
}
