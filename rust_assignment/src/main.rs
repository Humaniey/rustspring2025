#[derive(Debug)]
enum Insurance {
    House,
    Car,
    Life,
}

#[derive(Debug)]
struct Car {
    model: String
}

#[derive(Debug)]
struct Person {
    name: String,
    car: Car,
    insurances: Vec<Insurance>,
}

impl Person {
    fn new(n: String, c: Car) -> Person {
        Person {
            name: n,
            car: c,
            insurances: vec![],
        }
    }

    fn add_insurance(&mut self, i: Insurance) {
        self.insurances.push(i);
    }

    fn show_insurance(&self) {
        println!("Hey I am {:?}. I have a next type of insurances", self);
        for i in self.insurances.iter(){
            match i {
                Insurance::Car => println!("I insured my {:?}", self.car),
                Insurance::House => println!("I insured my {:?}", self.house),
                Insurance::House => println!("I insured my {:?}", self.life),
            };
        }
    }
}

fn main() {
    let my_c = Insurance::Car;
    let my_h = Insurance::House;
    let my_l = Insurance::Life;

    let car = Car {
        model: "Toyota".to_string(),
    };

    let mut person = Person::new("John".to_string(), car);

    person.add_insurance(my_c);
    person.add_insurance(my_h);
    person.add_insurance(my_l);
    person.show_insurance();

    println!("{:?}", person)
}


