use chromeos_launcher::modules::app::App;
use chromeos_launcher::utils::error::Error;
use chromeos_launcher::utils::shell;
use clap::Parser;
fn main() -> Result<(), Error> {
    let args = shell::Args::parse();
    let app = App::from(args);
    app.exec()
}
