import ('./pkg/wasm_audio').then(m => {
    m.runner().then(async (data) => {
        const buffer = data;
        let fm = null;
        if (fm === null) {
            fm = new m.M3dAudio();
            /*  fm.decode(buffer).then((ac) => {
                const numberOfChannels = ac.numberOfChannels;
                 const length = ac.length;
                 const sampleRate = ac.sampleRate;
                 const oac = new OfflineAudioContext(numberOfChannels, length, sampleRate);
                 console.log('1', oac);
                console.log('2', ac);
            });*/;

            const a = new Promise((resolve, reject) => {
                fm.decode(buffer, (res) => res !== null ? resolve(res) : reject("Failed to decode the audio, check WASM code"));
            })

            const ctx = await a;
            // const oad = m.M3dAudio.new_offline_ctx(ctx.numberOfChannels, ctx.length, ctx.sampleRate);
            const oad = m.new_offline_ctx(ctx.numberOfChannels, ctx.length, ctx.sampleRate);
            console.log(oad)

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
