const rust = import ('./pkg/wasm_audio');

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

rust.then(m => {
    m.runner().then((data) => {
        const buffer = data;
        const ctx = new m.M3dAudio();
        ctx.decode(buffer).then((ac)=>{
            const numberOfChannels = ac.numberOfChannels;
            const length = ac.length;
            const sampleRate = ac.sampleRate;
            const oac = new OfflineAudioContext(numberOfChannels, length, sampleRate);
            console.log('1',oac);
            console.log('2',ac);
        });
    })
}).catch(console.error);
