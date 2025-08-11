int* leak() {
    int* ptr = new int(42);  // no delete
    return ptr;
}