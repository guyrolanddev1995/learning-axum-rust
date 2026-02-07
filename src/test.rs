trait Animal {
    fn speak(&self);
    fn eat(&self);
}

enum ActionType {
    Speak,
    Eat,
}

struct Cat {
    name: String,
}

struct Dog {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} is speaking", self.name);
    }

    fn eat(&self) {
        println!("{} is eating", self.name);
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{}, is speaking", self.name)
    }

    fn eat(&self) {
        println!("{} is eating", self.name);
    }
}

fn do_something<T>(animal: T, action: ActionType)
where
    T: Animal,
{
    match action {
        ActionType::Eat => animal.eat(),
        ActionType::Speak => animal.speak(),
    }
}

fn run() {
    let dog = Dog {
        name: "black".to_string()
    };

    let cat = Cat {
        name: "mickey".to_string()
    };

    do_something(dog, ActionType::Speak);
    do_something(cat, ActionType::Eat)
}
