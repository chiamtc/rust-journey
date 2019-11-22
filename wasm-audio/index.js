let play = document.querySelector(".play");

import ('./pkg/wasm_audio').then(async (m) => {

    let oad = new OfflineAudioContext(1, 44100 * 40, 44100);
    const source = oad.createBufferSource();
    const a = m.runner().then(async (data) => {
        const buffer = data;
        let fm = null;
        let oad = null;
        if (fm === null) {
            fm = new m.M3dAudio();
            /*  fm.decode(buffer).then((ac) => {
                const numberOfChannels = ac.numberOfChannels;
                 const length = ac.length;
                 const sampleRate = ac.sampleRate;
                 const oac = new OfflineAudioContext(numberOfChannels, length, sampleRate);
                 console.log('1', oac);
                console.log('2', ac);
            });*/

            const a = new Promise((resolve, reject) => {
                fm.decode(buffer, (res) => res !== null ? resolve(res) : reject("Failed to decode the audio, check WASM code"));
            })


            const ctx = await a;

            source.buffer = ctx;
            source.connect(oad.destination);
            oad.startRendering().then(function (renderedBuffer) {
                console.log('Rendering completed successfully');
                var audioCtx = new (window.AudioContext || window.webkitAudioContext)();
                var song = audioCtx.createBufferSource();
                song.buffer = renderedBuffer;
                song.connect(audioCtx.destination);
                play.onclick = function () {
                    song.start();
                }
            }).catch(function (err) {
                console.log('Rendering failed: ' + err);
                // Note: The promise should reject when startRendering is called a second time on an OfflineAudioContext
            });
            // const bSource = m.create_buffer_source(oad);
        /*    var song;
            // document.getElementById("playbtn").addEventListener('click', async() => {
            const audiobuffer = await oad.prep_buffer_and_rendering();
*/
          /*  audiobuffer.startRendering().then(function (buffer) {
                var audioCtx = new AudioContext();
                source = audioCtx.createBufferSource();
                source.buffer = buffer;
                const script = audioCtx.createScriptProcessor(1024, 1, 1);
                source.connect(script);
                console.log(audioCtx)
                script.connect(audioCtx.destination);
                source.connect(audioCtx.destination);
                play.onclick = function () {
                    source.start(0);

                }
                // buffer contains the output buffer
            });*/
            // const myScript = document.querySelector('script');
            // document.querySelector('pre').innerHTML = myScript.innerHTML;
            // console.log('audiobuffer',audiobuffer.start())


            // })
            // oad.apply_filter(audiobuffer);

        } else {
            fm.free();
            fm = null;
        }

    })

    /*const audiobuffer = await a;
    audiobuffer.startRendering().then(function (buffer) {
        var audioCtx = new (window.AudioContext)()
        let song = audioCtx.createBufferSource();
        song.buffer = buffer;
        // const script = audioCtx.createScriptProcessor(1024, 1, 1);
        // song.connect(script);
        console.log(audioCtx.destination)
        // script.connect(audioCtx.destination);
        song.connect(audioCtx.destination);
        play.onclick = function () {
            song.start();

        }
        // buffer contains the output buffer
    }).catch(function (err) {
        console.log('err', err)
    });*/
});

/* OG working decoded audio context
rust.then(m => {
    m.runner().then((data) => {
        const buffer = data;
        const ctx = new m.M3dAudio();
        ctx.decode(buffer).then((res)=>{
            console.log('res',res)
        })
    })
}).catch(console.error);
*/
/*

 */
