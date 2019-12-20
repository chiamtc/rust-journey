let play = document.querySelector(".play");
// define online and offline audio context
var audioCtx = new AudioContext();
var offlineCtx = new OfflineAudioContext(1, 44100 * 40, 44100);
source = offlineCtx.createBufferSource();
// define variables
var pre = document.querySelector('pre');
var myScript = document.querySelector('script');
var stop = document.querySelector('.stop');

/*
function getData() {

    var nowTime = new Date().getTime()/1000;
    let timer = setInterval(()=>{
        let ms = new Date().getTime()/1000 - nowTime;
        document.getElementById("timer").innerHTML = ms.toFixed(3);
    },100);
    request = new XMLHttpRequest();
    request.open('GET', "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288", true);
    request.responseType = 'arraybuffer';
    request.onload = function () {
        var audioData = request.response;
        audioCtx.decodeAudioData(audioData, function (buffer) {
            myBuffer = buffer;
            source.buffer = myBuffer;
            source.connect(offlineCtx.destination);
            source.start();
            //source.loop = true;
            console.log(offlineCtx);
            offlineCtx.startRendering().then(function (renderedBuffer) {
                console.log('Rendering completed successfully');
                var audioCtx = new (window.AudioContext || window.webkitAudioContext)();
                var song = audioCtx.createBufferSource();


                let bufferSize = renderedBuffer.length;
                let d = [0, 0]; //Array.apply(null, Array(ord)).map(Number.prototype.valueOf,0);
                let input = renderedBuffer.getChannelData(0);
                let outputBuff = audioCtx.createBuffer(renderedBuffer.numberOfChannels, renderedBuffer.length, renderedBuffer.sampleRate)

                const _coef = [
                    {
                        fb: [1, -1.8633348941802979, 0.8801209330558777],
                        ff: [0.09732796562328783, -0.10529189079474921, 0.026142444150071956]
                    },
                    {
                        fb: [1, -1.821143627166748, 0.9694930911064148],
                        ff: [0.9176731674323697, -1.6463434709716742, 0.9176284335265728]
                    },
                    {
                        fb: [1, -1.8136717081069946, 0.9057400822639465],
                        ff: [0.260635334442507, -0.18719826837461115, 0.003026156143995027]
                    },
                    {
                        fb: [1, -1.9527064561843872, 0.9532656073570251],
                        ff: [0.267510268594725, -0.3677071379112296, 0.10019812325780178]
                    }
                ]

                let output = outputBuff.getChannelData(0);
                let maxes = [];
                for (let j = 0; j < _coef.length; j += 1) {
                    for (let i = 0; i < bufferSize; i++) {
                        output[i] = _coef[j].ff[0] * input[i] + d[0];
                        d[0] = _coef[j].ff[1] * input[i] - _coef[j].fb[1] * output[i] + d[1];
                        d[1] = _coef[j].ff[2] * input[i] - _coef[j].fb[2] * output[i];
                        input[i] = output[i];
                        maxes.push(output[i]);
                        output[i] = output[i] * 1;
                    }
                    d[0] = d[1] = 0;
                }
                song.buffer = outputBuff;
                console.log(song.buffer.getChannelData(0))
                song.connect(audioCtx.destination);
                if(song !== undefined) {
                    clearInterval(timer);
                }
                play.onclick = function () {
                    song.start();
                }
            }).catch(function (err) {
                console.log('Rendering failed: ' + err);
                // Note: The promise should reject when startRendering is called a second time on an OfflineAudioContext
            });
        });
    }
    request.send();
}

getData();
*/



import ('./pkg/wasm_audio').then(async (m) => {


    var nowTime = new Date().getTime()/1000;
    let timer = setInterval(()=>{
        let ms = new Date().getTime()/1000 - nowTime;
        document.getElementById("timer").innerHTML = ms.toFixed(3);
    },100);
    const a = m.runner().then(async (data) => {
        const buffer = data;
        // console.log('buffer',buffer)
        let fm = null;
        if (fm === null) {
            fm = new m.M3dAudio();
            const a = new Promise((resolve, reject) => {
                fm.decode(buffer, (res) => res !== null ? resolve(res) : reject("Failed to decode the audio, check WASM code"));
            })

            const audio_buffer = await a;
            let offline_audio_ctx2 = fm.new_offline_ctx(audio_buffer.numberOfChannels, audio_buffer.length, audio_buffer.sampleRate);
            // console.log(audio_buffer.sampleRate);
            offline_audio_ctx2.prep_buffer_and_rendering(audio_buffer).then(function (renderedBuffer) {
                const filtered_buffer= fm.apply_m3d_filter(renderedBuffer);
                const song = fm.prep_buffer_source(filtered_buffer);
                console.log(filtered_buffer.getChannelData(0));
                console.log(fm.attempt_fft(filtered_buffer.getChannelData(0)));
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
