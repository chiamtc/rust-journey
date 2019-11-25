let play = document.querySelector(".play");

import ('./pkg/wasm_audio').then(async (m) => {

    var nowTime = new Date().getTime()/1000;
    let timer = setInterval(()=>{
        let ms = new Date().getTime()/1000 - nowTime;
        document.getElementById("timer").innerHTML = ms.toFixed(3);
    },100);

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
                const filtered_buffer= fm.apply_m3d_filter(renderedBuffer);
                const song = fm.prep_buffer_source(filtered_buffer);
                console.log(song.buffer.getChannelData(0));
                if(song !== undefined) {
                    clearInterval(timer);
                }
                play.onclick = () => {
                    console.log('click');
                    song.start();
                }
            }).catch(function (err) {
                console.log('Rendering failed: ' + err);
            })
        } else {
            fm.free();
            fm = null;
        }

    })
});
