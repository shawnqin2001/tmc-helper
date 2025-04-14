mod environment;
mod host_handler;
mod interaction;
mod pod_handler;
mod utils;

fn main() {
    let mut pod_list = pod_handler::PodList::new();
    println!("Welcome to THU-Med Login Helper");
    println!("Current: Lecture version");
    loop {
        match interaction::get_user_action() {
            Ok(action) => match action {
                0 => break,
                1 => environment::check_env(),
                2 => {
                    pod_list.get_pod_list();
                    pod_list.display();
                }
                3 => {
                    let pod_config = pod_handler::PodConfig::new();
                    pod_config.save_config_yaml().unwrap();
                    pod_config.install_pod();
                }
                4 => {
                    pod_list.get_pod_list();
                    pod_list.display();
                    pod_list.login_pod();
                }
                5 => {
                    pod_list.get_pod_list();
                    pod_list.display();
                    pod_list.uninstall_pod();
                }
                _ => println!("Invalid action"),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
