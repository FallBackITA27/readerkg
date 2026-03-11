use std::fmt::{Display, Write};

pub struct TabsWriter {
    str: String,
    col_lengths: [u8; 255],
    current_col: usize,
    tmp_current_col_length: u8,
}

impl Default for TabsWriter {
    fn default() -> Self {
        Self {
            str: String::new(),
            col_lengths: [2; 255],
            current_col: 0,
            tmp_current_col_length: 0,
        }
    }
}

impl Write for TabsWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.str += s;

        for char in s.chars() {
            match char {
                '\t' => {
                    self.col_lengths[self.current_col] = std::cmp::max(
                        self.tmp_current_col_length,
                        self.col_lengths[self.current_col],
                    );
                    self.current_col += 1;
                    self.tmp_current_col_length = 0;
                }
                '\n' => {
                    self.current_col = 0;
                    self.tmp_current_col_length = 0;
                }
                _ => {
                    self.tmp_current_col_length += 1;
                }
            }
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.str.push(c);
        match c {
            '\t' => {
                self.col_lengths[self.current_col] = std::cmp::max(
                    self.tmp_current_col_length,
                    self.col_lengths[self.current_col],
                );
                self.current_col += 1;
                self.tmp_current_col_length = 0;
            }
            '\n' => {
                self.current_col = 0;
            }
            _ => {
                self.tmp_current_col_length += 1;
            }
        }

        Ok(())
    }
}

impl Display for TabsWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_string_length = 0;
        let mut current_col = 0;
        for char in self.str.chars() {
            match char {
                '\t' => {
                    for _ in current_string_length..2 + self.col_lengths[current_col] {
                        write!(f, " ")?;
                    }
                    current_col += 1;
                    current_string_length = 0;
                }
                '\n' => {
                    writeln!(f)?;
                    current_col = 0;
                    current_string_length = 0;
                }
                v => {
                    write!(f, "{}", v)?;
                    current_string_length += 1;
                }
            }
        }

        Ok(())
    }
}
