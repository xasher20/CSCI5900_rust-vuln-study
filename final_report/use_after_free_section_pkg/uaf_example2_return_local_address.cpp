#include <iostream>
int* make_ptr() {
    int x = 99;                 // Line 3: local variable
    return &x;                  // Line 4: DANGEROUS — address of dead stack object
}
int main() {
    int* p = make_ptr();
    std::cout << *p << "\n";   // Line 8: use-after-lifetime / dangling pointer (UB)
    return 0;
}
