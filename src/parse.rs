use std::collections::HashMap;

use crate::dom::{self, Node};

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

    fn parse_node(&mut self)-> dom::Node{
        match  self.next_character() {
            '<' =>self.parse_element(),
            _ =>self.parse_text()
            
        }


    
    }

    fn parse_text(&mut self)->dom::Node{
        dom::text(self.consume_while(|c| c!='<'))
    }

    fn parse_element(&mut self)->dom::Node{
        assert!(self.consume_character()=='<');
        let tag_name = self.parse_tag();
        let tag_attribute = self.parse_attributes();
        assert!(self.consume_character()=='>');

        let children  = self.parse_nodes();

        assert!(self.consume_character() == '<');
        assert!(self.consume_character() == '/');
        assert!(self.parse_tag() == tag_name);
        assert!(self.consume_character() == '>');

        return dom::elem(tag_name, tag_attribute, children);

   
    }
    
    fn parse_attr(&mut self)->(String,String){
        let name = self.parse_tag();
        assert!(self.consume_character() == '=');
        let value = self.parse_attr_value();
        return (name,value);
    }

    fn parse_attr_value(&mut self)->String{
        let open_quote = self.consume_character();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_character() == open_quote);
        return value;
    }

    fn parse_attributes(&mut self)->dom::AttrMap{
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_character()=='>'{
                break;
            }
            let (name,value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_nodes(&mut self)->Vec<dom::Node>{
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node());
        }

        return  nodes;

    }

}

    
