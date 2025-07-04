use crate::utils::error::Error;
use crate::utils::shell::{Args, SubCommands, is_available};

pub mod list;
pub mod new;
pub mod recover;
pub mod remove;
pub mod run;

pub struct App {
    args: Args,
}

impl App {
    pub fn from(args: Args) -> Self {
        Self { args }
    }
    pub fn exec(&self) -> Result<(), Error> {
        match &self.args.sub_command {
            SubCommands::Run(run_command) => {
                if !is_available("qemu-system-x86_64") {
                    println!("qemu-system-x86_64 is not installed.");
                    return Ok(());
                }
                run_command.exec()?;
            }
            SubCommands::List => {
                list::list()?;
            }
            SubCommands::New(new_command) => {
                new_command.exec()?;
            }
            SubCommands::Remove(remove_command) => {
                remove_command.exec()?;
            }
            SubCommands::Recover(recover_command) => {
                recover_command.exec()?;
            }
        }
        Ok(())
    }
}
