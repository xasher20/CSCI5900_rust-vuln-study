#include <iostream>
#include <climits>
int main() {
    unsigned int u = UINT_MAX;   // Line 4: e.g., 4294967295 on 32-bit
    u = u + 1;                   // Line 5: well-defined wraparound to 0
    std::cout << "u=" << u << "\n";
    return 0;
}
