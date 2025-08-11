#include <memory>
#include <iostream>

struct Node {
    std::shared_ptr<Node> next; // Line 5: owning strong ref creates potential cycle
    ~Node() { std::cout << "~Node\n"; }
};

int main() {
    auto a = std::make_shared<Node>();
    auto b = std::make_shared<Node>();
    a->next = b;              // Line 12
    b->next = a;              // Line 13: cycle: a -> b -> a
    // Both `a` and `b` go out of scope, but refcounts never hit zero -> leak
    return 0;                 // No destructor prints if leaked
}
