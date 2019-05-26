mod me;
pub use me::Me;

mod help;
pub use help::Help;

use crate::application;

pub struct CommandBus {
}

impl CommandBus {
    pub fn executeCommand(command: impl application::Command) {
    }
}
