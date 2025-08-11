#include <iostream>
int main() {
    int arr[3] = {1, 2, 3};
    int sum = 0;
    for (int i = 0; i <= 3; ++i) {  // Line 5: DANGEROUS — i==3 reads past end
        sum += arr[i];              // Line 6: UB when i==3
    }
    std::cout << sum << "\n";
    return 0;
}
