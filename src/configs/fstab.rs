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
                let mut conf = DriveConf::new();
                if let Yaml::String(s) = drive.0 {
                    conf.dev = s;
                }
                get_yaml(&mut conf.mountpoint, &drive.1, "mountpoint");
                get_yaml(&mut conf.format, &drive.1, "format");
                get_yaml(&mut conf.rules, &drive.1, "rules");
                get_yaml(&mut conf.dump, &drive.1, "dump");
                self.drives.push(conf);
            }
        }
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        println!("Populating fstab...");
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
