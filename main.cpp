#include <emscripten.h>

extern "C" {
    extern void runthis();
}

int main() {

    runthis();
}
