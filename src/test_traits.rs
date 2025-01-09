pub fn run_test_traits() {
    let bear01: &dyn AnimalActions = &Bear{};
    let dog01:&dyn AnimalActions = &Dog{};
    eat_some_food(dog01);
    make_some_noise(bear01);

    let bear01 = get_animal();
    bear01.eat_food();
    bear01.make_sound();
}

fn make_some_noise(a: &dyn AnimalActions) {
    a.make_sound();
}

fn eat_some_food(a:&dyn AnimalActions) {
    a.eat_food();
  
}


fn get_animal() -> Box::<dyn AnimalActions> {
    let animal = Bear{};
    return Box::from(animal);
}
// fn make_some_noise<Animal:AnimalSounds>(a:Animal) {
//     a.make_sound();
// }


struct Dog {}
struct Cat {}

struct Bear {}


trait AnimalActions : AnimalEating + AnimalSounds {}
trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSounds{
    fn make_sound(&self);
}


impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("Dog is eating bones");
    }
}


impl AnimalSounds for Dog {
    fn make_sound(&self) {
        println!("Dog barks");
    }
}

impl AnimalActions for Dog {

}

impl AnimalEating for Cat {
    fn eat_food(&self) {
        println!("Animal is drinking milk");
    }
}


impl AnimalSounds for Cat {
    fn make_sound(&self) {
        println!("Cat meows");
    }
}

impl AnimalActions for Cat {
    
}


impl AnimalEating for Bear {
    fn eat_food(&self) {
        println!("Bear is eating other animals ")
    }
}

impl AnimalSounds for Bear {
    fn make_sound(&self) {
        println!("Bear roars");
    }
}
impl AnimalActions for Bear {
    
}