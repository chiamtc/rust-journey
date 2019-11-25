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

use std::ops::Index;

enum Nucleotide {
    fb,
    ff,
}

#[derive(Debug)]
struct Coefs {
    fb: Vec<f32>,
    ff: Vec<f32>,
}

impl Coefs {
    pub fn get(&self, iteration: u32) {}
}
/*
impl Index<Nucleotide> for Coefs {
    type Output = usize;

    fn index(&self, nucleotide: Nucleotide) -> &Self::Output {
        match nucleotide {
            Nucleotide::fb => &self.fb,
            Nucleotide::ff => &self.ff
        }
    }
}*/


#[wasm_bindgen]
pub struct M3dAudio {
    ctx: AudioContext,
}

#[wasm_bindgen]
pub struct M3dOfflineAudio {
    ctx: OfflineAudioContext,
}

#[wasm_bindgen]
impl M3dOfflineAudio {
    #[wasm_bindgen(constructor)]
    pub fn new(offctx: OfflineAudioContext) -> Result<M3dOfflineAudio, JsValue> {
        Ok(M3dOfflineAudio { ctx: offctx })
    }

    #[wasm_bindgen(method)]
    pub fn get(&self) -> web_sys::OfflineAudioContext {
        self.ctx.clone()
    }

    #[wasm_bindgen]
    pub fn prep_buffer_and_rendering(&self, audio_buffer: AudioBuffer) -> Result<js_sys::Promise, JsValue> {
        let b_source = self.get().create_buffer_source().unwrap();
        b_source.set_buffer(Some(&audio_buffer));
        let destination = self.get().destination();
        b_source.connect_with_audio_node(&destination).unwrap();
        b_source.start();
        Ok(self.get().start_rendering()?)
    }


