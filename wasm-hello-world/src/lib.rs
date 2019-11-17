extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use web_sys::{AudioContext, XmlHttpRequest, console, EventListener};

#[wasm_bindgen]
//abiltiy to import JS native API ** has to match with actual JS WEB MDN API
extern "C" {
    fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;


    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);


    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
//native rust environment
pub fn run_alert(item: &str) {
    alert(&format!("this is WASm and {}", item));
}

#[wasm_bindgen]
pub fn create_stuff() {
    let div = document.createElement("div");
    let p = document.createElement("p");

    p.set_inner("hellow");
    div.append(p);
    document.body().append(div);
}

//good resource : https://github.com/rustwasm/wasm-bindgen/issues/939
#[wasm_bindgen]
pub fn instantiate_context() -> Result<web_sys::AudioContext, wasm_bindgen::JsValue> {
    let ctx = AudioContext::new();
    let request = XmlHttpRequest::new().unwrap();
    request.open("GET", "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288");
//    let response_type = from_js_value(ArrayBuffer);
    request.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
    let results = request.send();
     match results{
             Err(e) => println!("nothing {:?}", e),
             Ok(res) => {
                 println!("{:?}", res);
                 console::log_1(&"Hello using web-sys".into());
             }
     }
  /*  match request.onload() {
        None => println!("nothing..."),
        Some(f) => {
            println!("?? {:?}", f);

        }
    }*/
//    request.send();
//    request::onload()
    ctx
}