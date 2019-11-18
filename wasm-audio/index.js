const rust = import ('./pkg/wasm_audio');

rust.then(m => {
    m.runner().then((data) => {
        /*const ctx = new AudioContext().decodeAudioData(data);
        ctx.then(((res)=>{
            console.log(res)
        }))*/
        const buffer = data;
        const ctx = new m.M3dAudio();
        ctx.decode(buffer).then((res)=>{
            console.log('res',res)
        })
    })
}).catch(console.error);
