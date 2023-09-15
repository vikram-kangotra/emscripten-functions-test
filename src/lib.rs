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

fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

// support for wasm32 only

fn is_pointer<T>(t: T) -> bool {
    type_name_of(&t) == "*const i8" ||
    type_name_of(&t) == "*mut i8" ||
    type_name_of(&t) == "*const u8" ||
    type_name_of(&t) == "*mut u8" ||
    type_name_of(&t) == "*const i16" ||
    type_name_of(&t) == "*mut i16" ||
    type_name_of(&t) == "*const u16" ||
    type_name_of(&t) == "*mut u16" ||
    type_name_of(&t) == "*const i32" ||
    type_name_of(&t) == "*mut i32" ||
    type_name_of(&t) == "*const u32" ||
    type_name_of(&t) == "*mut u32" ||
    type_name_of(&t) == "*const i64" ||
    type_name_of(&t) == "*mut i64" ||
    type_name_of(&t) == "*const u64" ||
    type_name_of(&t) == "*mut u64" ||
    type_name_of(&t) == "*const isize" ||
    type_name_of(&t) == "*mut isize" ||
    type_name_of(&t) == "*const usize" ||
    type_name_of(&t) == "*mut usize" ||
    type_name_of(&t) == "*const f32" ||
    type_name_of(&t) == "*mut f32" ||
    type_name_of(&t) == "*const f64" ||
    type_name_of(&t) == "*mut f64" ||
    type_name_of(&t) == "*const char" ||
    type_name_of(&t) == "*mut char"
}

fn is_interger<T>(t: T) -> bool {
    type_name_of(&t) == "char" ||
    type_name_of(&t) == "i32" ||
    type_name_of(&t) == "i64" ||
    type_name_of(&t) == "u32" ||
    type_name_of(&t) == "u64" ||
    type_name_of(&t) == "i16" ||
    type_name_of(&t) == "u16" ||
    type_name_of(&t) == "i8" ||
    type_name_of(&t) == "u8" ||
    type_name_of(&t) == "isize" ||
    type_name_of(&t) == "usize"
}

fn is_float<T>(t: T) -> bool {
    type_name_of(&t) == "f64" ||
    type_name_of(&t) == "f32"
}

macro_rules! process_arguments {
    () => {
        "\0".to_string()
    };
    ($x:expr, $($rest:tt)*) => {

        match $x {
            _ if is_interger($x) || is_pointer($x) => format!("i{}", process_arguments!($($rest)*)),
            _ if is_float($x) => format!("d{}", process_arguments!($($rest)*)),
            _ => format!("\0") // TODO: provide some error message
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
