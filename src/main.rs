mod assets;
mod defs;
mod game;
mod plant_anim;
mod render;
mod sfx;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, PointerEvent};

fn canvas_coords(canvas: &HtmlCanvasElement, ev: &PointerEvent) -> (f32, f32) {
    let rect = canvas.get_bounding_client_rect();
    let x = (ev.client_x() as f64 - rect.left()) / rect.width().max(1.0) * defs::W as f64;
    let y = (ev.client_y() as f64 - rect.top()) / rect.height().max(1.0) * defs::H as f64;
    (x as f32, y as f32)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("no window")?;
    let doc = window.document().ok_or("no document")?;
    let canvas = doc
        .get_element_by_id("game")
        .ok_or("no canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_width(defs::W as u32);
    canvas.set_height(defs::H as u32);
    let ctx = canvas
        .get_context("2d")?
        .ok_or("no 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    assets::init();
    let game = Rc::new(RefCell::new(game::Game::new()));

    // 指针按下
    {
        let g = game.clone();
        let c = canvas.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |ev: PointerEvent| {
            ev.prevent_default();
            let (x, y) = canvas_coords(&c, &ev);
            let btn = ev.button();
            let mut gm = g.borrow_mut();
            gm.pointer_down(x, y, btn as i16);
            gm.sfx.resume();
        });
        canvas.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }
    // 指针移动
    {
        let g = game.clone();
        let c = canvas.clone();
        let cb = Closure::<dyn FnMut(PointerEvent)>::new(move |ev: PointerEvent| {
            let (x, y) = canvas_coords(&c, &ev);
            g.borrow_mut().pointer_move(x, y);
        });
        canvas.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }
    // 屏蔽右键菜单
    {
        let cb = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            ev.prevent_default();
        });
        canvas.add_event_listener_with_callback("contextmenu", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }
    // 键盘
    {
        let g = game.clone();
        let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |ev: KeyboardEvent| {
            let code = ev.code();
            if code == "Space" {
                ev.prevent_default();
            }
            g.borrow_mut().key(&code);
        });
        window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // 主循环
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let f2 = f.clone();
    let last = Rc::new(Cell::new(0.0f64));
    let last2 = last.clone();
    let g2 = game.clone();
    let ctx2 = ctx.clone();
    let win2 = window.clone();

    *f.borrow_mut() = Some(Closure::wrap(Box::new(move |ts: f64| {
        let dt = if last2.get() <= 0.0 {
            0.0
        } else {
            (ts - last2.get()) as f32 / 1000.0
        };
        last2.set(ts);
        let dt = dt.clamp(0.0, 0.05);
        {
            let mut gm = g2.borrow_mut();
            gm.update(dt);
            render::render(&gm, &ctx2);
        }
        if let Some(cb) = f2.borrow().as_ref() {
            let _ = win2.request_animation_frame(cb.as_ref().unchecked_ref());
        }
    }) as Box<dyn FnMut(f64)>));

    if let Some(cb) = f.borrow().as_ref() {
        window.request_animation_frame(cb.as_ref().unchecked_ref())?;
    }

    // 隐藏加载层
    if let Some(body) = doc.body() {
        let _ = body.class_list().add_1("ready");
    }
    Ok(())
}

fn main() {}
