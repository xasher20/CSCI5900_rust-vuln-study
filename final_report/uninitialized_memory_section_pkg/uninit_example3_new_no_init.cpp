#include <iostream>
int main() {
    int* p = new int;            // Line 3: uninitialized heap allocation
    std::cout << *p << "\n";    // Line 4: DANGEROUS — reading uninitialized memory (UB)
    delete p;
    return 0;
}
