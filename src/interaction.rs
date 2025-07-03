use std::io::{self, Error, Write};

pub fn get_user_action() -> Result<u8, Error> {
    println!("\nWhat would you like to do?");
    println!("0. Exit");
    println!("1. Initialize / Check Environment and Tools");
    println!("2. List Pod and Website Address");
    println!("3. Install Pod");
    println!("4. Login Pod in the Terminal");
    println!("5. Uninstall Pod");
    println!("6. Update User info");
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
