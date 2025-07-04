// src/modules/app.rs

use crate::utils::error::Error;
use crate::utils::shell::{Args, SubCommands};
pub struct App {
    args: Args,
}
impl From<Args> for App {
    fn from(value: Args) -> Self {
        App { args: value }
    }
}
impl App {
    pub fn exec(self) -> Result<(), Error> {
        match self.args.sub_command {
            SubCommands::Run => {
                
            }
            SubCommands::List => {}
            SubCommands::New => {}
            SubCommands::Remove => {}
        }
        Ok(())
    }
}
