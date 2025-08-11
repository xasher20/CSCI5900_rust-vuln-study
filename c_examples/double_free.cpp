#include <cstdlib>

int main() {
    int* ptr = (int*)malloc(sizeof(int));
    free(ptr);
    free(ptr);  // double free
    return 0;
}