#include <iostream>
int main() {
    int* p = new int(1);     // Line 3: allocate block A
    p = new int(2);          // Line 4: DANGEROUS — previous pointer lost; block A leaked
    std::cout << *p << "\n";
    delete p;                // Line 6: frees only block B; A still leaked
    return 0;
}
