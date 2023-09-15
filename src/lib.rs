macro_rules! CODE_EXPR {
    ($code:expr) => {
        {
            const C: &[u8] = $code.as_bytes();
            const C_LEN: usize = C.len();

            const BUF_LEN: usize = C_LEN + 1;

            #[used]
            #[link_section = "em_asm"]
            static BUF: [u8; BUF_LEN] = {
                let mut arr = [0u8; BUF_LEN];
                let mut idx = 0;
                while idx < C_LEN {
                    arr[idx] = C[idx];
                    idx += 1;
                }
                arr[idx] = b'\0';
                arr
            };

            BUF.as_ptr() as *const i8
        }
    };
}

fn type_name_of<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

// support for wasm32 only

fn is_pointer<T>(t: T) -> bool {
    type_name_of(&t).starts_with("*const") || type_name_of(&t).starts_with("*mut")
}

fn is_interger<T>(t: T) -> bool {
    match type_name_of(&t) {
        "char" | "i32" | "i64" | "u32" | "u64" | "i16" | "u16" | "i8" | "u8" | "isize" | "usize" => true,
        _ => false
    }
}

fn is_float<T>(t: T) -> bool {
    match type_name_of(&t) {
        "f32" | "f64" => true,
        _ => false
    }
}

macro_rules! process_arguments {
    () => {
        "\0".to_string()
    };
    ($x:expr, $($rest:tt)*) => {

        match $x {
            _ if is_interger($x) || is_pointer($x) => format!("i{}", process_arguments!($($rest)*)),
            _ if is_float($x) => format!("d{}", process_arguments!($($rest)*)),
            _ => {
                panic!("Unsupported type: {}", type_name_of(&$x));
            }
        }
    };
}

macro_rules! MAIN_THREAD_EM_ASM {
    ($code:expr, $($rest:tt)*) => {
        unsafe {
            emscripten_functions_sys::emscripten::
            emscripten_asm_const_int_sync_on_main_thread(
                CODE_EXPR!($code), 
                process_arguments!($($rest)*).as_ptr() as *const i8, 
                $($rest)*);
        }
    };
}

#[no_mangle]
pub extern "C" fn handle_video_frame(buffer: *const i8, width: i32, height: i32) {

    println!("width: {}, height: {}", width, height);
}

#[no_mangle]
fn runthis() {

    MAIN_THREAD_EM_ASM!(r#"
        console.log($0);
        console.log($1);
        console.log($2);
        console.log(UTF8ToString($2));
    "#, 123, 12.34, "dsnfkd\0".as_ptr(),);

    MAIN_THREAD_EM_ASM!(r#"

         const mediaStream = navigator.mediaDevices.getUserMedia({ video: true });
        mediaStream.then((stream) => {
            const videoTrack = stream.getVideoTracks()[0];
            const videoProcessor = new MediaStreamTrackProcessor({ track: videoTrack });
            const videoData = videoProcessor.readable;

            handleVideoData(videoData);
        });

        function handleVideoData(videoData) {
            const reader = videoData.getReader();

            function readFrame() {
                reader.read().then(({ done, value }) => {
                    if (done) {
                        return;
                    }

                    const frameData = value;

                    const size = frameData.codedWidth * frameData.codedHeight * 4;

                    const arrayBuffer = new ArrayBuffer(size);
                    let videoBuffer = new Uint8Array(arrayBuffer);

                    frameData.copyTo(videoBuffer);

                    Module._handle_video_frame(videoBuffer, frameData.codedWidth, frameData.codedHeight);

                    frameData.close();

                    readFrame();
                });
            }

            readFrame();
        }
    "#, 8055,);
}
