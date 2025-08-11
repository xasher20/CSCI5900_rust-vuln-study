#include <iostream>
int main() {
    int arr[3] = {1, 2, 3};       // Line 3: valid indices 0..2
    std::cout << arr[5] << "\n"; // Line 4: DANGEROUS — out-of-bounds READ (UB)
    return 0;
}