    #[wasm_bindgen]
    pub fn apply_filter(&self, audio_buffer: AudioBuffer) {
        let buffer_size = audio_buffer.length();
        let mut d = [0, 0];
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

    #[wasm_bindgen(method)]
    pub fn get(&self) -> web_sys::AudioContext {
        self.ctx.clone()
    }

    #[wasm_bindgen]
    pub fn decode(&self, buffer: js_sys::ArrayBuffer, decode_cb: &js_sys::Function) -> Result<js_sys::Promise, JsValue> {
        self.ctx.decode_audio_data_with_success_callback(&buffer, decode_cb)
    }

    #[wasm_bindgen]
    pub fn new_offline_ctx(&self, number_of_channels: u32, length: u32, sample_rate: f32) -> Result<M3dOfflineAudio, JsValue> {
        let off_ctx = web_sys::OfflineAudioContext::new_with_number_of_channels_and_length_and_sample_rate(number_of_channels, length, sample_rate).unwrap();
        M3dOfflineAudio::new(off_ctx)
    }

    #[wasm_bindgen]
    pub fn prep_buffer_source(&self, audio_buffer: AudioBuffer) -> web_sys::AudioBufferSourceNode {//-> Result<js_sys::Promise, JsValue> {
        let b_source = self.ctx.create_buffer_source().unwrap();
        b_source.set_buffer(Some(&audio_buffer));
        let destination = self.get().destination();
        b_source.connect_with_audio_node(&destination).unwrap();
        b_source
        /*
        was using promise_cb when context.oncomplete = function .. in js
        let promise_cb = Closure::wrap(Box::new(move |x: JsValue| {
 //            let b:u32 = JsValue::into_serde(&x).unwrap().length();
 //            console::log_1(&b.into());
 //            console::log_1(ab?.into());
 //            let a = js_sys::Reflect::own_keys(&x).unwrap();
         }) as Box<dyn FnMut(JsValue)>);*/
    }

    #[wasm_bindgen]
    pub fn apply_m3d_filter(&self, audio_buffer: AudioBuffer) -> web_sys::AudioBuffer { //-> Vec<f32> {
        let length = audio_buffer.length() as u32;
        let mut channel_data: Vec<f32> = audio_buffer.get_channel_data(0).unwrap();
        let mut d: Vec<f32> = vec![0.0, 0.0];

        let output_buff = self.ctx.create_buffer(audio_buffer.number_of_channels(), audio_buffer.length(), audio_buffer.sample_rate()).unwrap();
        let mut output = output_buff.get_channel_data(0).unwrap();
        let extended_coefs: Vec<Coefs> = vec![
            Coefs {
                fb: vec![1.0, -1.4791464805603027, 0.6930942535400391],
                ff: vec![0.35, -0.4605122089385986, 0.11051515042781829],
            },
            Coefs {
                fb: vec![1.0, -1.785384178161621, 0.7876397967338562],
                ff: vec![0.35, -0.39466336369514465, 0.4124599575996399]
            },
            Coefs {
                fb: vec![1.0, -1.38728928565979, 0.8583449721336365],
                ff: vec![0.35, -0.46513869166374205, 0.3464472651481628]
            },
            Coefs {
                fb: vec![1.0, -1.3877276182174683, 0.9699763059616089],
                ff: vec![0.35, 0.29919922947883604, 0.04006841853260994]
            },
        ];

        let bell_coefs: Vec<Coefs> = vec![
            Coefs {
                fb: vec![1.00, -1.8633348941802979, 0.8801209330558777],
                ff: vec![0.09732796562328783, -0.10529189079474921, 0.026142444150071956]
            },
            Coefs {
                fb: vec![1.00, -1.821143627166748, 0.9694930911064148],
                ff: vec![0.9176731674323697, -1.6463434709716742, 0.9176284335265728]
            },
            Coefs {
                fb: vec![1.00, -1.8136717081069946, 0.9057400822639465],
                ff: vec![0.260635334442507, -0.18719826837461115, 0.003026156143995027]
            },
            Coefs {
                fb: vec![1.00, -1.9527064561843872, 0.9532656073570251],
                ff: vec![0.267510268594725, -0.3677071379112296, 0.10019812325780178]
            },
        ];

       /* bell {
            fb: [1, -1.8633348941802979, 0.8801209330558777],
            ff: [0.09732796562328783, -0.10529189079474921, 0.026142444150071956]
        },
        {
            fb: [1, -1.821143627166748, 0.9694930911064148],
            ff: [0.9176731674323697, -1.6463434709716742, 0.9176284335265728]
        },
        {
            fb: [1, -1.8136717081069946, 0.9057400822639465],
            ff: [0.260635334442507, -0.18719826837461115, 0.003026156143995027]
        },
        {
            fb: [1, -1.9527064561843872, 0.9532656073570251],
            ff: [0.267510268594725, -0.3677071379112296, 0.10019812325780178]
        }*/
        //read this https://www.reddit.com/r/rust/comments/61x2yd/idiomatic_way_to_handle_modifying_vectors_in_a/

        for j in bell_coefs.iter() {
            for i in 0..length {
                output[i as usize] = j.ff[0] * channel_data[i as usize] + d[0];
                d[0] = j.ff[1] * channel_data[i as usize] - j.fb[1] * output[i as usize] + d[1];
                d[1] = j.ff[2] * channel_data[i as usize] - j.fb[2] * output[i as usize];
                channel_data[i as usize] = output[i as usize];
//                output[i as usize]  = output[i as usize] * 1.5;
            }
            d[0] = 0.0;
            d[1] = 0.0;
        }

        let filtered_buffer = self.ctx.create_buffer(audio_buffer.number_of_channels(), audio_buffer.length(), audio_buffer.sample_rate()).unwrap();
        filtered_buffer.copy_to_channel(&mut output, 0);
        filtered_buffer
//        audio_buffer.copy_to_channel(&mut output, 0);
//        audio_buffer
//        output_buff //no luck
//        output
        /*   console::log_1(&output_buff.get_channel_data(0).unwrap()[0].into());
        let b_source = self.get().create_buffer_source().unwrap();
        b_source.set_buffer(Some(&audio_buffer));
        let destination = self.get().destination();
        b_source.connect_with_audio_node(&destination).unwrap();
        b_source*/
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
