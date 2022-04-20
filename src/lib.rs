pub struct Word {
    pub search: String,
    essaie: u32,
    pub mixt: String,
}

impl Word {
    pub fn new() -> Word {
        Word { search: "".to_string() , essaie: 1, mixt: "".to_string() }
    }
    fn add(&mut self)  {
        self.essaie += 1;
    }
    pub fn same(&mut self, send: &String) -> bool {
        self.add();
        self.search.eq(send)
    }
    pub fn get_essai(&self) -> u32 {
        self.essaie
    }
}

pub struct VecteurString {
        text: Vec<char>
}

impl VecteurString {
    pub fn new(s: &String) -> VecteurString {
         let char_vec: Vec<char> = s.chars().collect();
         VecteurString { text: char_vec }
    }
    pub fn len(&self) -> u32 {
        let load: u32 = self.text.len() as u32;
        load
    }
    pub fn get(&self, num: u32) -> char {
        self.text[num as usize]
    }
    pub fn get_integreat(&self) -> String {
        let mut ret = String::new();

        for i in 0..self.len() {
            ret += &self.text[i as usize].to_string();
        }
        ret
    }
    pub fn remove(&mut self, num: u32) -> String {
        let mut letter = String::new();

        letter += &self.get(num).to_string();

        self.text.swap_remove(num as usize);
        letter
    }
}
