use super::*;

pub struct HostnameConf {
    /// System hostname; defaults to "localhost"
    hostname: String
}

impl HostnameConf {
    pub fn new() -> Self {
        Self {hostname: "localhost".to_string()}
    }
}

impl config::EtcConf for HostnameConf {
    fn name(&self) -> &'static str {"hostname"}

    fn parse(&mut self, yaml: Yaml) -> Result<(), Box<dyn Error>> {
        if let Yaml::String(s) = yaml {
            self.hostname = s;
        }
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        println!("Setting hostname...");
        write("etc/hostname", &self.hostname)?;
        Ok(())
    }
}
