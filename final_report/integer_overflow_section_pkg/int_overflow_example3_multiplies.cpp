#include <iostream>
#include <climits>
int main() {
    int x = 50000;               // Line 4
    int y = 50000;               // Line 5
    long long p = (long long)x * y; // Line 6: safe: promote before multiply
    int q = x * y;               // Line 7: DANGEROUS — signed overflow (UB)
    std::cout << "p=" << p << " q=" << q << "\n";
    return 0;
}
