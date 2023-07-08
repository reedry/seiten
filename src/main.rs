use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
struct Config {
    build: Build,
}

#[derive(Deserialize)]
struct Build {
    input: String,
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = std::fs::read_to_string("seiten.toml")?;
    let config: Config = toml::from_str(&config_file)?;
    println!(
        "input: {}, output: {}",
        config.build.input, config.build.output
    );

    let md_input = std::fs::read_to_string(config.build.input)?;
    let parser = pulldown_cmark::Parser::new(&md_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let mut output = std::fs::File::create(config.build.output)?;
    writeln!(output, "{}", html_output)?;

    Ok(())
}
