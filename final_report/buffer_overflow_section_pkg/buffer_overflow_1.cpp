#include <iostream>
int main() {
    int buffer[5] = {0};           // fixed-size buffer
    buffer[10] = 42;               // out-of-bounds write
    std::cout << "Buffer: " << buffer[10] << std::endl;
    return 0;
}
