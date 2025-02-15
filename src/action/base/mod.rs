//! Base [`Action`](crate::action::Action)s that themselves have no other actions as dependencies

mod create_directory;
mod create_file;
mod create_group;
mod create_or_append_file;
mod create_user;
mod fetch_and_unpack_nix;
mod move_unpacked_nix;
mod setup_default_profile;

pub use create_directory::CreateDirectory;
pub use create_file::CreateFile;
pub use create_group::CreateGroup;
pub use create_or_append_file::CreateOrAppendFile;
pub use create_user::CreateUser;
pub use fetch_and_unpack_nix::{FetchAndUnpackNix, FetchUrlError};
pub use move_unpacked_nix::{MoveUnpackedNix, MoveUnpackedNixError};
pub use setup_default_profile::{SetupDefaultProfile, SetupDefaultProfileError};
