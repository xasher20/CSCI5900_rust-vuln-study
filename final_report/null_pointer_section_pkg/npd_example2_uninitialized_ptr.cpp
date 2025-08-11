#include <iostream>
int main() {
    int* p;                       // Line 3: uninitialized pointer (indeterminate value)
    std::cout << *p << "\n";     // Line 4: DANGEROUS — undefined behavior (may crash or print garbage)
    return 0;
}
