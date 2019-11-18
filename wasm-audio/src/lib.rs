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
    primary: web_sys::OscillatorNode,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    /// Amount of frequency modulation
    fm_gain: web_sys::GainNode,

    /// The oscillator that will modulate the primary oscillator's frequency
    fm_osc: web_sys::OscillatorNode,

    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,

    fm_gain_ratio: f32,

}

#[wasm_bindgen]
impl M3dAudio {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<M3dAudio, JsValue> {
        let ctx = AudioContext::new()?;
        // Create our web audio objects.
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain)?;

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination())?;

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain)?;

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(M3dAudio {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    #[wasm_bindgen]
    pub async fn decode(&self, buffer: js_sys::ArrayBuffer) {
        self.ctx.decode_audio_data(&buffer).await
    }

    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }
}


/*#[wasm_bindgen]
pub fn decode(js_buffer: js_sys::ArrayBuffer) {
    let ctx = M3dAudio::new();
    ctx.decode(js_buffer);
}*/

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
