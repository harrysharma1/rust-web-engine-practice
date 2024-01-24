
struct Stylesheet{
    rules:Vec<Rule>,
}

struct Rule{
    selector: Vec<Selector>,
    declaration: Vec<Declaration>,

}

enum Selector{
    Simple(SimpleSelector),
}

struct SimpleSelector{
    tag: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration{
    name: String,
    value: Value,
}

enum Value{
    Keyword(String),
    Length(f32,Unit),
    ColorValue(Colour),
}

enum Unit{
    px,
    em,
    pc,
}

struct Colour{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}