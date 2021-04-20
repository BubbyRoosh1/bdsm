use super::*;

pub trait EtcConf {
    /// The name to check for in bdsm_conf.yml
    fn name(&self) -> &'static str;
    /// Parse the values given the vec of yaml; only passed what's in the hash
    fn parse(&mut self, yaml: Yaml) -> Result<(), Box<dyn Error>>;
    /// Write the config to the file
    fn write(&self) -> Result<(), Box<dyn Error>>;
}
