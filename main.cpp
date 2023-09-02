#include <emscripten.h>

extern "C" void run(const char* code) {
    MAIN_THREAD_EM_ASM({
        eval(UTF8ToString($0));
    }, code);
}

extern "C" {
    extern void runthis();
}

int main() {

    runthis();

}
