

// define variables
var pre = document.querySelector('pre');
var myScript = document.querySelector('script');
var play = document.querySelector('.play');
var stop = document.querySelector('.stop');
// use XHR to load an audio track, and
// decodeAudioData to decode it and stick it in a buffer.
// Then we put the buffer into the source
async function getData() {
    // request = new XMLHttpRequest();
    const request = new Request("https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288");
    // request.open('GET', "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288", true);
    const response = await fetch(request);
    const audioData = await response.arrayBuffer();
    var audioCtx = new AudioContext();
    var offlineCtx = new OfflineAudioContext(1, 44100 * 40, 44100);
    source = offlineCtx.createBufferSource();
    audioCtx.decodeAudioData(audioData, function (buffer) {
        source.buffer = buffer;
        source.connect(offlineCtx.destination);
        source.start();
        //source.loop = true;
        console.log(offlineCtx);
        offlineCtx.startRendering().then(function (renderedBuffer) {
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
    });
    // request.responseType = 'arraybuffer';
    /*request.onload = function () {
        var audioData = request.response;
        // define online and offline audio context
        var audioCtx = new AudioContext();
        var offlineCtx = new OfflineAudioContext(1, 44100 * 40, 44100);
        source = offlineCtx.createBufferSource();
        audioCtx.decodeAudioData(audioData, function (buffer) {
            source.buffer = buffer;
            source.connect(offlineCtx.destination);
            source.start();
            //source.loop = true;
            console.log(offlineCtx);
            offlineCtx.startRendering().then(function (renderedBuffer) {
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
        });
    }
    request.send();*/
}

// Run getData to start the process off
getData();
// dump script to pre element
