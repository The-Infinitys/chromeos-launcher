use chromeos_launcher::utils::error::Error;
use chromeos_launcher::utils::shell;
use clap::Parser;
use chromeos_launcher::modules::app::App;
fn main() -> Result<(), Error> {
    let args = shell::Args::parse();
    let app = App::from(args);
    app.exec()
}
