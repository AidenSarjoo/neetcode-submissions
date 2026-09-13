struct Meal {
    cost: f64,
    take_out: bool,
    main: String,
    drink: String,
}

impl Meal {
    fn new() -> Self {
        Meal {
            cost: 0.0,
            take_out: false,
            main: String::new(),
            drink: String::new(),
        }
    }

    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_take_out(&self) -> bool {
        self.take_out
    }

    fn get_main(&self) -> &str {
        &self.main
    }

    fn get_drink(&self) -> &str {
        &self.drink
    }

    fn set_cost(&mut self, cost: f64) {
        self.cost = cost;
    }

    fn set_take_out(&mut self, take_out: bool) {
        self.take_out = take_out;
    }

    fn set_main(&mut self, main: String) {
        self.main = main;
    }

    fn set_drink(&mut self, drink: String) {
        self.drink = drink;
    }
}

struct MealBuilder {
    meal: Meal,
}

impl MealBuilder {
    pub fn new() -> Self {
        MealBuilder { meal: Meal::new() }
    }

    pub fn add_cost(mut self, cost: f64) -> Self {
        self.meal.set_cost(cost);
        return self;
    }

    pub fn add_take_out(mut self, take_out: bool) -> Self {
        self.meal.set_take_out(take_out);
        return self;
    }

    pub fn add_main_course(mut self, main: String) -> Self {
        self.meal.set_main(main);
        return self;
    }

    pub fn add_drink(mut self, drink: String) -> Self {
        self.meal.set_drink(drink);
        return self;
    }

    pub fn build(self) -> Meal {
        return self.meal;
    }
}
