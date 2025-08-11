#include <iostream>
#include <vector>
int main() {
    std::vector<int> v = {1, 2, 3};
    int* p = &v[0];                 // Line 5: pointer into vector's buffer
    v.push_back(4);                 // Line 6: may reallocate and move buffer
    std::cout << *p << "\n";       // Line 7: DANGEROUS — `p` may now dangle (UB)
    return 0;
}
