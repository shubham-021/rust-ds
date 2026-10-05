#![allow(unused)]

struct Trie {
    data: Vec<Option<Box<Trie>>>,
    flag: bool,
    child: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            data: (0..26).map(|_| None).collect(),
            flag: false,
            child: 0,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut temp = self;
        let lowercased = word.to_ascii_lowercase();

        for &b in lowercased.as_bytes() {
            let index = b as usize - 'a' as usize;
            // temp = temp.data[index].get_or_insert_with(|| Box::new(Trie::new()));
            if temp.data[index].is_none() {
                temp.child += 1;
                temp.data[index] = Some(Box::new(Trie::new()));
            }

            temp = temp.data[index].as_mut().unwrap();
        }

        temp.flag = true;
    }

    fn exists(&self, word: &str) -> bool {
        let mut temp = self;
        let size = word.len();
        let lowercased = word.to_ascii_lowercase();

        for i in 0..size {
            let index = lowercased.as_bytes()[i] as usize - 'a' as usize;
            if let Some(next) = &temp.data[index] {
                temp = next.as_ref();
            } else {
                return false;
            }
        }

        temp.flag
    }

    fn matches(&self, word: &str) -> bool {
        let mut temp = self;
        let size = word.len();
        let lowercased = word.to_ascii_lowercase();

        for i in 0..size {
            let index = lowercased.as_bytes()[i] as usize - 'a' as usize;
            if let Some(next) = &temp.data[index] {
                temp = next.as_ref();
            } else {
                return false;
            }
        }

        true
    }

    fn delete_helper(next: &mut Trie, word: &[u8], index: usize, size: usize) -> bool {
        if index == size {
            if next.flag == true {
                next.flag = false;
                return true;
            }

            return false;
        }

        let mut have: bool = false;
        let alphabet_num = word[index] as usize - 'a' as usize;
        if let Some(charc) = next.data[alphabet_num].as_deref_mut() {
            have = Self::delete_helper(charc, word, index + 1, size);
            if have && !charc.flag && charc.child == 0 {
                next.data[alphabet_num] = None;
                next.child -= 1;
            }

            return have;
        }

        false
    }

    fn delete(&mut self, word: &str) -> bool {
        let mut temp = self;
        let size = word.len();
        let lowercased = word.to_ascii_lowercase();

        Self::delete_helper(temp, lowercased.as_bytes(), 0, size)
    }
}

fn main() {
    let mut mt = Trie::new();
    mt.insert("cat");
    mt.insert("car");
    mt.insert("catty");
    mt.insert("dog");
    mt.insert("canteen");

    println!("Cat exists: {}", mt.exists("cat"));
    println!("Can exists: {}", mt.exists("can"));
    println!("Can matches: {}", mt.matches("can"));
    println!("caRe exists: {}", mt.exists("caRe"));
    println!("cAT exists: {}", mt.exists("cAT"));
    println!("do exists: {}", mt.exists("do"));
    println!("catty matches: {}", mt.matches("catty"));

    mt.delete("cat");
    println!("\n\nAfter deleting cat: \n\n");
    println!("Cat exists: {}", mt.exists("cat"));
    println!("Cat matches: {}", mt.matches("cat"));
    println!("Catty exists: {}", mt.exists("catty"));
}
