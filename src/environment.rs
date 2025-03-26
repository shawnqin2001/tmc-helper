use std::env; 
use std::error::Error;

pub fn get_os() -> Result<String, Box<dyn Error>> {
    let os:String = env::consts::OS.to_string();
    Ok(os)
}

pub fn get_home_dir() -> Result<String, Box<dyn Error>> {
    let home_dir:String = env::var("HOME")?;
    Ok(home_dir)
}

pub fn command_runner() {}

pub fn package_available() {}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn os_right() {
        let os:String = String::from("linux");
        assert_eq!(os, get_os().unwrap())
    }

    #[test]
    #[should_panic]
    fn home_right() {
        let dir = String::from("C\\User");
        assert_eq!(dir, get_home_dir().unwrap())
    }
}
