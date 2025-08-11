#include <iostream>
int main() {
    int arr[3] = {0, 0, 0};       // Line 3: valid indices 0..2
    arr[5] = 99;                  // Line 4: DANGEROUS — out-of-bounds WRITE (UB)
    std::cout << arr[5] << "\n"; // Line 5: READs from the invalid slot too (UB)
    return 0;
}
