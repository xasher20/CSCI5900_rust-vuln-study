#include <iostream>
int main() {
    int x;                       // Line 3: uninitialized local (indeterminate value)
    std::cout << x << "\n";     // Line 4: DANGEROUS — use of uninitialized variable (UB)
    return 0;
}
