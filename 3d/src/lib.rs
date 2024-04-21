use three_d::FrameInputGenerator;
use three_d::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console;

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

// CHINESE_GARDEN_4K
#[link_section = "chinese_garden_4k"]
pub static CHINESE_GARDEN_4K: [u8; get_chinese_garden_4k().len()] = get_chinese_garden_4k();

const fn get_chinese_garden_4k() -> [u8; include_bytes!("../assets/chinese_garden_4k.hdr").len()] {
    *include_bytes!("../assets/chinese_garden_4k.hdr")
}

// DamagedHelmet
#[link_section = "damaged_helmet"]
pub static DAMAGED_HELMET: [u8; get_damaged_helmet().len()] = get_damaged_helmet();

const fn get_damaged_helmet() -> [u8; include_bytes!("../assets/DamagedHelmet.glb").len()] {
    *include_bytes!("../assets/DamagedHelmet.glb")
}

fn threed_canvas() -> Result<(), JsValue> {
    let event_loop = winit::event_loop::EventLoop::new();

    let window_builder = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        winit::window::WindowBuilder::new()
            .with_canvas(Some(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("canvas")
                    .item(0)
                    .unwrap()
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .unwrap(),
            ))
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .with_prevent_default(true)
    };
    let window = window_builder.build(&event_loop).unwrap();
    let context = WindowedContext::from_winit_window(&window, SurfaceSettings::default()).unwrap();

    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let mut gui = three_d::GUI::new(&context);

    let mut raw_assets = three_d_asset::io::RawAssets::new();

    raw_assets.insert("CHINESE_GARDEN_4K".to_string(), CHINESE_GARDEN_4K.to_vec());
    raw_assets.insert("DAMAGED_HELMET.glb".to_string(), DAMAGED_HELMET.to_vec());

    let environment_map = raw_assets.deserialize("CHINESE_GARDEN_4K").unwrap();
    let skybox = Skybox::new_from_equirectangular(&context, &environment_map);

    let cpu_model_result: Result<CpuModel, _> = raw_assets.deserialize("DAMAGED_HELME");
    let mut cpu_model = match cpu_model_result {
        Ok(model) => model,
        Err(e) => {
            console::log_2(&"Failed to load thy model".into(), &e.to_string().into());
            return Err(JsValue::from_str("Failed to load model"));
        }
    };

    cpu_model
        .geometries
        .iter_mut()
        .for_each(|m| m.compute_tangents());
    let model = Model::<PhysicalMaterial>::new(&context, &cpu_model)
        .unwrap()
        .remove(0);

    let light = AmbientLight::new_with_environment(&context, 1.0, Srgba::WHITE, skybox.texture());

    // main loop
    let mut normal_map_enabled = true;
    let mut occlusion_map_enabled = true;
    let mut metallic_roughness_enabled = true;
    let mut albedo_map_enabled = true;
    let mut emissive_map_enabled = true;

    let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            let mut frame_input = frame_input_generator.generate(&context);

            let mut panel_width = 0.0;
            gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {},
            );

            let viewport = Viewport {
                x: (panel_width * frame_input.device_pixel_ratio) as i32,
                y: 0,
                width: frame_input.viewport.width
                    - (panel_width * frame_input.device_pixel_ratio) as u32,
                height: frame_input.viewport.height,
            };
            camera.set_viewport(viewport);
            control.handle_events(&mut camera, &mut frame_input.events);
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
                .render(&camera, &skybox, &[])
                .write(|| {
                    let material = PhysicalMaterial {
                        name: model.material.name.clone(),
                        albedo: model.material.albedo,
                        albedo_texture: if albedo_map_enabled {
                            model.material.albedo_texture.clone()
                        } else {
                            None
                        },
                        metallic: model.material.metallic,
                        roughness: model.material.roughness,
                        metallic_roughness_texture: if metallic_roughness_enabled {
                            model.material.metallic_roughness_texture.clone()
                        } else {
                            None
                        },
                        normal_scale: model.material.normal_scale,
                        normal_texture: if normal_map_enabled {
                            model.material.normal_texture.clone()
                        } else {
                            None
                        },
                        occlusion_strength: model.material.occlusion_strength,
                        occlusion_texture: if occlusion_map_enabled {
                            model.material.occlusion_texture.clone()
                        } else {
                            None
                        },
                        emissive: if emissive_map_enabled {
                            model.material.emissive
                        } else {
                            Srgba::BLACK
                        },
                        emissive_texture: if emissive_map_enabled {
                            model.material.emissive_texture.clone()
                        } else {
                            None
                        },
                        render_states: model.material.render_states,
                        is_transparent: model.material.is_transparent,
                        lighting_model: LightingModel::Cook(
                            NormalDistributionFunction::TrowbridgeReitzGGX,
                            GeometryFunction::SmithSchlickGGX,
                        ),
                    };
                    model.render_with_material(&material, &camera, &[&light]);
                    gui.render()
                })
                .unwrap();

            FrameOutput::default();
        }
        winit::event::Event::WindowEvent { ref event, .. } => {
            frame_input_generator.handle_winit_window_event(event);
            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    context.resize(*physical_size);
                }
                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    context.resize(**new_inner_size);
                }
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => (),
            }
        }
        _ => {}
    });
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

    let div = document.create_element("div")?;
    let canvas = document.create_element("canvas")?;

    div.append_child(&canvas)?;
    body.append_child(&div)?;

    threed_canvas()?;

    Ok(())
}
