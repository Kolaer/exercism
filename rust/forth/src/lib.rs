pub type Value = i32;
pub type ForthResult = Result<(), Error>;

type Stack = Vec<Value>;

#[derive(Debug, PartialEq, Clone)]
enum ForthWord {
    Val(Value),
    Native,
    UserDefined(Vec<CompiledWord>),
}

#[derive(Debug, PartialEq, Clone)]
struct CompiledWord {
    name: String,
    code: ForthWord,
}

impl CompiledWord {
    fn with_name(name: String, code: ForthWord) -> CompiledWord {
        CompiledWord { name, code }
    }
}

#[derive(Default)]
pub struct Forth {
    words: Vec<CompiledWord>,
    stack: Stack,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    fn native_words() -> Vec<CompiledWord> {
        ["+", "-", "*", "/", "dup", "swap", "over", "drop"]
            .iter()
            .map(|name| CompiledWord {
                name: (*name).to_owned(),
                code: ForthWord::Native,
            })
            .collect()
    }

    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            words: Self::native_words(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    fn eval_word(&mut self, word: CompiledWord) -> ForthResult {
        match word.code {
            ForthWord::Val(value) => {
                self.stack.push(value);
                Ok(())
            },
            ForthWord::Native => {
                match word.name.as_ref() {
                    "+" => {
                        if self.stack.len() < 2 {
                            Err(Error::StackUnderflow)
                        } else {
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();

                            self.stack.push(a + b);

                            Ok(())
                        }
                    },
                    "-" => {
                        if self.stack.len() < 2 {
                            Err(Error::StackUnderflow)
                        } else {
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();

                            self.stack.push(a - b);

                            Ok(())
                        }
                    },
                    "*" => {
                        if self.stack.len() < 2 {
                            Err(Error::StackUnderflow)
                        } else {
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();

                            self.stack.push(a * b);

                            Ok(())
                        }
                    },
                    "/" => {
                        if self.stack.len() < 2 {
                            Err(Error::StackUnderflow)
                        } else {
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();

                            if b == 0 {
                                Err(Error::DivisionByZero)
                            } else {
                                self.stack.push(a / b);

                                Ok(())
                            }
                        }
                    },
                    "dup" => {
                        if self.stack.is_empty() {
                            Err(Error::StackUnderflow)
                        } else {
                            let a = self.stack.last().unwrap();

                            self.stack.push(*a);

                            Ok(())
                        }
                    },
                    "swap" => {
                        if self.stack.len() < 2 {
                            Err(Error::StackUnderflow)
                        } else {
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();

                            self.stack.push(b);
                            self.stack.push(a);

                            Ok(())
                        }
                    },
                    "over" => {
                        if self.stack.len() < 2 {
                            Err(Error::StackUnderflow)
                        } else {
                            let b = self.stack.pop().unwrap();
                            let a = self.stack.pop().unwrap();

                            self.stack.push(a);
                            self.stack.push(b);
                            self.stack.push(a);

                            Ok(())
                        }
                    },
                    "drop" => {
                        if self.stack.is_empty() {
                            Err(Error::StackUnderflow)
                        } else {
                            let _ = self.stack.pop().unwrap();

                            Ok(())
                        }
                    },
                    _ => Err(Error::UnknownWord)
                }
            },
            ForthWord::UserDefined(words) => {
                for word in words {
                    self.eval_word(word)?;
                }
                Ok(())
            }
        }
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut compilation_mode = false;
        let mut compiled_word = Vec::new();
        let mut word_name = String::new();
        let mut waiting_name = false;

        for word in input.split_whitespace() {
            let word = word.to_lowercase();
            // if we are waiting name for new word
            if waiting_name {
                if word.parse::<Value>().is_ok() {
                    return Err(Error::InvalidWord);
                }
                word_name = word.to_owned();
                waiting_name = false;
                continue;
            }

            let defined_word = self
                .words
                .iter()
                .filter(|compiled_word| compiled_word.name == word)
                .last();

            match word.as_ref() {
                ":" => {
                    if compilation_mode {
                        return Err(Error::InvalidWord);
                    } else {
                        compilation_mode = true;
                        waiting_name = true;
                    }
                }
                ";" => {
                    if compilation_mode {
                        let code = ForthWord::UserDefined(compiled_word.clone());
                        let new_word = CompiledWord::with_name(word_name.clone(), code);

                        self.words.push(new_word);

                        compilation_mode = false;

                        word_name.clear();
                        compiled_word.clear();
                    } else {
                        return Err(Error::InvalidWord);
                    }
                }
                // if number
                s if s.parse::<Value>().is_ok() => {
                    let val = s.parse().unwrap();
                    if compilation_mode {
                        let number_word = CompiledWord::with_name(String::new(), ForthWord::Val(val));
                        compiled_word.push(number_word);
                    } else {
                        self.stack.push(val);
                    }
                }
                // if defined word
                _ if defined_word.is_some() => {
                    let defined_word = defined_word.unwrap();

                    if compilation_mode {
                        compiled_word.push(defined_word.clone());
                    } else {
                        self.eval_word(defined_word.clone())?;
                    }
                }
                // not a number or defined word
                _ => return Err(Error::UnknownWord),
            }
        }

        if compilation_mode || waiting_name {
            Err(Error::InvalidWord)
        } else {
            Ok(())
        }
    }
}
