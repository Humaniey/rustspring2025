#[allow(unused_variables)]
fn turbofish() {
    #[derive(Debug)]
    struct Pet<T> {
        cats: T,
        dogs: T,
    }

    impl<T> Pet<T> {
        fn new(a: T, b: T) -> Self {
            Pet {
                cats: a,
                dogs: b,
            }
        }
    }
    
    let pets = Pet::<i64>::new(5, 10);
    println!("Cats: {}, Dogs: {}", pets.cats, pets.dogs);

    let pets = Pet::<String>::new("Million".into(), "Billion".into());
    println!("Cats: {}, Dogs: {}", pets.cats, pets.dogs);
}

fn main() {
    turbofish();
}
