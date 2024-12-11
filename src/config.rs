pub use crate::db::Config as Db;

#[derive(Clone, Debug)]
pub struct Config {
    pub db: crate::db::Config,
}
