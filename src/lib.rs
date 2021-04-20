use yaml_rust::*;
use std::error::Error;

pub mod config;
use config::*;
pub mod configs;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let mut etc_confs: Vec<Box<dyn EtcConf>> = vec!(
        Box::new(configs::fstab::FSTabConf::new()),
        Box::new(configs::hostname::HostnameConf::new()),
        Box::new(configs::users::UsersConf::new()),
    );

    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = &docs[0];
    for conf in etc_confs.iter_mut() {
        if !doc[conf.name()].is_badvalue() {
            conf.parse(doc[conf.name()].clone())?;
            conf.write()?;
        }
    }
    Ok(())
}
