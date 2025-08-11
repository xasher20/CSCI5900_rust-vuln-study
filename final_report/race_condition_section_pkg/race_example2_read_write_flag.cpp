#include <iostream>
#include <thread>
#include <chrono>
#include <atomic>

bool running = true;                        // Line 6: shared non-atomic flag

void worker() {
    size_t spins = 0;
    while (running) {                       // Line 10: read race with main writer
        ++spins;
    }
    std::cout << "worker spins=" << spins << "\n";
}

int main() {
    std::thread th(worker);
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    running = false;                        // Line 19: write without synchronization
    th.join();
    return 0;
}
