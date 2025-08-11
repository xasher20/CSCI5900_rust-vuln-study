#include <iostream>
#include <thread>
#include <vector>
#include <atomic>

int main() {
    int counter = 0;                         // Line 6: shared non-atomic
    const int threads = 8;
    const int iters = 100000;
    std::vector<std::thread> ts;

    for (int t = 0; t < threads; ++t) {
        ts.emplace_back([&]() {
            for (int i = 0; i < iters; ++i) {
                counter++;                   // Line 14: DANGEROUS — data race (UB)
            }
        });
    }
    for (auto& th : ts) th.join();
    std::cout << "counter=" << counter << "\n"; // Line 20: result is nondeterministic
    return 0;
}
