// Proverb-Programming: "Frog bears Frogs"

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
}

fn main() {
    let n_max_frogs = 4096;
    let mut frogs: Vec<Frog> = Vec::new();
    let first_frog = Frog::new();
    frogs.push(first_frog);

    let mut cnt = 0;
    loop{
        let mut new_frogs : Vec<Frog> = Vec::new();
        for frog in &frogs {
            let child_frog = frog.bears_child_frog();
            if child_frog.is_frog() {
                new_frogs.push(child_frog);
            }
        }
        frogs.append(&mut new_frogs);
        cnt += 1;
        println!("Epochs: {:5}, Frogs: {:5}", cnt, frogs.len());
        if frogs.len() >= n_max_frogs{break;}
    }
    println!("The pond get fulfilled.");
}
