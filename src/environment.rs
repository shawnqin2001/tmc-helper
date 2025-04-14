use crate::utils;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

pub struct UserInfo {
    pub user: String,
    pub password: String,
}

impl UserInfo {
    pub fn new(user: String, password: String) -> Self {
        UserInfo { user, password }
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_dir = env::current_dir()?.join("config");
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        let user_config_path = config_dir.join("user.config");

        if user_config_path.exists() {
            let mut file = File::open(&user_config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let lines: Vec<&str> = contents.lines().collect();
            if lines.len() >= 2 {
                let user = lines[0].to_string();
                let password = lines[1].to_string();
                Ok(UserInfo::new(user, password))
            } else {
                Err("Config file format err".into())
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
        let config_dir = env::current_dir()?.join("config");
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        let config_file = config_dir.join("user.config");
        let mut file = File::create(&config_file)?;
        writeln!(file, "{}", self.user)?;
        writeln!(file, "{}", self.password)?;
        Ok(())
    }
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
            utils::run_cmd(
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

    add_path(bin_dir.to_string_lossy().as_ref())?;

    let kubectl_path = bin_dir.join(if cfg!(windows) {
        "kubectl.exe"
    } else {
        "kubectl"
    });
    let helm_path = bin_dir.join(if cfg!(windows) { "helm.exe" } else { "helm" });

    let kubectl_exists = kubectl_path.exists();
    let helm_exists = helm_path.exists();

    if !kubectl_exists || !helm_exists {
        println!("Some required tools are missing. Will attempt to download them:");

        if !kubectl_exists {
            match utils::download_kubectl(&bin_dir) {
                Ok(_) => println!("Successfully downloaded kubectl"),
                Err(e) => println!(
                    "Failed to download kubectl: {}. Please download it manually.",
                    e
                ),
            }
        }

        if !helm_exists {
            match utils::download_helm(&bin_dir) {
                Ok(_) => println!("Successfully downloaded helm"),
                Err(e) => println!(
                    "Failed to download helm: {}. Please download it manually.",
                    e
                ),
            }
        }
    } else {
        println!("All required tools found in bin directory.");
    }

    // Re-check after potential downloads
    let kubectl_exists = kubectl_path.exists();
    let helm_exists = helm_path.exists();

    if kubectl_exists {
        let bin_kubectl = kubectl_path.to_string_lossy().to_string();
        match utils::run_cmd(&bin_kubectl, &["version", "--client"]) {
            Ok(_) => println!("kubectl is working correctly"),
            Err(e) => println!("Warning: kubectl may not be working: {}", e),
        }
    } else {
        println!("kubectl is still missing. Please download it manually from:");
        println!("kubectl: kubernetes.io/docs/tasks/tools/");
    }

    if helm_exists {
        let bin_helm = helm_path.to_string_lossy().to_string();
        match utils::run_cmd(&bin_helm, &["version"]) {
            Ok(_) => println!("helm is working correctly"),
            Err(e) => println!("Warning: helm may not be working: {}", e),
        }
    } else {
        println!("helm is still missing. Please download it manually from:");
        println!("helm: https://github.com/helm/helm/releases");
    }

    Ok(())
}

fn init_helm() -> Result<(), Box<dyn Error>> {
    // Check if med-helm repo already exists
    let helm_list = utils::run_cmd("helm", &["repo", "list"])?;

    if !helm_list.contains("med-helm") {
        let _helm_init = utils::run_cmd(
            "helm",
            &["repo", "add", "med-helm", "http://166.111.153.65:7001"],
        )?;
        println!("Added med-helm repository");
    } else {
        println!("med-helm repository already exists");
    }
    let helm_update = utils::run_cmd("helm", &["repo", "update"])?;
    println!("{}", helm_update);
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
    match init_helm() {
        Ok(_) => println!("Helm initialized successfully"),
        Err(e) => println!("Error initializing helm: {}", e),
    }
    println!("Environment check completed!");
}
