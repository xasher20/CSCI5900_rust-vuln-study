#include <iostream>
int main() {
    int* p = new int(42);   // Line 3: allocate
    int* q = p;              // Line 4: alias raw pointer to same allocation
    delete p;                // Line 5: free
    delete q;                // Line 6: DANGEROUS — double delete via alias (UB)
    return 0;
}
