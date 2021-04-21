use super::*;
use std::fs::{OpenOptions, read_to_string};
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
                let mut conf = UserConf::new();
                if let Yaml::String(s) = user.0 {
                    conf.name = s;
                }
                get_yaml(&mut conf.shell, &user.1, "shell");
                get_yaml(&mut conf.perms, &user.1, "perms");
                self.users.push(conf);
            }
        }
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        println!("Adding users...");
        for user in self.users.clone() {
            let mut skip = false;
            let passwd = read_to_string("/etc/passwd")?;
            for line in passwd.lines() {
                let name = line.split(":").nth(0).unwrap();
                if user.name == name {skip = true}
            }
            if skip {continue}

            println!("Adding user {}", user.name);

            // FIXME: Proper stuff for this that isn't std::process::Command. Only works with
            // busybox cause busybox uses adduser.
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
