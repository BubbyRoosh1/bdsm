use rargsxd::*;
use std::fs::read_to_string;

fn main() {
    let mut parser = ArgParser::new("bdsm");
    parser.author("BubbyRoosh")
        .version("0.1.0")
        .info("Bubby's Declarative System Manager")
        .args(
            vec!(
                Arg::new("config")
                    .short('c')
                    .help("Location of the config file")
                    .option("/etc/bdsm.yml"),
            )
        ).parse();

    let result_contents = read_to_string(parser.get_option("config").unwrap());
    match result_contents {
        Ok(contents) => {
            if let Err(e) = bdsm::run(contents) {
                eprintln!("Error applying configs: {}", e);
            }
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
