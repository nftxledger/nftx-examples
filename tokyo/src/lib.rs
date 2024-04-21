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

// Video
pub static VIDEO: [u8; get_video().len()] = get_video();

const fn get_video() -> [u8; include_bytes!("video.mp4").len()] {
    *include_bytes!("video.mp4")
}

// return a DIV element with "Hello World" text
fn video_div() -> Result<web_sys::Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div = document.create_element("div")?;

    let video = document.create_element("video")?;
    video.set_attribute("width", "100%")?;
    video.set_attribute("height", "100%")?;
    video.set_attribute("controls", "")?;
    video.set_attribute("loop", "true")?;
    video.set_attribute("muted", "true")?;
    video.set_attribute(
        "src",
        &format!("data:video/mp4;base64,{}", base64::encode(&VIDEO)),
    )?;

    div.append_child(&video)?;

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

    let div = video_div()?;
    body.append_child(&div)?;
    Ok(())
}
