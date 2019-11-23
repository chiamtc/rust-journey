extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use js_sys::{ArrayBuffer, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use web_sys::{AudioBufferSourceNode, AudioBuffer, OfflineAudioContext, AudioContext, Request, RequestInit, RequestMode, Response, console};
//use web_sys::OfflineAudioContext;

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
///


/*
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
#[wasm_bindgen]
pub struct M3dAudio {
    //    #[wasm_bindgen(vendor_prefix = webkit)]
    ctx: AudioContext,
//    offlineCtx:OfflineAudioContext
    //TODO add filter
}

#[wasm_bindgen]
pub struct M3dOfflineAudio {
    //    #[wasm_bindgen(vendor_prefix = webkit)]
    ctx: OfflineAudioContext,
//    b_source: AudioBufferSourceNode,

//    offlineCtx:OfflineAudioContext
    //TODO add filter
}

#[wasm_bindgen]
impl M3dOfflineAudio {
    #[wasm_bindgen(constructor)]
    pub fn new(offctx: OfflineAudioContext) -> Result<M3dOfflineAudio, JsValue> {
        Ok(M3dOfflineAudio { ctx: offctx })
    }

    #[wasm_bindgen]
    pub fn prep_buffer_and_rendering(&self) -> Result<OfflineAudioContext,JsValue>{
        let b_source = self.ctx.create_buffer_source().unwrap();
        let destination = self.ctx.destination();
        b_source.connect_with_audio_node(&destination).unwrap();

        let new_buffer_opts = web_sys::AudioBufferOptions::new(self.ctx.length(), self.ctx.sample_rate());
        let new_buffer = web_sys::AudioBuffer::new(&new_buffer_opts).unwrap();
        b_source.set_buffer(Some(&new_buffer));

        let promise_cb = Closure::wrap(Box::new(move |x: JsValue| {
//            let b:u32 = JsValue::into_serde(&x).unwrap().length();
//            console::log_1(&b.into());
//            console::log_1(ab?.into());
//            let a = js_sys::Reflect::own_keys(&x).unwrap();
//            console::log_1(&a.into());

//            console::log_1(&x.into());
            console::log_1(&x.into());

            /*
            let obj = js_sys::Object::new();
            let a = js_sys::Reflect::set(&obj, &"foo".into(), &x.into()).unwrap();
            let ab = js_sys::Reflect::get(&obj, &"foo".into()).unwrap();*/
            /* let b= match x.dyn_into(){
                 Ok(t) => {
                     let c:AudioBuffer= t.into_serde().unwrap();
                     JsValue::from("aaa")
                 },
                 Err(E)=>{
                     JsValue::from("error")
                 }
             };*/
//            x.into_serde().unwrap()
        }) as Box<dyn FnMut(JsValue)>);


        /*let b = |x: ArrayBuffer| {
            console::log_1(&x.into());
        };*/

        b_source.start();
//        Ok(self.ctx.start_rendering()?)

        Ok(self.ctx.clone())
//        promise_cb.forget();
//        Ok(())
//    let complete_func = &b;

//        let c = js_sys::Function::new_with_args("off_ctx", "buffer"); //buffer undefined
//    let c = js_sys::Function::new_no_args("off_ctx");

//    off_ctx.set_oncomplete(Some(&c));
//         self.ctx.set_oncomplete(Some(&c));
        /* let e = match self.ctx.oncomplete() {
             Some(t) => {
                 console::log_1(&t.into());
     //            console::log_1(t.call0(&JsValue::from(buffer)));
             },
             None => ()
         };*/
    }

    #[wasm_bindgen]
    pub fn apply_filter(&self, audio_buffer: AudioBuffer) {
        let buffer_size = audio_buffer.length();
        let mut d = [0,0];
        /*let coef =vec![
            []
        ];*/
    /*    let coef = [
            {
                fb: [1, -1.4791464805603027, 0.6930942535400391],
                ff: [0.35, -0.4605122089385986, 0.11051515042781829]
            },
            {
                fb: [1, -1.785384178161621, 0.7876397967338562],
                ff: [0.35, -0.39466336369514465, 0.4124599575996399]
            },
            {
                fb: [1, -1.38728928565979, 0.8583449721336365],
                ff: [0.35, -0.46513869166374205, 0.3464472651481628]
            },
            {
                fb: [1, -1.3877276182174683, 0.9699763059616089],
                ff: [0.35, 0.29919922947883604, 0.04006841853260994]
            }
        ];*/

    }
}

