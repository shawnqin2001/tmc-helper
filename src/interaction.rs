use std::io::{self, Error, Write};
pub fn get_user_action() -> Result<u8, Error> {
    println!("\n What would you like to do?");
    println!("0. Exit");
    println!("1. Initialize / Check Environment and Tools");
    println!("2. Install Pod");
    println!("3. List Pod and Website Address");
    println!("4. Login Pod in Terminal");
    println!("5. Uninstall Pod");
    print!("Enter action: ");

    io::stdout().flush()?;

    let mut action = String::new();
    io::stdin().read_line(&mut action)?;
    match action.trim().parse::<u8>() {
        Ok(num) => Ok(num),
        Err(_) => Err(Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid input, please enter a number",
        )),
    }
}
