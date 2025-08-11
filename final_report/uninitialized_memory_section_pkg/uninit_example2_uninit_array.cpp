#include <iostream>
int main() {
    int arr[3];                   // Line 3: uninitialized array
    int sum = 0;
    for (int i = 0; i < 3; ++i) {
        sum += arr[i];            // Line 6: DANGEROUS — reading indeterminate values (UB)
    }
    std::cout << sum << "\n";
    return 0;
}
