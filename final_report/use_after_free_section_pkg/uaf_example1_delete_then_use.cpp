#include <iostream>
int main() {
    int* p = new int(42);      // Line 3: allocate
    delete p;                   // Line 4: free
    std::cout << *p << "\n";   // Line 5: DANGEROUS — use-after-free (undefined behavior)
    return 0;
}
