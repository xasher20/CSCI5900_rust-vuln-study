#include <iostream>
#include <climits>
int main() {
    int a = INT_MAX;             // Line 4: 2147483647 on 32-bit int
    int b = 1;
    int c = a + b;               // Line 6: DANGEROUS — signed overflow is undefined behavior
    std::cout << "c=" << c << "\n"; // May print a negative value or be optimized unpredictably
    return 0;
}
