use crate::environment;
use crate::host_handler;
use crate::utils;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
pub struct PodConfig {
    container_name: String,
    cpu: Option<u8>,
    memory: Option<u8>,
}

impl PodConfig {
    pub fn new() -> Self {
        let mut container_name = String::new();
        loop {
            container_name.clear();
            println!("Please input the pods' name (only lowercase letters and numbers allowed):");
            io::stdin()
                .read_line(&mut container_name)
                .expect("Failed to read line");
            container_name = container_name.trim().to_string();
            if container_name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                && !container_name.is_empty()
            {
                break;
            } else {
                println!(
                    "Invalid input. Please enter a valid name,
                    only lowercase letters and numbers are allowed."
                );
            }
        }
        let mut cpu = String::new();
        println!("Please input the CPU limit (in cores, default: 32):");
        io::stdin()
            .read_line(&mut cpu)
            .expect("Failed to read line");
        let cpu = if cpu.trim().is_empty() {
            None
        } else {
            match cpu.trim().parse::<u8>() {
                Ok(cpu) => Some(cpu),
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    None
                }
            }
        };

        let mut memory = String::new();
        println!("Please input the memory limit (in GB, default: 50):");
        io::stdin()
            .read_line(&mut memory)
            .expect("Failed to read line");
        let memory = if memory.trim().is_empty() {
            None
        } else {
            match memory.trim().parse::<u8>() {
                Ok(memory) => Some(memory),
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    None
                }
            }
        };
        PodConfig {
            container_name,
            cpu,
            memory,
        }
    }
    fn get_cpu(&self) -> u8 {
        self.cpu.unwrap_or(32)
    }
    fn get_memory(&self) -> u8 {
        self.memory.unwrap_or(50)
    }
    pub fn save_config_yaml(&self) -> io::Result<()> {
        let user_info = environment::UserInfo::load().unwrap();
        let yaml_content = format!(
            r#"replicaCount: 1

image:
  repository: base.med.thu/public/rstudio
  pullPolicy: Always
  tag: "v1"

containerName: "{container_name}"

service:
  type: ClusterIP
  port: 8787

resources:
  limits:
    cpu: "{cpu}"
    memory: "{memory}"

imageCredentials:
  registry: base.med.thu
  username: {username}
  password: {password}

loadDataPath:
  public:
    - "input"
    - "lessonPublic"
  personal:
    - "{username}"

type: centos

nfs: "Aries"

transfer: false
        "#,
            container_name = self.container_name,
            cpu = self.get_cpu(),
            memory = self.get_memory(),
            username = user_info.user,
            password = user_info.password
        );
        let config_dir = env::current_dir()?.join("config");
        fs::create_dir_all(&config_dir)?;
        let file_path = config_dir.join(format!("{}.yaml", self.container_name));
        let mut file = fs::File::create(&file_path)?;
        file.write_all(yaml_content.as_bytes())?;
        println!("Configuration saved to {}", file_path.display());
        Ok(())
    }
    pub fn install_pod(&self) {
        let config_dir = env::current_dir().unwrap().join("config");
        let file_path = config_dir.join(format!("{}.yaml", self.container_name));
        if !file_path.exists() {
            eprintln!("Configuration file not found: {}", file_path.display());
            return;
        }
        let output = Command::new("helm")
            .args(&[
                "install",
                &self.container_name,
                "med-helm/alpha",
                "-f",
                &file_path.to_string_lossy(),
            ])
            .output()
            .expect("Failed to install pod");
        if !output.status.success() {
            eprintln!(
                "Error installing pod: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        match host_handler::HostsFile::new() {
            Ok(mut host_file) => {
                let hostname = format!("{}.app.med.thu", self.container_name);
                match host_file.add_entry(
                    "166.11.153.65",
                    &[&hostname],
                    Some("Added by thumed_login"),
                ) {
                    Ok(_) => {
                        println!("Hostname {} added to hosts file.", hostname);
                    }
                    Err(e) => {
                        eprintln!(
                            "Error adding hostname to hosts file: {}.\n
                            You may need to add {} manually.",
                            e, hostname
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Error creating hosts file: {}. \n
                    You may need to manually add host:\n166.111.153.65 {}",
                    e, self.container_name
                );
            }
        }
    }
}

pub struct PodList {
    pub pod_list: Vec<String>,
}

impl PodList {
    pub fn new() -> Self {
        PodList {
            pod_list: Vec::new(),
        }
    }
    pub fn get_pod_list(&mut self) {
        let stdout = utils::run_cmd("kubectl", &["get", "pods"]).unwrap();
        let lines: Vec<&str> = stdout.lines().collect();
        let mut pod_list = Vec::new();
        for line in lines.iter().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                pod_list.push(parts[0].to_string());
            }
        }
        self.pod_list = pod_list;
    }

    pub fn display(&self) {
        println!("Pods:");
        for pod in &self.pod_list {
            let website = pod.split('-').collect::<Vec<&str>>()[0].to_string() + ".app.med.thu";
            println!("Pod ID: {}, Website: {}", pod, website);
        }
    }
    pub fn login_pod(&self) {}
    pub fn uninstall_pod(&self) {
        println!("Please input the pod name you want to uninstall:");
        let mut pod_name = String::new();
        io::stdin()
            .read_line(&mut pod_name)
            .expect("Failed to read line");
        let output = Command::new("helm")
            .args(&["uninstall", &pod_name])
            .output()
            .expect("Failed to uninstall pod");
        if output.status.success() {
            println!("Pod uninstalled successfully.");
        } else {
            eprintln!(
                "Error uninstalling pod: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
