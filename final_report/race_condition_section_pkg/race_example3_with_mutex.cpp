#include <iostream>
#include <thread>
#include <vector>
#include <mutex>

int main() {
    long long counter = 0;
    std::mutex m;                            // Line 8: mutex protects shared state
    const int threads = 8;
    const int iters = 100000;
    std::vector<std::thread> ts;

    for (int t = 0; t < threads; ++t) {
        ts.emplace_back([&]() {
            for (int i = 0; i < iters; ++i) {
                std::lock_guard<std::mutex> lock(m); // Line 16: synchronized
                counter++;
            }
        });
    }
    for (auto& th : ts) th.join();
    std::cout << "counter=" << counter << "\n";     // Deterministic: 8*100000
    return 0;
}
