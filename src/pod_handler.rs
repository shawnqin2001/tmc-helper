use std::fs;
use std::io::{Error, Write};
use std::process::Command;

use crate::host_handler;
use crate::interaction;

#[derive(Debug)]
pub struct PodConfig {
    container_name: String,
}

impl PodConfig {
    pub fn new(container_name: String) -> Self {
        PodConfig { container_name }
    }
    pub fn install_pod(&self) {}
    pub fn uninstall_pod(&self) {}
    pub fn show_website(&self) {}
}

pub fn list_pods() {}
