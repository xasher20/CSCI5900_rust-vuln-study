#include <iostream>
using namespace std;
int main() {
    char buffer[5];
    for (int i = 0; i < 10; ++i) {
        buffer[i] = 'A';  // out-of-bounds write
    }
    buffer[4] = '\0';     // attempt to null-terminate
    cout << "Buffer content: " << buffer << endl;
    return 0;
}
