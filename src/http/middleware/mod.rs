// JsonDump should only be used in debug mode
#[cfg(debug_assertions)]
pub use json_dump::JsonDump;
#[cfg(not(debug_assertions))]
pub type JsonDump<T> = axum::Json<T>;

#[cfg(debug_assertions)]
mod json_dump;

pub mod upgrade;
pub mod user;
