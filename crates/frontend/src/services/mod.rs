/// Services for API communication and state management.
pub mod api_client;
pub mod geolocation;
pub mod map_service;

pub use api_client::ApiClient;
// Unused map service functions - kept for future use
// pub use map_service::{init_map, update_marker};
