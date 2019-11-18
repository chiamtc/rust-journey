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
        ctx.decode(buffer).then((res)=>{
            console.log('res',res)
        })
    })
}).catch(console.error);
