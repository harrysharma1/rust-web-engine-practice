mod dom;
mod parse;
mod css;

use parse::parse;

fn main() {
    let html:String = "<html><body>Hello, World!</body></html>".to_string();
    parse(html);
}
