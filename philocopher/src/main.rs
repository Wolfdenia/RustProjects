extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

enum Sex {
    Men,
    Women, 
}

struct Philosopher {
    name: String,
    sex: Sex,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, sex: Sex, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            sex: sex,
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
                
        
        match self.sex {
            Sex::Men => println!("{} начал кушать.", self.name),
            Sex::Women => println!("{} начала кушать.", self.name)
        };
        
        thread::sleep(Duration::from_millis(1000));
        match self.sex {
            Sex::Men => println!("{} закончил кушать.", self.name),
            Sex::Women => println!("{} закончила кушать.", self.name)
        };
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec! [
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),]
    });


    let philosphers = vec![
        Philosopher::new("Айн Рэнд", Sex::Women, 0, 1),
        Philosopher::new("Шелдон", Sex::Men, 2, 3),
        Philosopher::new("Софья Ковалевская", Sex::Women, 1, 2),
        Philosopher::new("Тинки-Винки", Sex::Men, 3, 4),
        Philosopher::new("Гаечка", Sex::Women, 0, 4)
    ];
    
    let handles: Vec<_> = philosphers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
    //println!("Философ дня: {}", );
}
