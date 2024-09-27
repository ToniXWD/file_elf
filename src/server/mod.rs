pub mod api;

#[cfg(feature = "webserver")]
mod server;

#[cfg(feature = "webserver")]
pub use server::init_route;
