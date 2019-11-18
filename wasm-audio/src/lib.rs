use futures::{future, Future};
use js_sys::{ArrayBuffer, Promise};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use web_sys::{AudioContext,OscillatorType, Request, RequestInit, RequestMode, Response, console};

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
///
#[wasm_bindgen]
pub struct M3dAudio {
    ctx: AudioContext,
    //TODO add filter

}

#[wasm_bindgen]
impl M3dAudio {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<M3dAudio, JsValue> {
        let ctx = AudioContext::new()?;

        Ok(M3dAudio {
            ctx,
        })
    }

   /*
    OG working code to return a promise
    #[wasm_bindgen]
    pub fn decode(&self, buffer: js_sys::ArrayBuffer) -> Result<js_sys::Promise, JsValue>{
        self.ctx.decode_audio_data(&buffer)
    }*/


    #[wasm_bindgen]
    pub fn decode(&self, buffer: js_sys::ArrayBuffer) -> Result<JsValue, JsValue>{
        self.ctx.decode_audio_data(&buffer)
    }
}

#[wasm_bindgen(js_name = "runner")]
pub fn run() -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288",
        &opts,
    ).unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.array_buffer()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
//            console::log_1(&json.into());
            assert!(json.is_instance_of::<ArrayBuffer>());
//            console::log_1(&AudioContext::new().unwrap().buffer);
//            let audioCtx = AudioContext::new().unwrap();
//            let decoded = audioCtx.decode_audio_data(&json.into());
//            JsFuture::from(decoded)
//            let m3d_ctx = M3dAudio::new();
//            m3d_ctx.decode()
//            json
//            console::log_1(&audioCtx);
            future::ok(JsValue::from(json))
        });
    future_to_promise(future)
}
