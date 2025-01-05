//! `definitions` are the API responses as-is for types.
//!
//! They are not meant to be used directly, but rather
//! to be transformed into more usable data and
//! that's the job of the `parsers` module.
//!
//! Some types can still be used directly and exposed publicly
//! when the structure is simple enough and easy to read.

mod response;
pub use response::{ErrorResponse, RedirectResponse};

mod user_status;
pub use user_status::{UserStatus, UserStatusResponse};

mod semester;
pub use semester::Semester;

mod config;
pub use config::Config;

mod summary;
pub use summary::{Summary, SummaryStudent};

mod first_authentication_data;
pub use first_authentication_data::FirstAuthenticationDataResponse;
