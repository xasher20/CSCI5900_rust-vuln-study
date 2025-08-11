#include <iostream>
#include <cstring>

int main() {
    char buffer[5];
    strcpy(buffer, "overflow");  // buffer overflow
    std::cout << "Buffer: " << buffer << std::endl;
    return 0;
}