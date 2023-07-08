use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = args().nth(1).expect("No input");
    let md_input = std::fs::read_to_string(input_file)?;
    let parser = pulldown_cmark::Parser::new(&md_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    println!("{}", html_output);
    Ok(())
}
