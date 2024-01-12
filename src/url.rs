pub trait Url {
    fn sub_path(&self) -> &str;
    fn url_path(&self) -> &str;
}
