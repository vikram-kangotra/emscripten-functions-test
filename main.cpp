#include <emscripten.h>
#include <iostream>

extern "C" {
    extern void runthis();
}

int main() {

    const char* x = CODE_EXPR("console.log('Hello World!');");

    std::cout << (void*)x << std::endl;

    const char* ok = "console.log('Hello World!');";

    std::cout << (void*)ok << std::endl;

    runthis();
}
