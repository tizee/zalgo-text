use rand::{thread_rng, Rng};

#[derive(Debug, Default)]
pub struct MarkConfig {
    mark_num: usize,
    extend: bool,
    supplement: bool,
    symbol: bool,
    half: bool,
}

impl MarkConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_mark_num(&mut self, num: usize) -> &mut Self {
        self.mark_num = num;
        self
    }

    pub fn set_extend(&mut self, flag: bool) -> &mut Self {
        self.extend = flag;
        self
    }

    pub fn set_supplement(&mut self, flag: bool) -> &mut Self {
        self.supplement = flag;
        self
    }

    pub fn set_symbol(&mut self, flag: bool) -> &mut Self {
        self.symbol = flag;
        self
    }

    pub fn set_half(&mut self, flag: bool) -> &mut Self {
        self.half = flag;
        self
    }
}

/// Unicode combining characters
/// Main Diacritics for EU langs and IPA: U+0300 - U+036F
/// Diacritics Extend: U+1AB0 -  U+1AFF ( Currently the last defined mark is U+1ACE )
/// Diacritics Supplement: U+1DC0 -  U+1DFF
/// Diacritics for symbols : U+20D0 -  U+20FF ( Currently the last defined mark is U+20F0 )
/// Half marks: U+FE20 - U+FE2F
#[derive(Debug)]
struct DiacriticsMark<'a> {
    mark_config: &'a MarkConfig,
    main_marks: Vec<char>,
    extend_marks: Vec<char>,
    supplement_marks: Vec<char>,
    symbol_marks: Vec<char>,
    half_marks: Vec<char>,
}

impl<'a> DiacriticsMark<'a> {
    pub fn new(config: &'a MarkConfig) -> Self {
        Self {
            mark_config: config,
            main_marks: ('\u{0300}'..='\u{036F}').collect(),
            extend_marks: ('\u{1AB0}'..='\u{1ACE}').collect(),
            supplement_marks: ('\u{1DC0}'..='\u{1DFF}').collect(),
            symbol_marks: ('\u{20D0}'..='\u{20F0}').collect(),
            half_marks: ('\u{FE20}'..='\u{FE2F}').collect(),
        }
    }

    fn random_index(&self, len: usize) -> usize {
        thread_rng().gen_range(0..len)
    }

    pub fn random_main_mark(&self) -> char {
        let index = self.random_index(self.main_marks.len());
        self.main_marks[index]
    }

    pub fn random_extend_mark(&self) -> char {
        let index = self.random_index(self.extend_marks.len());
        self.extend_marks[index]
    }

    pub fn random_supplement_mark(&self) -> char {
        let index = self.random_index(self.supplement_marks.len());
        self.supplement_marks[index]
    }

    pub fn random_symbol_mark(&self) -> char {
        let index = self.random_index(self.symbol_marks.len());
        self.symbol_marks[index]
    }

    pub fn random_half_mark(&self) -> char {
        let index = self.random_index(self.half_marks.len());
        self.half_marks[index]
    }

    pub fn random_mark(&self) -> char {
        let index = thread_rng().gen_range(0..5);
        match index {
            0 => self.random_main_mark(),
            1 => {
                if self.mark_config.extend {
                    self.random_extend_mark()
                } else {
                    self.random_mark()
                }
            }
            2 => {
                if self.mark_config.supplement {
                    self.random_supplement_mark()
                } else {
                    self.random_mark()
                }
            }
            3 => {
                if self.mark_config.symbol {
                    self.random_symbol_mark()
                } else {
                    self.random_mark()
                }
            }
            4 => {
                if self.mark_config.half {
                    self.random_half_mark()
                } else {
                    self.random_mark()
                }
            }
            _ => self.random_main_mark(),
        }
    }
}

pub fn zalgo_text(input: String, config: &MarkConfig) -> String {
    let marks = DiacriticsMark::new(config);

    let iter = input.chars();
    let mut zalgo: Vec<Vec<char>> = Vec::new();
    for c in iter {
        // In Unicode, diacritics are ALWAYS added after the main character.
        // It's also possible to add several diacritics to the same character including stacking
        // diacritics above and below.
        // Old combining characters like ANSEL may be added before the main character.
        let mut glitch_char: Vec<char> = vec![c];
        for _ in 0..config.mark_num {
            glitch_char.push(marks.random_mark());
        }
        zalgo.push(glitch_char);
    }
    return zalgo
        .iter()
        .map(|e| e.iter().collect())
        .collect::<Vec<String>>()
        .join("");
}
