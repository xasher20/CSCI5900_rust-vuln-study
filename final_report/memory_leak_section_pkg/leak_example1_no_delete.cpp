#include <iostream>
int main() {
    int* p = new int(42);   // Line 3: allocate
    // no delete -> memory leak
    std::cout << *p << "\n"; // Line 5: program ends without freeing `p`
    return 0;                // Leak persists until process exit
}
