#[no_mangle]
fn runthis() {

    let _buffer = vec![0 as i8; 640 * 480 * 4];

    emscripten_functions::emscripten::
    main_thread_em_asm("
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
                    const buffer = new Uint8Array(size);

                    frameData.copyTo(buffer);

                    console.log(buffer);

                    frameData.close();

                    readFrame();
                });
            }

            readFrame();
        }
    ");
}
