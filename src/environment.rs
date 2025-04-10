use crate::utils::run_cmd;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

struct UserInfo {
    user: String,
    password: String,
}

impl UserInfo {
    pub fn new(user: String, password: String) -> Self {
        UserInfo { user, password }
    }

    fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = env::current_dir()?.join("user.config");

        if config_path.exists() {
            let mut file = File::open(&config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let lines: Vec<&str> = contents.lines().collect();
            if lines.len() >= 2 {
                let user = lines[0].to_string();
                let password = lines[1].to_string();
                return Ok(UserInfo::new(user, password));
            } else {
                return Err("Config file format err".into());
            }
        } else {
            println!("No user configuration found, Please enter credentials:");
            print!("Username: ");
            io::stdout().flush()?;
            let mut user = String::new();
            io::stdin().read_line(&mut user)?;
            let user = user.trim().to_string();
            print!("Password: ");
            io::stdout().flush()?;
            let mut password = String::new();
            io::stdin().read_line(&mut password)?;
            let password = password.trim().to_string();

            let user_info = UserInfo::new(user, password);
            user_info.save()?;
            Ok(user_info)
        }
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_file = env::current_dir()?.join("user.config");
        let mut file = File::create(&config_file)?;
        writeln!(file, "{}", self.user)?;
        writeln!(file, "{}", self.password)?;
        Ok(())
    }
}

pub fn get_os() -> Result<String, Box<dyn Error>> {
    let os: String = env::consts::OS.to_string();
    Ok(os)
}

// Get Current Directory
pub fn get_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = env::current_dir()?;
    Ok(dir)
}

// Add Path to Environment Variable
pub fn add_path(path: &str) -> Result<(), Box<dyn Error>> {
    let path_str = path.to_string();
    let paths = env::var("PATH")?;
    let mut path_vec: Vec<String> = env::split_paths(&paths)
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    if !path_vec.contains(&path_str) {
        path_vec.insert(0, path_str);
        let new_path = env::join_paths(path_vec)?;
        if cfg!(target_os = "windows") {
            run_cmd(
                "setx",
                &["path", new_path.to_string_lossy().to_string().as_str()],
            )?;
        } else {
            unsafe {
                env::set_var("PATH", new_path);
            }
        };
    };
    Ok(())
}

pub fn ensure_tools_available() -> Result<(), Box<dyn Error>> {
    let bin_dir = std::env::current_dir()?.join("bin");

    if !bin_dir.exists() {
        println!("Creating bin directory...");
        std::fs::create_dir_all(&bin_dir)?;
    }

    add_path(&bin_dir.to_string_lossy().to_string())?;

    let kubectl_path = bin_dir.join(if cfg!(windows) {
        "kubectl.exe"
    } else {
        "kubectl"
    });
    let helm_path = bin_dir.join(if cfg!(windows) { "helm.exe" } else { "helm" });

    let kubectl_exists = kubectl_path.exists();
    let helm_exists = helm_path.exists();

    if !kubectl_exists || !helm_exists {
        println!("Warning: Some required tools are missing in bin directory:");

        if !kubectl_exists {
            println!("  - kubectl is missing. Please place kubectl in the bin directory.");
        }

        if !helm_exists {
            println!("  - helm is missing. Please place helm in the bin directory.");
        }

        println!("You can download these tools from their official websites:");
        println!("  kubectl: https://kubernetes.io/docs/tasks/tools/");
        println!("  helm: https://helm.sh/docs/intro/install/");
    } else {
        println!("All required tools found in bin directory.");
    }

    if kubectl_exists {
        let bin_kubectl = kubectl_path.to_string_lossy().to_string();
        match run_cmd(&bin_kubectl, &["version"]) {
            Ok(_) => println!("kubectl is working correctly"),
            Err(e) => println!("Warning: kubectl may not be working: {}", e),
        }
    }

    if helm_exists {
        let bin_helm = helm_path.to_string_lossy().to_string();
        match run_cmd(&bin_helm, &["version"]) {
            Ok(_) => println!("helm is working correctly"),
            Err(e) => println!("Warning: helm may not be working: {}", e),
        }
    }

    Ok(())
}

pub fn check_env() {
    println!("Checking environment...");
    match UserInfo::load() {
        Ok(user_info) => {
            println!("User: {}", user_info.user);
        }
        Err(e) => {
            println! {"Error loading user info: {}", e};
            return;
        }
    }

    match ensure_tools_available() {
        Ok(_) => println!("Tool directory setup complete"),
        Err(e) => println!("Error setting up tool directory: {}", e),
    }

    println!("Environment check completed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_right() {
        let os: String = String::from("linux");
        assert_eq!(os, get_os().unwrap())
    }
}
