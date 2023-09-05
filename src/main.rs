use std::io;

enum dataSource {
    local,
    api,
}

struct Config {
    source: dataSource,
}

fn obtainIntention() -> Result<()> {
    loop{
        println!("select a input data form:");
        println!("1. local");
        println!("2. api");
        println!("3. exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => handleDataSource(dataSource::local),
            "2" => handleDataSource(dataSource::api),
            "3" => break,
            _ => println!("invalid option selected, try again."),
        }
    }
}

fn handleDataSource(dataSource: source, Config: configuration) -> result<Config> {
    match source {
        dataSource::local => configuration::source = dataSource::local;//do something;
        dataSource::api => configuration::source = dataSource::api;//do something else
    }
}

fn main() {
    println!("Hello, world!");
}

