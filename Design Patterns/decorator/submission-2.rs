trait Coffee {
    fn get_cost(&self) -> f64;
}

struct SimpleCoffee;

impl SimpleCoffee {
    fn new() -> Self {
        SimpleCoffee
    }
}

impl Coffee for SimpleCoffee {
    fn get_cost(&self) -> f64 {
        1.1
    }
}

struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn get_cost(&self) -> f64 {
        self.coffee.get_cost() + 0.5
    }
}

struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}

impl Coffee for SugarDecorator {
    fn get_cost(&self) -> f64 {
        self.coffee.get_cost() + 0.2
    }
}

struct CreamDecorator {
    coffee: Box<dyn Coffee>,
}

impl CreamDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        CreamDecorator { coffee }
    }
}

impl Coffee for CreamDecorator {
    fn get_cost(&self) -> f64 {
        self.coffee.get_cost() + 0.7
    }
}