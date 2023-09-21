use std::{collections::HashMap, fs};
use crate::tokenizer::LexerSettings;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Instruction {
    SwapBuffer,
    SkipNext,
    ExpandPrevious,
    CrazySwap,
    JoinValues,
    EndLine,
    DereferenceLast,
    IfLess
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Object {
    Text(String),
    Keyword(Instruction)
}

pub struct Variables<'a> {
    vars: HashMap<String, Object>,
    cached_name: String,
    cached_value: String,
    file_path: String,
    lexer_settings: &'a LexerSettings,
    sysvar_prefix: String
}

impl<'a> Variables<'a> {
    pub fn new(lexer_settings: &'a LexerSettings) -> Self {
        Variables {
            vars: HashMap::new(),
            cached_name: String::new(),
            cached_value: String::new(),
            file_path: String::new(),
            lexer_settings,
            sysvar_prefix: String::from("$")
        }
    }

    pub fn get(&self, key: &str) -> Object {
        if key.starts_with( &self.sysvar_prefix ) {
            let sysvar_prefix_length = self.sysvar_prefix.chars().count();
            let var_without_prefix = key.chars().skip( sysvar_prefix_length ).collect::<Vec<_>>();
            match var_without_prefix[..] {
                [':'] => Object::Keyword( Instruction::SwapBuffer ),
                ['"'] => Object::Keyword( Instruction::SkipNext ),
                ['!'] => Object::Keyword( Instruction::ExpandPrevious ),
                ['?'] => Object::Keyword( Instruction::CrazySwap ),
                [','] => Object::Keyword( Instruction::JoinValues ),
                [';'] => Object::Keyword( Instruction::EndLine ),
                ['&'] => Object::Keyword( Instruction::DereferenceLast ),
                ['<'] => Object::Keyword( Instruction::IfLess ),
                ['n'] => Object::Text( self.cached_name.clone() ),
                ['v'] => Object::Text( self.cached_value.clone() ),
                ['i'] => panic!("input is not yet implemented"),
                ['I'] => panic!("non blocking input is not yet implemented"),
                ['p'] => Object::Text( self.file_path.clone() ),
                ['f'] => Object::Text( fs::read_to_string(&self.file_path).unwrap_or(String::new()) ),
                ['u'] => Object::Text( self.lexer_settings.get_uniques() ),
                ['s'] => Object::Text( self.lexer_settings.get_separators() ),
                ['S'] => Object::Text( self.lexer_settings.get_primary_sep() ),
                ['$'] => Object::Text( self.sysvar_prefix.clone() ),
                _ => Object::Text( String::new() )
            }
        } else {
            match self.vars.get(key) {
                Some(value) => value.to_owned(),
                None => Object::Text( String::new() )
            }
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.cached_name = key.clone();
        self.cached_value = value.clone();
        if key.starts_with( &self.sysvar_prefix ) {
            let sysvar_prefix_length = self.sysvar_prefix.chars().count();
            let var_without_prefix = key.chars().skip( sysvar_prefix_length ).collect::<Vec<_>>();
            match var_without_prefix[..] {
                ['o'] => println!("{}", value),
                ['c'] => panic!("running console commands is not yet implemented"),
                ['p'] => self.file_path = value,
                ['f'] => panic!("setting file contents is not yet implemented"),
                ['u'] => self.lexer_settings.set_uniques(value),
                ['s'] => self.lexer_settings.set_separators(value),
                ['S'] => self.lexer_settings.set_primary_sep(value),
                ['$'] => self.sysvar_prefix = value,
                _ => ()
            }
        } else {
            self.vars.insert(key, Object::Text(value));
        }
    }

    pub fn set_object(&mut self, key: String, value: Object) {
        if key.starts_with( &self.sysvar_prefix ) {
            return;
        }
        self.vars.insert(key, value);
    }
}
