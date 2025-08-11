#include <memory>
#include <iostream>

struct Node {
    std::weak_ptr<Node> next;  // Line 5: use weak_ptr to break cycle
    ~Node() { std::cout << "~Node\n"; }
};

int main() {
    auto a = std::make_shared<Node>();
    auto b = std::make_shared<Node>();
    a->next = b;               // Line 12: store weak ref
    b->next = a;               // Line 13
    return 0;                  // Destructors run; no leak
}
