pub mod cache;
pub mod hot_dir;
pub mod trie;

use tokio::sync::Mutex;

pub use cache::Cacher;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CACHER: Mutex<Cacher> = Mutex::new(Cacher::new());
}
