#include <iostream>
int* make_ptr() {
    int x = 123;                 // Line 2: stack local
    return &x;                   // Line 3: DANGEROUS — returns address of dead stack object
}
int main() {
    int* p = make_ptr();         // Line 6: `p` now dangles
    std::cout << *p << "\n";    // Line 7: use of dangling pointer (UB)
    return 0;
}
