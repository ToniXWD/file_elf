pub mod cache;
pub mod trie;

use std::sync::Mutex;

pub use cache::Cacher;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CACHER: Mutex<Cacher> = Mutex::new(Cacher::new());
}
