use std::sync::Mutex;

pub struct LexerSettings {
    uniques: Mutex<String>,
    separators: Mutex<String>,
    primary_separator: Mutex<String>
}

impl Default for LexerSettings {
    fn default() -> Self {
        LexerSettings {
            uniques: String::new().into(),
            separators: String::from("\t\n\x0c\r ").into(),
            primary_separator: String::from(" ").into()
        }
    }
}

impl LexerSettings {
    pub fn get_uniques(&self) -> String {
        self.uniques.lock().unwrap().clone()
    }

    pub fn set_uniques(&self, value: String) {
        *self.uniques.lock().unwrap() = value;
    }

    pub fn get_separators(&self) -> String {
        self.separators.lock().unwrap().clone()
    }

    pub fn set_separators(&self, value: String) {
        *self.separators.lock().unwrap() = value;
    }

    pub fn get_primary_sep(&self) -> String {
        self.primary_separator.lock().unwrap().clone()
    }

    pub fn set_primary_sep(&self, value: String) {
        *self.primary_separator.lock().unwrap() = value;
    }
}

struct PointedProgram {
    program: Vec<char>,
    pointer: usize
}

impl PointedProgram {
    fn new(program: String) -> Self {
        PointedProgram {
            program: program.chars().collect(),
            pointer: 0
        }
    }

    fn current_char(&self) -> Option<char> {
        self.program.get( self.pointer ).map(|c| *c)
    }

    fn increment_pointer(&mut self) {
        self.pointer += 1;
    }
}

pub struct Tokenizer<'a> {
    programs: Vec<PointedProgram>,
    pub lexer_settings: &'a LexerSettings
}

impl<'a> Tokenizer<'a> {
    pub fn new(lexer_settings: &'a LexerSettings) -> Self {
        Tokenizer {
            programs: Vec::with_capacity(16),
            lexer_settings
        }
    }

    pub fn new_program(&mut self, program: String) {
        self.programs.push( PointedProgram::new(program) );
    }

    pub fn next_word(&mut self) -> Option<String> {
        let uniques = self.lexer_settings.get_uniques();
        let separators = self.lexer_settings.get_separators();
        'main_loop: loop {
            let mut program = match self.programs.pop() {
                Some(p) => p,
                None => return None
            };

            let mut current_char = match program.current_char() {
                Some(c) => c,
                None => continue 'main_loop
            };
            while separators.contains( current_char ) {
                program.increment_pointer();
                current_char = match program.current_char() {
                    Some(c) => c,
                    None => continue 'main_loop
                };
            }
            if uniques.contains( current_char ) {
                program.increment_pointer();
                self.programs.push( program );
                return Some( String::from( current_char ) );
            }
            let mut word = String::from( current_char );
            program.increment_pointer();
            current_char = match program.current_char() {
                Some(c) => c,
                None => return Some( word )
            };
            while !separators.contains( current_char ) && !uniques.contains( current_char ) {
                word.push( current_char );
                program.increment_pointer();
                current_char = match program.current_char() {
                    Some(c) => c,
                    None => return Some( word )
                };
            }
            self.programs.push( program );
            return Some(word);
        }
    }
}
