use super::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

#[derive(Debug, Clone)]
struct UserConf {
    /// The name of the user; defaults to "user"
    name: String,
    /// The path to the shell (/etc/passwd); defaults to "/bin/sh"
    shell: String,
    /// Sudo/Doas perms; "Sudo:<line>" for sudo; "Doas:<line>" for doas; defaults to "Doas:permit
    /// user as root"
    perms: String,
}

impl UserConf {
    fn new() -> Self {
        Self {
            name: "user".to_string(),
            shell: "/bin/sh".to_string(),
            perms: "Doas:permit user as root".to_string(),
        }
    }
}

pub struct UsersConf {
    users: Vec<UserConf>,
}

impl UsersConf {
    pub fn new() -> Self {
        Self {users: Vec::new()}
    }
}

impl config::EtcConf for UsersConf {
    fn name(&self) -> &'static str {"users"}

    fn parse(&mut self, yaml: Yaml) -> Result<(), Box<dyn Error>> {
        if let Yaml::Hash(hash) = yaml {
            for user in hash {
                let mut user_conf = UserConf::new();
                if let Yaml::String(s) = user.0 {
                    user_conf.name = s;
                }
                if let Yaml::Hash(hash) = user.1 {
                    if let Some(Yaml::String(s)) = hash.get(&Yaml::String("shell".to_string())) {
                        user_conf.shell = s.to_string();
                    }
                    if let Some(Yaml::String(s)) = hash.get(&Yaml::String("perms".to_string())) {
                        user_conf.perms = s.to_string();
                    }
                }
                self.users.push(user_conf);
            }
        }
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        for user in self.users.clone() {
            // FIXME: Proper stuff for this that isn't std::process::Command. Only works with
            // busybox cause busybox uses adduser.
            println!("Adding user {}", user.name);
            Command::new("adduser")
                .arg(user.name)
                .spawn()?
                .wait()?;

            let mut iter = user.perms.split(":");
            let perm_file = if iter.nth(0).unwrap() == "Sudo" {"etc/sudoers"} else {"etc/doas.conf"};
            let perm_string = iter.nth(0).unwrap();
            let mut file = OpenOptions::new()
                .append(true)
                .open(perm_file)?;

            writeln!(file, "{}", perm_string)?;
        }
        Ok(())
    }
}
