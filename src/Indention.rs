#[derive(Clone)]
pub struct Indention {
    pub level: usize,
}

impl Indention {
    pub fn new() -> Indention {
        Indention { level: 0 }
    }
    pub fn inc(&mut self) {
        self.level += 1;
    }
    pub fn dec(&mut self) {
        self.level -= 1;
    }
    pub fn get(&self) -> String {
        let mut s = String::new();
        for _ in 0..self.level {
            s.push_str("    ");
        }
        return s;
    }
}
