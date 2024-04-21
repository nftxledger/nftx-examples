use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// Artifact Metadata
#[link_section = "metadata"]
pub static METADATA: [u8; get_metadata().len()] = get_metadata();

const fn get_metadata() -> [u8; include_bytes!("metadata.json").len()] {
    *include_bytes!("metadata.json")
}

// Artifact Thumbnail
#[link_section = "thumbnail"]
pub static THUMBNAIL: [u8; get_thumbnail().len()] = get_thumbnail();

const fn get_thumbnail() -> [u8; include_bytes!("thumbnail.png").len()] {
    *include_bytes!("thumbnail.png")
}

// Image
pub static IMAGE: [u8; get_image().len()] = get_image();

const fn get_image() -> [u8; include_bytes!("image.png").len()] {
    *include_bytes!("image.png")
}

// return a DIV element with "Hello World" text
fn image_div() -> Result<web_sys::Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div = document.create_element("div")?;

    let img = document.create_element("img")?;
    img.set_attribute(
        "src",
        &format!("data:image/png;base64,{}", base64::encode(&IMAGE)),
    )?;
    img.set_attribute("width", "100%")?;
    img.set_attribute("height", "100%")?;
    div.append_child(&img)?;

    Ok(div)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.set_attribute(
        "style",
        "display: flex; align-items: center; justify-content: center;",
    )?;

    let div = image_div()?;
    body.append_child(&div)?;
    Ok(())
}
