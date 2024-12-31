fn main() {

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