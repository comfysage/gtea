use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOME: String = std::env::var("HOME").unwrap();
    pub static ref CWD: String = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    pub static ref CONFIG_NAME: String = "gtea.toml".to_string();
}
