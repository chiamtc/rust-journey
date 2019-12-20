extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;
extern crate rustfft;
extern crate sonogram;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBufferSourceNode, AudioBuffer, OfflineAudioContext, AudioContext, Request, RequestInit, RequestMode, Response, console, window};

//good book https://zsiciarz.github.io/24daysofrust/book/vol2/day2.html
//rustfft https://docs.rs/rustfft/3.0.0/rustfft/
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::convert::WasmAbi;
use wasm_bindgen::describe::WasmDescribe;

mod frequency;

use frequency::Frequency;

use std::sync::Arc;

use rustfft::FFTplanner;
use rustfft::FFT;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::{Complex, Complex64};
use rustfft::num_traits::Zero;

use sonogram::{Spectrograph, SpecOptionsBuilder};
/*
#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = fft)]
    fn get_fft(fft:Radix4<f32>) -> Radix4<f32>;
}
*/

#[derive(Debug)]
struct Coefs {
    fb: Vec<f32>,
    ff: Vec<f32>,
}

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

    #[wasm_bindgen]
    pub fn prep_buffer_and_rendering(&self, audio_buffer: AudioBuffer) -> Result<js_sys::Promise, JsValue> {
        let b_source = self.ctx.create_buffer_source().unwrap();
        b_source.set_buffer(Some(&audio_buffer));
        let destination = self.ctx.destination();
        b_source.connect_with_audio_node(&destination).unwrap();
        b_source.start();
        Ok(self.ctx.start_rendering()?)
    }
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
    pub fn prep_buffer_source(&self, audio_buffer: AudioBuffer) -> AudioBufferSourceNode {//-> Result<js_sys::Promise, JsValue> {
        let b_source = self.ctx.create_buffer_source().unwrap();
        b_source.set_buffer(Some(&audio_buffer));
        let destination = self.get().destination();
        b_source.connect_with_audio_node(&destination).unwrap();
        b_source
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
                ff: vec![0.35, -0.39466336369514465, 0.4124599575996399],
            },
            Coefs {
                fb: vec![1.0, -1.38728928565979, 0.8583449721336365],
                ff: vec![0.35, -0.46513869166374205, 0.3464472651481628],
            },
            Coefs {
                fb: vec![1.0, -1.3877276182174683, 0.9699763059616089],
                ff: vec![0.35, 0.29919922947883604, 0.04006841853260994],
            },
        ];

        let bell_coefs: Vec<Coefs> = vec![
            Coefs {
                fb: vec![1.00, -1.8633348941802979, 0.8801209330558777],
                ff: vec![0.09732796562328783, -0.10529189079474921, 0.026142444150071956],
            },
            Coefs {
                fb: vec![1.00, -1.821143627166748, 0.9694930911064148],
                ff: vec![0.9176731674323697, -1.6463434709716742, 0.9176284335265728],
            },
            Coefs {
                fb: vec![1.00, -1.8136717081069946, 0.9057400822639465],
                ff: vec![0.260635334442507, -0.18719826837461115, 0.003026156143995027],
            },
            Coefs {
                fb: vec![1.00, -1.9527064561843872, 0.9532656073570251],
                ff: vec![0.267510268594725, -0.3677071379112296, 0.10019812325780178],
            },
        ];
        //read this https://www.reddit.com/r/rust/comments/61x2yd/idiomatic_way_to_handle_modifying_vectors_in_a/

        for j in bell_coefs.iter() {
            for i in 0..length {
                output[i as usize] = j.ff[0] * channel_data[i as usize] + d[0];
                d[0] = j.ff[1] * channel_data[i as usize] - j.fb[1] * output[i as usize] + d[1];
                d[1] = j.ff[2] * channel_data[i as usize] - j.fb[2] * output[i as usize];
                channel_data[i as usize] = output[i as usize];
            }
            d[0] = 0.0;
            d[1] = 0.0;
        }

        let filtered_buffer = self.ctx.create_buffer(audio_buffer.number_of_channels(), audio_buffer.length(), audio_buffer.sample_rate()).unwrap();
        filtered_buffer.copy_to_channel(&mut output, 0);
        let c1_data = filtered_buffer.get_channel_data(0).unwrap();

        /*
         let mut input: Vec<Complex<f32>> = vec![Complex::zero(); 4096];
         let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 4096];

         let fft = Radix4::new(4096, false);
         fft.process(&mut input, &mut output);*/

        /*  let vert_array = js_sys::Float32Array::new(&JsValue::from(2));
            let out:Vec<f32> = output.iter().map(|c|  {
                c.norm();
            }).collect();
    //        let memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap();
            JsValue::from(vert_array);*/
