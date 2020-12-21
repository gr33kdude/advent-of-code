#include <iostream>
#include <string>

#include <optional>

template <typename T>
using Option = std::optional<T>;

class Foo {
public:
    Foo(Option<int> b) : b_(b) {}
    Foo() {}
    ~Foo() = default;

    int getB() {
        return b_.value();
    }
    
    operator std::string() const {
        return "FOOBAR";
    }

private:
    Option<int> b_;
};

namespace std {

template <typename T>
std::string to_string(Option<T> o) {
    if (o.has_value())
        return std::to_string(o.value());
    else
        return "NULL";
}

#if 0
std::string to_string(Foo& f) {
    return std::to_string(f.getB());
}
#endif

} // namespace std

int main(void) {
    Option<int> a{7};
    Option<uint_least16_t> b{4};
    Foo f{a};

    std::cout << "foobar: " + std::to_string(b) << std::endl;

    std::cout << std::to_string(b) << std::endl;
    return 0;
}
