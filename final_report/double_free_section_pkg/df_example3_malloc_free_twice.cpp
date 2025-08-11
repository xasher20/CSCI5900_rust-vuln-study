#include <cstdlib>
int main() {
    void* p = std::malloc(16); // Line 3: allocate
    std::free(p);              // Line 4: free
    std::free(p);              // Line 5: DANGEROUS — double free (UB, often exploitable)
    return 0;
}
