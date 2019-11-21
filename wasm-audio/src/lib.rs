extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use futures::{future, Future};
use js_sys::{ArrayBuffer, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use web_sys::{AudioBufferSourceNode, OfflineAudioContext, OfflineAudioContextOptions, AudioContext, OscillatorType, Request, RequestInit, RequestMode, Response, console};
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
    b_source:AudioBufferSourceNode
//    offlineCtx:OfflineAudioContext
    //TODO add filter
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
pub fn new_offline_ctx(number_of_channels: u32, length: u32, sample_rate: f32, buffer:ArrayBuffer) -> Result<M3dOfflineAudio, JsValue> {
    let off_ctx = web_sys::OfflineAudioContext::new_with_context_options(&web_sys::OfflineAudioContextOptions::new(length, sample_rate))?;
    let b_source = off_ctx.create_buffer_source()?;
    let destination = off_ctx.destination();
    b_source.connect_with_audio_node(&destination);
    b_source.set_buffer(buffer);
    Ok(M3dOfflineAudio { ctx: off_ctx, b_source})
}

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
    pub fn decode(&self, buffer: js_sys::ArrayBuffer, decodeCallback: &js_sys::Function) -> Result<js_sys::Promise, JsValue> {
        self.ctx.decode_audio_data_with_success_callback(&buffer, decodeCallback)
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
