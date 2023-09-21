use crate::variables::{Instruction, Object, Variables};
use crate::tokenizer::{Tokenizer, LexerSettings};


fn str_less_than(s1: String, s2: String) -> bool {
    let mut chars1 = s1.chars();
    let mut chars2 = s2.chars();
    loop {
        match (chars1.next(), chars2.next()) {
            (None, None) => return false,                        // Two strings are equal and same length
            (Some(_), None) => return false,                     // Strings have same start but s1 is longer
            (None, Some(_)) => return true,                      // Strings have same start but s2 is longer
            (Some(c1), Some(c2)) if c1 == c2 => (),  // Same character, cannot decide
            (Some(c1), Some(c2)) => return c1 < c2   // Different character
        }
    }
}

pub fn debug_interpret(program: String) {
    let lexer_settings = LexerSettings::default();
    let mut tokenizer = Tokenizer::new(&lexer_settings);
    tokenizer.new_program( program );

    let mut variables = Variables::new(&lexer_settings);
    let mut primary_buffer = Vec::new();
    let mut secondary_buffer = Vec::new();

    loop {
        // if debug_mode && secondary_buffer.get(0) == Some(&String::from("output")) {
        //     println!("{:?} {:?}", secondary_buffer, primary_buffer);
        //     let mut input = String::new();
        //     let _ = std::io::stdin().read_line(&mut input);
        // }

        let word = match tokenizer.next_word() {
            Some(w) => w,
            None => break
        };
        match variables.get( &word ) {
            Object::Keyword(Instruction::SwapBuffer) => {
                std::mem::swap(&mut primary_buffer, &mut secondary_buffer);
            },
            Object::Keyword(Instruction::SkipNext) => {
                let text = match tokenizer.next_word() {
                    Some(w) => w,
                    None => break
                };
                primary_buffer.push( text );
            },
            Object::Keyword(Instruction::ExpandPrevious) => 'match_arm: {
                let Some(word) = primary_buffer.pop() else {
                    break 'match_arm;
                };
                match variables.get( &word ) {
                    Object::Keyword(_) => tokenizer.new_program( word ),
                    Object::Text(text) => {
                        let mut temp_tokenizer = Tokenizer::new(&lexer_settings);
                        temp_tokenizer.new_program( text );
                        loop {
                            match temp_tokenizer.next_word() {
                                Some(word) => primary_buffer.push( word ),
                                None => break
                            }
                        }
                    }
                }
            },
            Object::Keyword(Instruction::CrazySwap) => {
                let maybe_text = tokenizer.next_word();
                let maybe_text2 = tokenizer.next_word();
                let maybe_word = primary_buffer.pop();
                let maybe_word2 = primary_buffer.pop();
                
                if let Some(word2) = maybe_word2 { tokenizer.new_program( word2 ) };
                if let Some(word) = maybe_word { tokenizer.new_program( word ) };
                if let Some(text2) = maybe_text2 { primary_buffer.push( text2 ) };
                if let Some(text) = maybe_text { primary_buffer.push( text ) };
            },
            Object::Keyword(Instruction::JoinValues) => {
                let text2 = match primary_buffer.pop() {
                    Some(text) => text,
                    None => continue
                };
                let combined = match primary_buffer.pop() {
                    Some(text1) => text1 + &text2,
                    None => text2
                };
                primary_buffer.push( combined )
            },
            Object::Keyword(Instruction::EndLine) => {
                let value = primary_buffer.join(" ");
                let identifier = secondary_buffer.join(" ");
                variables.set(identifier, value);
                primary_buffer.clear();
                secondary_buffer.clear();
            },
            Object::Keyword(Instruction::DereferenceLast) => 'match_arm: {
                let Some( var_name ) = primary_buffer.last() else {
                    break 'match_arm;
                };
                let Object::Text( text ) = variables.get( var_name ) else {
                    break 'match_arm;
                };
                let obj = variables.get( &text );
                variables.set_object( var_name.to_owned(), obj );
            },
            Object::Keyword(Instruction::IfLess) => {
                let skip_next_instruction = match (primary_buffer.pop(), primary_buffer.pop()) {
                    (None, None) => true,
                    (Some(_), None) => false,
                    (Some(val2), Some(val1)) => !str_less_than(val1, val2),
                    _ => unreachable!()
                };
                if skip_next_instruction {
                    tokenizer.next_word();  // destroy the next word without looking at its value
                }
            }
            Object::Text(text) => {
                tokenizer.new_program( text )
            }
        }
    }
}
