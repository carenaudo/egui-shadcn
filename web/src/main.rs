//! Web demo entry point for egui-shadcn component dashboard.
//!
//! Includes the component_dashboard example and adds a wasm entry point.

include!("../../examples/component_dashboard.rs");

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| {
                    egui_shadcn::setup_fonts(&cc.egui_ctx);
                    Ok(Box::new(DashboardApp::default()))
                }),
            )
            .await;

        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => loading_text.remove(),
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p>The app has crashed. See the developer console.</p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