/*
#[wasm_bindgen]
pub fn new_offline_ctx(number_of_channels: u32, length: u32, sample_rate: f32) -> OfflineAudioContext {
    OfflineAudioContext::new(number_of_channels, length, sample_rate)
}

#[wasm_bindgen]
pub fn create_buffer_source(oac:OfflineAudioContext) -> AudioBufferSourceNode{
    oac.create_buffer_source()
}*/


#[wasm_bindgen]
impl M3dAudio {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<M3dAudio, JsValue> {
        let ctx = web_sys::AudioContext::new()?;
        Ok(M3dAudio {
            ctx
        })
    }

    //OG working code to return a promise
    #[wasm_bindgen]
    pub fn decode(&self, buffer: js_sys::ArrayBuffer, decode_cb: &js_sys::Function) -> Result<js_sys::Promise, JsValue> {
        self.ctx.decode_audio_data_with_success_callback(&buffer, decode_cb)
    }

    #[wasm_bindgen]
    pub fn new_offline_ctx(&self, number_of_channels: u32, length: u32, sample_rate: f32) -> OfflineAudioContext {
        let off_ctx = web_sys::OfflineAudioContext::new_with_number_of_channels_and_length_and_sample_rate(number_of_channels, length, sample_rate).unwrap();
        off_ctx
//        M3dOfflineAudio::new(off_ctx)
    }

    #[wasm_bindgen]
    pub fn prep_buffer_and_rendering(&self,offline_audio_context:OfflineAudioContext, audio_buffer:AudioBuffer) -> Result<OfflineAudioContext,JsValue> {
        let b_source = offline_audio_context.create_buffer_source().unwrap();
        b_source.set_buffer(Some(&audio_buffer));
        let destination = offline_audio_context.destination();
        b_source.connect_with_audio_node(&destination).unwrap();

        b_source.start();
//        let new_buffer_opts = web_sys::AudioBufferOptions::new(self.ctx.length(), self.ctx.sample_rate());
//        let new_buffer = web_sys::AudioBuffer::new(&new_buffer_opts).unwrap();


        let promise_cb = Closure::wrap(Box::new(move |x: JsValue| {
//            let b:u32 = JsValue::into_serde(&x).unwrap().length();
//            console::log_1(&b.into());
//            console::log_1(ab?.into());
//            let a = js_sys::Reflect::own_keys(&x).unwrap();
//            console::log_1(&a.into());

//            console::log_1(&x.into());
            console::log_1(&x.into());

            /*
            let obj = js_sys::Object::new();
            let a = js_sys::Reflect::set(&obj, &"foo".into(), &x.into()).unwrap();
            let ab = js_sys::Reflect::get(&obj, &"foo".into()).unwrap();*/
            /* let b= match x.dyn_into(){
                 Ok(t) => {
                     let c:AudioBuffer= t.into_serde().unwrap();
                     JsValue::from("aaa")
                 },
                 Err(E)=>{
                     JsValue::from("error")
                 }
             };*/
//            x.into_serde().unwrap()
        }) as Box<dyn FnMut(JsValue)>);


        /*let b = |x: ArrayBuffer| {
            console::log_1(&x.into());
        };*/

//        Ok(self.ctx.start_rendering()?)

        Ok(offline_audio_context)
    }
}

#[wasm_bindgen(js_name = "runner")]
pub async fn run() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288",
        &opts,
    )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.array_buffer()?).await?;

    Ok(JsValue::from(json))
}
