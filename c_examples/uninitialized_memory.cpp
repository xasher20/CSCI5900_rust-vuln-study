#include <iostream>

int main() {
    int x;  // uninitialized
    std::cout << x << std::endl;  // undefined behavior
    return 0;
}