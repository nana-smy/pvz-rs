// 原版素材：植物立绘、僵尸整身拼装图、关卡背景，由 index.html 预加载后挂到 window.__pvzAssets
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlImageElement;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Art {
    Sunflower = 0,
    Peashooter = 1,
    Repeater = 2,
    Wallnut = 3,
    Cherry = 4,
    SnowPea = 5,
    PotatoMine = 6,
    Jalapeno = 7,
    Threepeater = 8,
    Squash = 9,
}

pub const ALL: [Art; 10] = [
    Art::Sunflower,
    Art::Peashooter,
    Art::Repeater,
    Art::Wallnut,
    Art::Cherry,
    Art::SnowPea,
    Art::PotatoMine,
    Art::Jalapeno,
    Art::Threepeater,
    Art::Squash,
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
            Art::PotatoMine => "potatomine",
            Art::Jalapeno => "jalapeno",
            Art::Threepeater => "threepeater",
            Art::Squash => "squash",
        }
    }
}

thread_local! {
    static IMGS: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static ZOMBIES: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static BGS: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
}

const Z_KEYS: [&str; 7] = [
    "z_normal",
    "z_cone",
    "z_bucket",
    "z_flag",
    "z_polevault",
    "z_screendoor",
    "z_football",
];
const BG_KEYS: [&str; 3] = ["bg_day", "bg_dusk", "bg_night"];

fn get_img(obj: &JsValue, key: &str) -> Option<HtmlImageElement> {
    if let Ok(v) = js_sys::Reflect::get(obj, &key.into()) {
        if let Ok(img) = v.dyn_into::<HtmlImageElement>() {
            if img.width() > 0 && img.height() > 0 {
                return Some(img);
            }
        }
    }
    None
}

pub fn init() {
    let mut imgs: Vec<Option<HtmlImageElement>> = vec![None; ALL.len()];
    let mut zs: Vec<Option<HtmlImageElement>> = vec![None; Z_KEYS.len()];
    let mut bgs: Vec<Option<HtmlImageElement>> = vec![None; BG_KEYS.len()];
    if let Some(window) = web_sys::window() {
        if let Ok(obj) = js_sys::Reflect::get(&window, &"__pvzAssets".into()) {
            if !obj.is_undefined() && !obj.is_null() {
                for (i, art) in ALL.iter().enumerate() {
                    imgs[i] = get_img(&obj, art.key());
                }
                for (i, k) in Z_KEYS.iter().enumerate() {
                    zs[i] = get_img(&obj, k);
                }
                for (i, k) in BG_KEYS.iter().enumerate() {
                    bgs[i] = get_img(&obj, k);
                }
            }
        }
    }
    IMGS.with(|m| *m.borrow_mut() = imgs);
    ZOMBIES.with(|m| *m.borrow_mut() = zs);
    BGS.with(|m| *m.borrow_mut() = bgs);
}

pub fn ready() -> bool {
    IMGS.with(|m| m.borrow().iter().any(|i| i.is_some()))
}

pub fn image(art: Art) -> Option<HtmlImageElement> {
    IMGS.with(|m| m.borrow().get(art as usize).cloned().flatten())
}

pub fn zombie(kind: u8) -> Option<HtmlImageElement> {
    ZOMBIES.with(|m| m.borrow().get(kind as usize).cloned().flatten())
}

pub fn bg(level: usize) -> Option<HtmlImageElement> {
    BGS.with(|m| m.borrow().get(level.min(2)).cloned().flatten())
}
