use std::io;

fn main() {


    let mut name = String::new();
    println!("Enter hero's name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim().to_string();

    let mut health_input = String::new();
    println!("Enter hero's health:");
    io::stdin()
        .read_line(&mut health_input)
        .expect("Failed to read line");
    let health: i32 = health_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut hero1 = Hero { id: 3, name, health};

    hero1.printHero();

    println!("______________________");


    let mut mutator: Hero = Hero {
        id: 1,
        name: "Mutator".to_string(),
        health: 100,
    };

    let mut threator: Hero = Hero {
        id: 2,
        name: "Threator".to_string(),
        health: 100,
    };

    mutator.printHero();
    threator.printHero();

    mutator.removeHealth(25);
    threator.addHealth(25);

    mutator.printHero();
    threator.printHero();

    mutator.removeHealth(200);
    mutator.printHero();
    threator.printHero();



}

struct Hero {

    id: i64,
    name: String,
    health: i32,

}


impl Hero {

    fn addHealth(&mut self, value: i32) {
        println!("Adding health to hero");

        if self.health < 100 {
            self.health += value;

            if self.health > 100 {
                self.health = 100;
            }

        }

    }

    fn removeHealth(&mut self, value: i32) {
        println!("Removing health from hero");

        if self.health > 0 {
            self.health -= value;

            if self.health <= 0 {
                self.health = 0;
                println!("Hero {} is dead.", self.name);
            }

        }

    }

    fn printHero(&mut self) {
        println!("{}'s health is {}", self.name, self.health);
    }

}