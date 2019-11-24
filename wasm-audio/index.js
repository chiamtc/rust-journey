let play = document.querySelector(".play");
import ('./pkg/wasm_audio').then(async (m) => {
    const a = m.runner().then(async (data) => {
        const buffer = data;
        let fm = null;
        if (fm === null) {
            fm = new m.M3dAudio();
            const a = new Promise((resolve, reject) => {
                fm.decode(buffer, (res) => res !== null ? resolve(res) : reject("Failed to decode the audio, check WASM code"));
            })


            const audio_buffer = await a;
            let offline_audio_ctx2 = fm.new_offline_ctx(audio_buffer.numberOfChannels, audio_buffer.length, audio_buffer.sampleRate);
            offline_audio_ctx2.prep_buffer_and_rendering(audio_buffer).then(function (renderedBuffer) {
                const song = fm.prep_buffer_source(renderedBuffer);
                /*
                * TODO:
                * 1. get filter coefficients from JS format into Rust format (tuples , array or vec or hashmap)
                * 2. use renderedBuffer variable to apply the filter in Rust. Ideally, M3dAudio struct should have a method called apply_filter
                * Signature : apply_filter(self:M3dAudio, coefficients: TBD type) -> web_sys:: AudioBuffer
                * 3. hopes that play.onClick has the actual buffer with custom filter
                * */
                // var song = audioCtx.createBufferSource();
                // song.buffer = renderedBuffer;
                // song.connect(audioCtx.destination);
                play.onclick = () => song.start();
            }).catch(function (err) {
                console.log('Rendering failed: ' + err);
            })
        } else {
            fm.free();
            fm = null;
        }

    })
});
