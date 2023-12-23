use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
        Lines,
        self},
    path::Path,
    str::FromStr,
};

pub struct InputHandler {}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler { }
    }

    pub fn parse_lines<P>(&self, filename: P, mode: Mode) -> FilteredLines<impl BufRead>
    where P: AsRef<Path>, {
        let file = File::open(filename).unwrap();
        let buffer = BufReader::new(file);

        FilteredLines::new(buffer, mode)
    }
}

pub struct FilteredLines<R>
where R: BufRead,
{
    inner: Lines<R>,
    mode: Mode,
    mode_found: bool,
}

impl<R> FilteredLines<R>
where R: BufRead,
{
    fn new(reader: R, mode: Mode) -> Self {
        let inner = reader.lines();

        FilteredLines { inner, mode, mode_found: false }
    }
}

impl<R> Iterator for FilteredLines<R>
where R: BufRead
{
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(line) = self.inner.next() {
            match line {
                Ok(line) => {
                    let line = line.trim();

                    if line.is_empty() {
                        continue;
                    }
                    else if line.starts_with('#') {
                        if let Ok(mode) = Mode::from_str(&line[1..]) {
                            if self.mode_found {
                                break;
                            }

                            if mode == self.mode {
                                self.mode_found = true;
                            }
                        }

                        continue;
                    }
                    else if !self.mode_found
                    {
                        continue;
                    }

                    return Some(Ok(line.to_owned()));
                },
                Err(err) => {
                    return Some(Err(err));
                }
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Fake,
    Real,
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "fake" => Ok(Mode::Fake),
            "real" => Ok(Mode::Real),
            _ => Err(()),
        }
    }
}