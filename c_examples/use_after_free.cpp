#include <iostream>

int main() {
    int* ptr = new int(42);
    delete ptr;
    std::cout << *ptr << std::endl;  // use-after-free
    return 0;
}