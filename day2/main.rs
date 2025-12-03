use fancy_regex::Regex;
use std::fs;

#[derive(Debug)]
struct Id {
    first: i64,
    last: i64,
}

impl Id {
    fn get_vec(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        for i in self.first..=self.last {
            vec.push(i.to_string());
        }
        vec
    }
}

fn is_invalid(re: &Regex, num: &str) -> bool {
    if re.captures(num).unwrap().is_some() {
        return true;
    }
    false
}

fn get_invalid_vec(re: &Regex, range: Vec<String>) -> Vec<i64> {
    let mut vec: Vec<i64> = Vec::new();

    for num in range {
        if is_invalid(&re, &num) {
            vec.push(num.parse().unwrap());
        }
    }
    vec
}

fn main() {
    let re = Regex::new(r"^(\d+)(?:\1)+$").unwrap();

    let contents = fs::read_to_string("in.txt").unwrap();
    let contents: Vec<&str> = contents.split(',').collect();

    let mut all_invalid: Vec<i64> = Vec::new();
    for id_full in contents {
        let (first, last) = id_full.split_once('-').unwrap();
        //println!("{}, {}, {}", id_full, first, last);
        let id = Id {
            first: first.parse().unwrap(),
            last: last.parse().unwrap(),
        };
        let mut invalid_vec = get_invalid_vec(&re, id.get_vec());
        //println!("{:?}", invalid_vec);
        all_invalid.append(&mut invalid_vec);
    }
    let sum: i64 = all_invalid.iter().sum();
    println!("All invalids sum to {}", sum);
}
