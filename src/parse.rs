
struct Parser{
    pos: usize,
    input: String,
}

impl Parser{
    fn next_char(&self)->char{
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, check: &str)->bool{
        self.input[self.pos..].starts_with(check)
    } 

    fn eof(&self)->bool{
        self.pos>self.input.len()
    } 

    
}