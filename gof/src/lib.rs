use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
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

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                // fill with game of line 101 Achime Flammenkamp
                if i == 64 * 32 + 32
                    || i == 64 * 32 + 33
                    || i == 64 * 32 + 34
                    || i == 64 * 33 + 31
                    || i == 64 * 34 + 31
                    || i == 64 * 35 + 31
                    || i == 64 * 36 + 32
                    || i == 64 * 37 + 33
                    || i == 64 * 38 + 34
                    || i == 64 * 39 + 35
                    || i == 64 * 40 + 35
                    || i == 64 * 41 + 35
                    || i == 64 * 42 + 34
                    || i == 64 * 43 + 33
                    || i == 64 * 44 + 32
                {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn gof_canvas() -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(641);
    canvas.set_height(641);

    let mut canvas_context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let mut universe = Universe::new();
    universe.tick();

    draw_grid(&universe, &mut canvas_context)?;

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();

    let mut last_tick = 0.0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        // execute game of life tick every 1 second using requestAnimationFrame
        let tick_interval = 1000.0;
        let window = web_sys::window().expect("no global `window` exists");
        let performance = window
            .performance()
            .expect("performance should be available");
        let now = performance.now();
        let elapsed = now - last_tick;

        if elapsed < tick_interval {
            request_animation_frame(f.borrow().as_ref().unwrap());
            return;
        }

        last_tick = now;

        universe.tick();

        draw_cells(&universe, &mut canvas_context).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(canvas)
}

fn draw_grid(
    universe: &Universe,
    canvas_context: &mut web_sys::CanvasRenderingContext2d,
) -> Result<(), JsValue> {
    canvas_context.begin_path();
    canvas_context.set_stroke_style(&JsValue::from_str("#E5E5E5"));
    for i in 0..=universe.width {
        canvas_context.move_to((i as f64) * 10.0, 0.0);
        canvas_context.line_to((i as f64) * 10.0, (universe.height as f64) * 10.0);
    }
    for i in 0..=universe.height {
        canvas_context.move_to(0.0, (i as f64) * 10.0);
        canvas_context.line_to((universe.width as f64) * 10.0, (i as f64) * 10.0);
    }
    canvas_context.stroke();

    Ok(())
}

fn draw_cells(
    universe: &Universe,
    canvas_context: &mut web_sys::CanvasRenderingContext2d,
) -> Result<web_sys::HtmlCanvasElement, JsValue> {
    canvas_context.begin_path();

    for row in 0..universe.height {
        for col in 0..universe.width {
            let idx = universe.get_index(row, col);
            canvas_context.set_fill_style(&JsValue::from_str(
                if universe.cells[idx] == Cell::Dead {
                    "#fff"
                } else {
                    "#000"
                },
            ));
            canvas_context.fill_rect(
                (col as f64) * 10.0 + 1.0,
                (row as f64) * 10.0 + 1.0,
                9.0,
                9.0,
            );
        }
    }

    canvas_context.stroke();

    Ok(web_sys::HtmlCanvasElement::from(
        canvas_context.canvas().unwrap(),
    ))
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    body.set_attribute(
        "style",
        "display: flex; align-items: center; justify-content: center; background-color: white; font-family: monospace",
    )?;

    let canvas = gof_canvas()?;

    body.append_child(&canvas)?;

    Ok(())
}
