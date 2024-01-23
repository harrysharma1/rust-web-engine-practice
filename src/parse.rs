
struct Parser{
    position: usize,
    input: String,
}

impl Parser{
    fn next_char(&self)->char{
        self.input[self.position..].chars().next().unwrap()
    }

    fn starts_with(&self, check: &str)->bool{
        self.input[self.position..].starts_with(check)
    } 

    fn eof(&self)->bool{
        self.position>self.input.len()
    } 

    fn consume_character(&mut self)->char{
        let mut iterator = self.input[self.position..].char_indices();
        let (_,current_character) = iterator.next().unwrap();
        let (next_position,_) = iterator.next().unwrap_or((1,' '));
        self.position+=next_position;
        return current_character;
    }

    
}