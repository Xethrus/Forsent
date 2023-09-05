use std::io;
use anyhow::Result;
use anyhow::Context;

enum DataSource {
    Local,
    Api,
}

struct Config {
    source: DataSource,
}

fn obtain_intention(config: &mut Config) -> Result<()> {
    loop {
        println!("Select an input data form:");
        println!("1. local");
        println!("2. api");
        println!("3. exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .context("Failed to read from stdin")?;

        match input.trim() {
            "1" => handle_data_source(DataSource::Local, config),
            "2" => handle_data_source(DataSource::Api, config),
            "3" => break,
            _ => println!("Invalid option selected, try again."),
        }
    }
    Ok(())
}

fn handle_data_source(source: DataSource, config: &mut Config) {
    match source {
        DataSource::Local => config.source = DataSource::Local,
        DataSource::Api => config.source = DataSource::Api,
    }
}

fn main() {
    let mut config = Config {
        source: DataSource::Local, // default to Local
    };

    if let Err(e) = obtain_intention(&mut config) {
        eprintln!("An error occurred: {}", e);
    }

    println!("Data source selected: {:?}", config.source);
}

