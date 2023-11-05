use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn draw() {
    let canvas = get_2d_context_with_name("canvas");
    draw_with_context(&canvas);
}

fn get_2d_context_with_name(name: &str) -> CanvasRenderingContext2d {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(name)
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap()
}

fn draw_with_context(context: &CanvasRenderingContext2d) {
    context.move_to(300.0, 0.0);
    context.begin_path();
    context.line_to(0.0, 600.0);
    context.line_to(600.0, 600.0);
    context.line_to(300.0, 0.0);
    context.set_fill_style(&JsValue::from_str("blue"));
    context.close_path();
    context.stroke();
    context.fill();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
