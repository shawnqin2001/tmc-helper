mod environment;
mod host_handler;
mod interaction;
mod pod_handler;
mod utils;

fn main() {
    let os = environment::get_os().unwrap();
    println!("Welcome to THU-Med Login Helper");
    println!("Current: Lecture version");
    loop {
        match interaction::get_user_action() {
            Ok(action) => match action {
                0 => break,
                1 => environment::check_env(),
                _ => println!("Invalid action"),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
