#include <iostream>
void free_then_keep(int** out) {
    *out = new int(7);              // Line 2: allocate
    delete *out;                    // Line 3: free
    // returning with *out still pointing to freed memory
}
int main() {
    int* p = nullptr;
    free_then_keep(&p);             // Line 9: p is dangling immediately
    // later...
    std::cout << *p << "\n";       // Line 11: use of dangling pointer (UB)
    return 0;
}
