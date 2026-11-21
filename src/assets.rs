// 原版素材（植物立绘）：由 index.html 预加载后挂到 window.__pvzAssets
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Art {
    Sunflower = 0,
    Peashooter = 1,
    Repeater = 2,
    Wallnut = 3,
    Cherry = 4,
    SnowPea = 5,
}

pub const ALL: [Art; 6] = [
    Art::Sunflower,
    Art::Peashooter,
    Art::Repeater,
    Art::Wallnut,
    Art::Cherry,
    Art::SnowPea,
];

impl Art {
    pub fn key(self) -> &'static str {
        match self {
            Art::Sunflower => "sunflower",
            Art::Peashooter => "peashooter",
            Art::Repeater => "repeater",
            Art::Wallnut => "wallnut",
            Art::Cherry => "cherrybomb",
            Art::SnowPea => "snowpea",
        }
    }
}

thread_local! {
    static IMGS: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
}

pub fn init() {
    let mut imgs: Vec<Option<HtmlImageElement>> = vec![None; ALL.len()];
    if let Some(window) = web_sys::window() {
        if let Ok(obj) = js_sys::Reflect::get(&window, &"__pvzAssets".into()) {
            if !obj.is_undefined() && !obj.is_null() {
                for (i, art) in ALL.iter().enumerate() {
                    if let Ok(v) = js_sys::Reflect::get(&obj, &art.key().into()) {
                        if let Ok(img) = v.dyn_into::<HtmlImageElement>() {
                            if img.width() > 0 && img.height() > 0 {
                                imgs[i] = Some(img);
                            }
                        }
                    }
                }
            }
        }
    }
    IMGS.with(|m| *m.borrow_mut() = imgs);
}

pub fn ready() -> bool {
    IMGS.with(|m| m.borrow().iter().any(|i| i.is_some()))
}

pub fn image(art: Art) -> Option<HtmlImageElement> {
    IMGS.with(|m| m.borrow().get(art as usize).cloned().flatten())
}
