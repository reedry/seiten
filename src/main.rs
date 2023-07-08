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

    let input_dir = std::env::current_dir()?.join(config.build.input);
    let output_dir = std::env::current_dir()?.join(config.build.output);
    println!(
        "input: {}, output: {}",
        input_dir.display(),
        output_dir.display()
    );

    let md_input = std::fs::read_to_string(input_dir)?;
    let parser = pulldown_cmark::Parser::new(&md_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let mut output = std::fs::File::create(output_dir)?;
    writeln!(output, "{}", html_output)?;

    Ok(())
}
