#include <cstdlib>
int main() {
    int* p = (int*)std::malloc(sizeof(int)); // Line 3: allocate
    std::free(p);                             // Line 4: free
    std::free(p);                             // Line 5: DANGEROUS — double free (often exploitable)
    return 0;
}
