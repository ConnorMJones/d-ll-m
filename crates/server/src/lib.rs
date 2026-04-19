#[cfg(not(target_arch = "wasm32"))]
pub mod connection;
#[cfg(not(target_arch = "wasm32"))]
pub mod dnd5e;
pub mod reducers;
pub mod tables;
#[cfg(not(target_arch = "wasm32"))]
pub use connection::connect;
