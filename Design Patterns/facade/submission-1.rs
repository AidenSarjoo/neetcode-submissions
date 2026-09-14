struct Order {
    contents: String,
    take_out: bool,
}

impl Order {
    fn new(contents: String, take_out: bool) -> Self {
        Order { contents, take_out }
    }

    fn get_order(&self) -> &str {
        &self.contents
    }

    fn is_take_out(&self) -> bool {
        self.take_out
    }
}

struct Cashier;

impl Cashier {
    fn new() -> Self {
        Cashier
    }

    fn take_order(&self, contents: String, take_out: bool) -> Order {
        Order::new(contents, take_out)
    }
}

struct Food {
    contents: String,
}

impl Food {
    fn new(order: String) -> Self {
        Food { contents: order }
    }

    fn get_food(&self) -> &str {
        &self.contents
    }
}

struct Chef;

impl Chef {
    fn new() -> Self {
        Chef
    }

    fn prepare_food(&self, order: &Order) -> Food {
        Food::new(order.get_order().to_string())
    }
}

struct PackagedFood {
    contents: String,
}

impl PackagedFood {
    fn new(food: Food) -> Self {
        PackagedFood {
            contents: format!("{} in a bag", food.get_food()),
        }
    }

    fn get_food(&self) -> &str {
        &self.contents
    }
}

struct KitchenStaff;

impl KitchenStaff {
    fn new() -> Self {
        KitchenStaff
    }

    fn package_order(&self, food: Food) -> PackagedFood {
        PackagedFood::new(food)
    }
}

struct DriveThruFacade {
    cashier: Cashier,
    chef: Chef,
    kitchen_staff: KitchenStaff,
}

impl DriveThruFacade {
    fn new() -> Self {
        DriveThruFacade { cashier: Cashier::new(), chef: Chef::new(), kitchen_staff: KitchenStaff::new() }
    }

    fn take_order(&self, order_contents: String, take_out: bool) -> String {
        let order = self.cashier.take_order(order_contents, take_out);
        let food = self.chef.prepare_food(&order);
        if take_out { return self.kitchen_staff.package_order( food ).get_food().to_string() } else { food.get_food().to_string() }
    }
}
