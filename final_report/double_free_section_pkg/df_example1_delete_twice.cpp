#include <iostream>
int main() {
    int* p = new int(7);   // Line 3: allocate
    delete p;               // Line 4: free once
    delete p;               // Line 5: DANGEROUS — double delete (undefined behavior)
    return 0;
}
