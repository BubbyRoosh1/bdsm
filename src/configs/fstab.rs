use super::*;

#[derive(Debug, Clone)]
struct DriveConf {
    /// Name of the device e.g. "/dev/sda1"
    dev: String,
    /// Where the device is mounted e.g. "/boot/efi"
    mountpoint: String,
    /// The format of the partition e.g. "vfat"
    format: String,
    /// Rules for the device e.g. "defaults"
    rules: String,
    /// Dump/pass e.g. "0 0"
    dump: String,
}

impl DriveConf {
    fn new() -> Self {
        Self {
            dev: String::new(),
            mountpoint: String::new(),
            format: String::new(),
            rules: String::new(),
            dump: String::new(),
        }
    }

    fn check(&self) -> bool {
        self.dev != "" && self.mountpoint != "" && self.format != "" && self.rules != "" && self.dump != ""
    }
}

// Is this how you capitalize fstab...?
pub struct FSTabConf {
    drives: Vec<DriveConf>,
}

impl FSTabConf {
    pub fn new() -> Self {
        Self {drives: Vec::new()}
    }
}

impl config::EtcConf for FSTabConf {
    fn name(&self) -> &'static str {"fstab"}

    fn parse(&mut self, yaml: Yaml) -> Result<(), Box<dyn Error>> {
        if let Yaml::Hash(hash) = yaml {
            for drive in hash {
                let mut drive_conf = DriveConf::new();
                if let Yaml::String(s) = drive.0 {
                    drive_conf.dev = s;
                }
                if let Yaml::Hash(hash) = drive.1 {
                    if let Some(Yaml::String(s)) = hash.get(&Yaml::String("mountpoint".to_string())) {
                        drive_conf.mountpoint = s.to_string();
                    }
                    if let Some(Yaml::String(s)) = hash.get(&Yaml::String("format".to_string())) {
                        drive_conf.format = s.to_string();
                    }
                    if let Some(Yaml::String(s)) = hash.get(&Yaml::String("rules".to_string())) {
                        drive_conf.rules = s.to_string();
                    }
                    if let Some(Yaml::String(s)) = hash.get(&Yaml::String("dump".to_string())) {
                        drive_conf.dump = s.to_string();
                    }
                }
                self.drives.push(drive_conf);
            }
        }
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        let mut contents = String::new();
        for drive in self.drives.iter() {
            if drive.check() {
                contents.push_str(&format!("{}\t{}\t{}\t{}\t{}\n", 
                    drive.dev, 
                    drive.mountpoint, 
                    drive.format, 
                    drive.rules,
                    drive.dump
                ));
            }
        }
        write("etc/fstab", contents)?;
        Ok(())
    }
}
