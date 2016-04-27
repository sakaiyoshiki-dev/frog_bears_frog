// Proverb-Programming: "A frog bears a frog"

use std::io;

struct Frog {
    is_frog: bool
}

impl Frog {
    // constructor
    pub fn new() -> Frog {
        Frog{
            is_frog: true
        }
    }
    pub fn is_frog(&self)->bool{
        self.is_frog
    }

    pub fn bears_child_frog (&self) -> Frog{
        Frog{
            is_frog: true
        }        
    }
    pub fn jump_into(self, pond: &mut Pond) -> bool{
        let pond_has_space: bool = !(pond.is_fulfilled());
        if pond_has_space {
            pond.accept(self);
            //println!("{:?}",self.is_frog()); //ownership has moved
        }
        pond_has_space
    }
}

struct Pond {
    pub n_frog_capacity: u32,
    pub frogs_living_in: Vec<Frog>
}

impl Pond {
    pub fn new(n_frog_capacity: u32) -> Pond {
        Pond{
            n_frog_capacity: n_frog_capacity,
            frogs_living_in: Vec::new()
        }
    }

    pub fn count_frogs(&self) -> u32{
        self.frogs_living_in.len() as u32
    }

    pub fn is_fulfilled(&self) -> bool{
        self.count_frogs() >= self.n_frog_capacity
    }

    pub fn accept(&mut self, frog: Frog) {
        self.frogs_living_in.push(frog);
    }
}

fn main() {

    println!("Please input the frog capacity of the pond:");
    let mut n_frog_capacity_string = String::new();
    //io::stdin().read_line(&mut n_frog_capacity_string);
    io::stdin().read_line(&mut n_frog_capacity_string)
        .expect("cannot read the line.");
    n_frog_capacity_string = n_frog_capacity_string.trim_right().to_string();
    let n_frog_capacity: u32 = n_frog_capacity_string.parse().unwrap();

    let mut the_pond = Pond::new(n_frog_capacity);
    let first_frog = Frog::new();
    first_frog.jump_into(&mut the_pond);

    let mut epochs = 0;
    'outer: loop{
        epochs += 1;
        let mut new_children_frogs: Vec<Frog> = Vec::new();
        for frog in & (the_pond.frogs_living_in) {
            let child_frog = frog.bears_child_frog();
            if child_frog.is_frog(){
                new_children_frogs.push(child_frog);
            }
        }
        for frog in new_children_frogs{
            let could_jump_into = frog.jump_into(&mut the_pond);
            if !could_jump_into{
                println!("Epochs: {:5}, Frogs: {:5}", epochs, the_pond.count_frogs());
                break 'outer;
            }
        }
        println!("Epochs: {:5}, Frogs: {:5}", epochs, the_pond.count_frogs());
    }
    println!("The pond has no space any more!");
}
