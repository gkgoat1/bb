use std::{collections::VecDeque, fmt::Display};
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}
pub struct Tape {
    pub elements: Vec<usize>,
    pub idx: usize,
    pub dir: Dir,
}
impl Display for Tape{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i,e) in self.elements.iter().enumerate(){
            if i == self.idx && self.dir == Dir::Right{
                write!(f," >")?;
            }
            write!(f," {e}")?;
            if i == self.idx && self.dir == Dir::Left{
                write!(f," <")?;
            }
        }
        Ok(())
    }
}
impl Tape {
    pub fn go(&mut self) -> bool {
        match self.dir {
            Dir::Left => match self.elements[self.idx] {
                1 => self.idx -= 1,
                2 => self.idx -= 1,
                0 => {
                    let m = self.elements.remove(self.idx - 1);
                    self.elements[self.idx - 1] = m + 1;
                    self.dir = Dir::Right;
                    self.idx += 1;
                    self.elements.insert(self.idx, 1);
                }
                _ => {
                    let m = self.elements[self.idx] - 3;
                    self.elements[self.idx] = m;
                    self.elements.insert(self.idx, 1);
                    self.idx += 1;
                    self.idx += 1;
                    self.dir = Dir::Right;
                    self.elements.insert(self.idx, 2);
                }
            },
            Dir::Right => {
                if self.idx == self.elements.len(){
                    self.idx -= 1;
                    self.dir = Dir::Left;
                    self.elements[self.idx] += 1;
                    return false;
                }
                if self.elements[self.idx] == 2{
                    if self.elements.len() == self.idx + 1{
                        self.idx -= 1;
                        self.dir = Dir::Left;
                        return false;
                    }
                    let a = self.elements[self.idx + 1];
                    if a == 2 || a == 1{
                        self.idx += 2;
                        return false;
                    }
                    return false;
                }
                if self.elements[self.idx] == 1{
                    if self.elements.len() == self.idx + 1{
                        return true;
                    }
                    let a = self.elements[self.idx + 1];
                    if a == 1{
                        let m = self.elements.remove(self.idx - 1);
                        self.idx -= 1;
                        self.elements[self.idx] = m + 1;
                        self.elements[self.idx + 1] = 2;
                        self.idx += 2;
                        return false;
                    }
                    if a == 2{
                        self.elements.remove(self.idx + 1);
                        self.dir = Dir::Left;
                        self.idx -= 1;
                        self.elements[self.idx] += 1;

                    }
                }
            },
        }
        return false;
    }
}