//        console::log_1(&c1_data[0].into());
        filtered_buffer
    }

    //0.10394616425037384  = spectrumArray[0][0]
    #[wasm_bindgen]
    pub fn attempt_fft(&self, samples: &[f32]) -> JsValue {
        //        let mut input: Vec<Complex<f32>> = vec![Complex::zero(); 4096];
        //        let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 4096];
        //        let s = samples.to_vec().iter().take(2048).map(|d| Complex::new(f32::from(*d), 0.0)).collect();



        let signal_len = samples.len();
        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(signal_len);

        let width = 600;
        let height = 200;
        let window_size = signal_len as f32 / width as f32;
        let pixel_size = window_size as f32 / (4. * height as f32);

        let mut signal: Vec<Complex<f32>> = samples
            .iter()
            .map(|x| Complex::new(*x, 0f32))
//             .frequency(pixel_size)
            .collect::<Vec<_>>();

        let mut spectrum: Vec<Complex<f32>> = signal.clone();

        //let bin = 44100f32 / num_samples as f32;

        fft.process(&mut signal, &mut spectrum);

        let a: Vec<f32> = spectrum
            .iter()
            .map(|x: &Complex<f32>| x.norm() / (signal_len) as f32)// * bin as f32)
            //.take(num_samples / 2)
            .collect();
        let num_chunks = get_number_of_chunks(2048, 2048) as f32;

        JsValue::from_serde(&a).unwrap()
        //        JsValue::from("some value")
    }

    fn get_number_of_chunks(&mut self, chunk_len: usize, step: usize) -> usize {
        let mut i = 0;
        let mut chunks = 0;
        while i + chunk_len <= self.data.len() {
            i += step;
            chunks += 1;
        }
        if i == self.data.len() {
            chunks -= 1;
        }
        chunks
    }


    /*  #[wasm_bindgen]
      pub fn attempt_fft(&self, samples: Vec<i16>) -> JsValue {
          // Build the model
          let mut spectrograph = SpecOptionsBuilder::new(600, 200)
  //            .set_window_fn(sonogram::hann_function)
              .load_data_from_memory(samples, 48000)
              .build();
          // Compute the spectrogram giving the number of bins and the window overlap.
          spectrograph.compute(2048, 0.5);
          let a:Vec<f32> = spectrograph.create_in_memory(false);
  //        JsValue::from("test")
          JsValue::from_serde(&a).unwrap()
      }*/
}

#[wasm_bindgen(js_name = "runner")]
pub async fn run() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-LvrfS3FUwxCIH8_3uT3.wav?alt=media&token=24d4a22a-793f-4d10-b2cb-3345e188fb6b",
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


//good source : https://github.com/yurydelendik/thunderplains-2018-demos/tree/master/spectrum-final
//spectrum : https://github.com/klangner/dsp/blob/master/examples/spectrum.rs

//https://stackoverflow.com/questions/54726349/how-can-i-do-fft-analysis-of-audio-file-in-chrome-without-need-for-playback

//dsp in rust https://github.com/klangner/dsp/blob/master/examples/spectrum.rs

//another one in js - https://stackoverflow.com/questions/18718337/fft-with-offlineaudiocontext
//another one in js - https://joshondesign.com/p/books/canvasdeepdive/chapter12.html

//window function research https://stackoverflow.com/questions/32637841/analysernode-windowing
