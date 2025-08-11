#include <iostream>

int* get_null() {
    return nullptr;
}

int main() {
    int* ptr = get_null();
    std::cout << *ptr << std::endl;  // null pointer dereference
    return 0;
}