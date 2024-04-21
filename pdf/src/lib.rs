use base64::{engine::general_purpose, Engine as _};
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

// PDF Source
#[link_section = "pdf"]
pub static PDF: [u8; get_pdf().len()] = get_pdf();

// get the pdf file from bitcoin.pdf in the same directory
const fn get_pdf() -> [u8; include_bytes!("bitcoin.pdf").len()] {
    *include_bytes!("bitcoin.pdf")
}

// return a DIV element with "Hello World" text
fn pdf_div() -> Result<web_sys::Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div = document.create_element("div")?;

    let object = document.create_element("object")?;

    let pdf_base64 = general_purpose::STANDARD.encode(PDF);
    let pdf = format!("data:application/pdf;base64,{}", pdf_base64);

    object.set_attribute("data", &pdf)?;
    object.set_attribute("type", "application/pdf")?;
    object.set_attribute("width", "100%")?;
    object.set_attribute("height", "100%")?;

    div.append_child(&object)?;

    Ok(div)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let div = pdf_div()?;
    body.append_child(&div)?;
    Ok(())
}
