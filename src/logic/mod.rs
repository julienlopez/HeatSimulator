//! The logic module contains all business logic for our app.
//! This includes data processing, state management, and any other non-UI related functionality.

mod infrastructure;
pub use infrastructure::Infrastructure;

pub mod solver;

pub mod state;
pub use state::State;
