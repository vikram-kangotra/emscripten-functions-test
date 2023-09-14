#[no_mangle]
pub extern "C" fn handle_video_frame(buffer: *const i8, width: i32, height: i32) {

    println!("width: {}, height: {}", width, height);
}

macro_rules! CODE_EXPR {
    ($code:block) => {
        {
            const C: &[u8] = stringify!($code).as_bytes();
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

macro_rules! MAIN_THREAD_EM_ASM {
    ($code:block) => {
        unsafe {
            emscripten_functions_sys::emscripten::
            emscripten_asm_const_int_sync_on_main_thread(CODE_EXPR!($code), "\0".as_ptr() as *const i8);
        }
    };
}

#[no_mangle]
fn runthis() {

    MAIN_THREAD_EM_ASM!({
        const mediaStream = navigator.mediaDevices.getUserMedia({ video: true });
    });

    /*
    let mut buffer = vec![0 as i8; 640 * 480 * 4];

    for i in 0..buffer.len() {
        buffer[i] = i as i8;
    }

    #[used]
    #[link_section = "em_asm"]
    static CODE: [u8; 1225] = *b"

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
    \0";

    unsafe {
        emscripten_functions_sys::emscripten::
        emscripten_asm_const_int_sync_on_main_thread(CODE.as_ptr() as *const i8, "i\0".as_ptr() as *const i8, buffer.as_mut_ptr() as *const i8);
    }
    */
}
