use chromeos_launcher::utils::error::Error;
fn main() -> Result<(), Error> {
    let e = Error::from("Hello, World");
    println!("{}", e);
    Err(e)
}
