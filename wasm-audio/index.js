import ('./pkg/wasm_audio').then(m => {
    m.runner().then((data) => {
        const buffer = data;
        let fm = null;
        if (fm === null) {
            fm = new m.M3dAudio();
            console.log(fm);
            fm.decode(buffer).then((ac) => {
               /* const numberOfChannels = ac.numberOfChannels;
                const length = ac.length;
                const sampleRate = ac.sampleRate;
                const oac = new OfflineAudioContext(numberOfChannels, length, sampleRate);
                console.log('1', oac);*/
                console.log('2', ac);
            });
        } else {
            fm.free();
            fm = null;
        }

    })

}).catch(console.error);

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
