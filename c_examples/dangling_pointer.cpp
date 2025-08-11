int* createPointer() {
    int x = 42;
    return &x;  // dangling
}

int main() {
    int* ptr = createPointer();
    std::cout << *ptr << std::endl;
}