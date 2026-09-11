class Singleton {
private:

    Singleton() {}
    string value;

public:

    static Singleton *getInstance() {
        static Singleton instance;
        return &instance;
    }

    string getValue() {
        return this->value;
    }

    void setValue(string &value) {
        this->value = value;
    }
};
