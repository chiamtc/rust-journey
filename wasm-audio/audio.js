
/* some docs about web:sys Promise.then(promise_cb) usage.
old promise callback, was using promise_cb when context.oncomplete = function .. in js
let promise_cb = Closure::wrap(Box::new(move |x: JsValue| {
           let b:u32 = JsValue::into_serde(&x).unwrap().length();
           console::log_1(&b.into());
           console::log_1(ab?.into());
           let a = js_sys::Reflect::own_keys(&x).unwrap();
   }) as Box<dyn FnMut(JsValue)>);*/


/* super old code, instantiate offlineaudiocontext without web::sys
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(vendor_prefix = webkit)]
    pub type OfflineAudioContext;

    pub type AudioBufferSourceNode;

    static offline_audio_ctx: OfflineAudioContext;


    #[wasm_bindgen(method, js_name=createBufferSource)]
    fn create_buffer_source(oac:&OfflineAudioContext) -> AudioBufferSourceNode;

    #[wasm_bindgen(constructor)]
    fn new(number_of_channels: u32, length: u32, sample_rate: f32) -> OfflineAudioContext;

}*/
