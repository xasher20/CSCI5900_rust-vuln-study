#include <iostream>
int main() {
    int* p = nullptr;            // Line 3: null pointer
    std::cout << *p << "\n";    // Line 4: DANGEROUS — null pointer dereference (crash/segfault)
    return 0;
}
