fn main() {
    let md_input = "Hello, world!";
    let parser = pulldown_cmark::Parser::new(md_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    println!("{}", html_output);
}
