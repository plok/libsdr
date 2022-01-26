use std::collections::HashMap;

pub struct Pattern {

    length: u8,
    hits: HashMap<u8,Hit>,

}

impl Pattern {

    pub fn new(length: u8) -> Pattern {
        Pattern { length: length, hits: HashMap::new() } 
    }

    pub fn add_hit(&mut self,location: u8, hit: Hit) {
        if location + hit.length < self.length {
            self.hits.insert(location,hit);
        } else {
            panic!("aaaaaaasdasd!");
        }
    }
}

pub struct Hit {
    pub length: u8,
    pub velocity: u8,
}
