#[no_mangle]
fn runthis() {

    let mut buffer = vec![0 as i8; 640 * 480 * 4];

    for i in 0..buffer.len() {
        buffer[i] = i as i8;
    }

    #[used]
    #[link_section = "em_asm"]
    static CODE: [u8; 1162] = *b"
        console.log($0);

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
                    Module.videoBuffer = new Uint8Array(Module.HEAPU8.buffer, $0, size);

                    console.log(Module.videoBuffer);

                    frameData.copyTo(Module.videoBuffer);

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

}
