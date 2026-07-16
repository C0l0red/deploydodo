use std::{fmt::Debug, str::FromStr, sync::Arc};

use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct Environment {
    pub database_url: String,
    pub local_ssh_hostname: String,
    pub local_ssh_port: u16,
    pub local_ssh_username: String,
    pub local_ssh_private_key: String,
}

static ENVIRONMENT: OnceCell<Arc<Environment>> = OnceCell::const_new();

pub fn get_env() -> Arc<Environment> {
    ENVIRONMENT
        .get()
        .expect("You must call env::init_env once before calling env::get_env")
        .clone()
}

pub fn init_env() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    ENVIRONMENT
        .set(Arc::new(Environment {
            database_url: read_env("DATABASE_URL"),
            local_ssh_hostname: read_env("LOCAL_SSH_HOSTNAME"),
            local_ssh_port: read_env("LOCAL_SSH_PORT"),
            local_ssh_username: read_env("LOCAL_SSH_USERNAME"),
            local_ssh_private_key: read_file_path_from_env("LOCAL_SSH_PRIVATE_KEY"),
        }))
        .expect("You should call env::init_env only once.")
}

fn read_env<T>(key: &str) -> T
where
    T: FromStr + Debug + TypeName,
    <T as FromStr>::Err: Debug,
{
    std::env::var(key)
        .map(|h| {
            h.parse::<T>()
                .unwrap_or_else(|_| panic!("{key} must be a valid {}", T::type_name()))
        })
        .unwrap_or_else(|_| panic!("The variable {key} must be present at runtime"))
}

fn read_file_path_from_env(key: &str) -> String {
    let path = read_env::<String>(key);

    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("The path stored in {key} ({path}) does not exist"))
}

trait TypeName {
    fn type_name() -> &'static str;
}

impl TypeName for u16 {
    fn type_name() -> &'static str {
        "u16"
    }
}

impl TypeName for u32 {
    fn type_name() -> &'static str {
        "u32"
    }
}

impl TypeName for String {
    fn type_name() -> &'static str {
        "String"
    }
}
