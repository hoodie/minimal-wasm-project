use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, EventTarget, KeyboardEvent};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(msg: &str);
}

#[used]
static MAGIC: u32 = 0x42424242;

#[wasm_bindgen]
pub fn greet(name: &str) {
    debug(&format!("Hello {}, you're awsm!", name));
}

fn get_document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    document
}

#[wasm_bindgen]
pub fn draw() -> Result<(), JsValue> {
    debug("start drawing");

    let document = get_document();
    let body = document.body().expect("document should have a body");

    let new_canvas = document
        .create_element("canvas")
        .expect(r"unable to create canvas from WASM /0\");

    AsRef::<web_sys::Node>::as_ref(&body).append_child(new_canvas.as_ref())?;

    Ok(())
}

#[wasm_bindgen]
pub fn call_me_maybe(cb: &js_sys::Function) -> Result<(), JsValue> {
    let message = "I ❤️ you";

    cb.call1(&JsValue::NULL, &JsValue::from(message))?;

    Ok(())
}

#[wasm_bindgen]
pub fn listen_for_keys() -> Result<(), JsValue> {
    let document = get_document();

    let cb = Closure::wrap(Box::new(move |v: KeyboardEvent| {
        debug(&format!("down wityh all the keys: {:#?}", v.key()))
    }) as Box<dyn Fn(_)>);

    let et: &EventTarget = document.as_ref();
    et.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())?;
    cb.forget();

    Ok(())
}
