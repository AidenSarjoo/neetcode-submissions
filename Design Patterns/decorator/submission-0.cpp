class Coffee {
public:
    virtual double getCost() = 0; // Pure virtual function makes this class abstract
};

class SimpleCoffee : public Coffee {
public:
    double getCost() override {
        return 1.1;
    }
};

class CoffeeDecorator : public Coffee {
protected:
    Coffee* decoratedCoffee;

public:
    CoffeeDecorator(Coffee* coffee) : decoratedCoffee(coffee) {}

    double getCost() override {
        return decoratedCoffee->getCost();
    }
};

class MilkDecorator : public CoffeeDecorator {
    double extra_cost = 0.5;
public:
    MilkDecorator(Coffee* coffee) : CoffeeDecorator(coffee) {} 
    double getCost() override {
        return decoratedCoffee->getCost() + extra_cost; 
    }
};

class SugarDecorator : public CoffeeDecorator {
    double extra_cost = 0.2;
public:
    SugarDecorator(Coffee* coffee) : CoffeeDecorator(coffee) {}
    double getCost() override {
        return decoratedCoffee->getCost() + extra_cost; 
    }
};

class CreamDecorator : public CoffeeDecorator {
    double extra_cost = 0.7;
public:
    CreamDecorator(Coffee* coffee) : CoffeeDecorator(coffee) {}
    double getCost() override {
        return decoratedCoffee->getCost() + extra_cost; 
    }
};
