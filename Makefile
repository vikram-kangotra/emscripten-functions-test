main.html: main.cpp libemscripten_function_test.a
	em++ $(LDFLAGS) $^ -o main.html -s ASYNCIFY
	wasm2wat main.wasm -o main.wat

libemscripten_function_test.a: src/lib.rs
	cargo build --target wasm32-unknown-emscripten --release
	cp target/wasm32-unknown-emscripten/release/libemscripten_function_test.a .

clean:
	rm main.html main.js main.wasm libemscripten_function_test.a main.worker.js main.wat
