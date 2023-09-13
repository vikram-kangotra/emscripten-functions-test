main.html: main.cpp libemscripten_function_test.a
	em++ $(LDFLAGS) $^ -o main.html
	wasm2wat main.wasm -o main.wat
main.html: LDFLAGS += \
	-s EXPORTED_FUNCTIONS='["_main", "_handle_video_frame"]'

libemscripten_function_test.a: src/lib.rs
	cargo build --target wasm32-unknown-emscripten --release
	cp target/wasm32-unknown-emscripten/release/libemscripten_function_test.a .

clean:
	rm main.html main.js main.wasm libemscripten_function_test.a main.worker.js main.wat
