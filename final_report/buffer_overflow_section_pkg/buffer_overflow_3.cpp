#include <iostream>
int main() {
    int buffer[5] = {0};
    int value = buffer[10]; // compiles fine, out-of-bounds access
    std::cout << "Value: " << value << std::endl;
    return 0;
}
