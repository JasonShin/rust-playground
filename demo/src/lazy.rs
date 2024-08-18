use lazy_static::lazy_static;
use std::{sync::Arc};
use tokio::{sync::Mutex};
use crate::info::DBSchema;

lazy_static! {
    pub static ref HMM: i32 = 2;
    pub static ref AAAAAAAAA: Arc<Mutex<DBSchema>> = Arc::new(Mutex::new(DBSchema::new()));
}
