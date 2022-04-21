pub struct Word {
    pub search: String,
    num_try: u32,
    arr_try: Vec<String>,
    pub mixt: String
}

impl Word {
    pub fn new() -> Word {
        let arr_try = Vec::new();
        Word { search: "".to_string() , num_try: 1, arr_try: arr_try ,mixt: "".to_string() }
    }
    fn add(&mut self, save_try: &String)  {
        self.arr_try.push(save_try.to_string());
        self.num_try += 1;
    }
    pub fn same(&mut self, send: &String) -> bool {
        self.add(send);
        self.search.eq(send)
    }
    pub fn get_num_try(&self) -> u32 {
        self.num_try
    }
    pub fn get_name_try(&self, num: u32) -> String {
        self.arr_try[num as usize].to_string()
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
    fn get(&self, num: u32) -> char {
        self.text[num as usize]
    }
    pub fn remove(&mut self, num: u32) -> String {
        let mut letter = String::new();

        letter += &self.get(num).to_string();

        self.text.swap_remove(num as usize);
        letter
    }
}
