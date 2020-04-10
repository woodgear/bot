pub mod bot_client;
pub mod bot_fs;
pub mod bot_shell;
pub mod bot_tail;
pub mod util;
pub use bot::protocol;
pub mod prelude {
    pub use crate::bot_client::*;
    pub use crate::bot_fs::*;
    pub use crate::bot_shell::*;
    pub use crate::bot_tail::*;
    pub use crate::util;
}
