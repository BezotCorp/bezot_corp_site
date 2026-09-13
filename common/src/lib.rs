mod commande_mode;
mod errors;
mod json_manager;

pub use commande_mode::CommandMode;
pub use errors::{invalid_data, invalid_input};
pub use json_manager::read_json;
