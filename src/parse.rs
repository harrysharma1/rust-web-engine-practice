use crate::dom;

struct Parser{
    position: usize,
    input: String,
}

impl Parser{
    fn next_character(&self)->char{
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

    fn consume_while<F>(&mut self, test: F) -> String
    where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.eof() && test(self.next_character()) {
                result.push(self.consume_character());
            }
            return result;
    }

    fn consume_whitespace(&mut self){
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag(&mut self)->String{
        self.consume_while(|x|  match x  {
            'a'..='z'|'A'..='Z'|'0'..='9'=> true,
            _=>false
        })    
    }

}

    
