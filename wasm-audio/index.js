let play = document.querySelector(".play");
import ('./pkg/wasm_audio').then(async (m) => {
    const a = m.runner().then(async (data) => {
        const buffer = data;
        let fm = null;
        let offline_audio_ctx = null;
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


            const audio_buffer = await a;
            let offline_audio_ctx = fm.new_offline_ctx(audio_buffer.numberOfChannels, audio_buffer.length, audio_buffer.sampleRate);
            let offline_audio_ctx2 = fm.new_offline_ctx2(audio_buffer.numberOfChannels, audio_buffer.length, audio_buffer.sampleRate);
            console.log('offline_audio_ctx2.get()',offline_audio_ctx2.get());
            /*const source = offline_audio_ctx.createBufferSource();
            source.buffer = audio_buffer;
            source.connect(offline_audio_ctx.destination);
            source.start();*/

            offline_audio_ctx2 = offline_audio_ctx2.prep_buffer_and_rendering(offline_audio_ctx, audio_buffer);
            offline_audio_ctx2.then(function (renderedBuffer) {
                console.log('Rendering completed successfully');
                var audioCtx = fm.get();//new (window.AudioContext || window.webkitAudioContext)();
                var song = fm.prep_buffer_and_rendering(renderedBuffer);
                // var song = audioCtx.createBufferSource();
                // song.buffer = renderedBuffer;
                // song.connect(audioCtx.destination);
                play.onclick = function () {
                    song.start();
                }
            }).catch(function (err) {
                console.log('Rendering failed: ' + err);
            })

         /*   offline_audio_ctx.startRendering().then(function (renderedBuffer) {
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
            });*/
            // const bSource = m.create_buffer_source(offline_audio_ctx);
            /*    var song;
                // document.getElementById("playbtn").addEventListener('click', async() => {
                const audiobuffer = await offline_audio_ctx.prep_buffer_and_rendering();
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
            // offline_audio_ctx.apply_filter(audiobuffer);

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
