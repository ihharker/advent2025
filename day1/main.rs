use std::fs;

fn main() {
    let contents = fs::read_to_string("in.txt").expect("read error");

    let mut dial = Dial { num: 50, zeros: 0 };

    for line in contents.lines() {
        //println!("{line}");
        let line_vec: Vec<char> = line.chars().collect();
        let turns_str: String = line_vec[1..].iter().collect();
        let turns: i32 = turns_str.parse().expect("invalid num");

        match line_vec[0] {
            'L' => dial.turn('L', turns),
            'R' => dial.turn('R', turns),
            _ => println!("💀💀💀 rip bozo"),
        }

        //println!("dial {dial:?}");
        //println!("-----")
    }
    println!("The password is {}", dial.zeros());
}

#[derive(Debug)]
struct Dial {
    num: i32,
    zeros: i32,
}

impl Dial {
    fn turn(&mut self, direction: char, clicks: i32) {
        let mut clicks = clicks;
        while clicks > 0 {
            match direction {
                'L' => self.num -= 1,
                'R' => self.num += 1,
                _ => {}
            }
            // correct dial if out of bounds
            match self.num {
                -1 => self.num = 99,
                100 => self.num = 0,
                _ => {}
            }
            clicks -= 1;
            if self.num == 0 {
                self.zeros += 1;
            }
        }
    }

    fn zeros(&self) -> i32 {
        self.zeros
    }
}
