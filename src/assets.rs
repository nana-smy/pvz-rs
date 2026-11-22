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

const Z_KEYS: [&str; 56] = [
    "z_normal_0",
    "z_normal_1",
    "z_normal_2",
    "z_normal_3",
    "z_normal_4",
    "z_normal_5",
    "z_normal_6",
    "z_normal_7",
    "z_cone_0",
    "z_cone_1",
    "z_cone_2",
    "z_cone_3",
    "z_cone_4",
    "z_cone_5",
    "z_cone_6",
    "z_cone_7",
    "z_bucket_0",
    "z_bucket_1",
    "z_bucket_2",
    "z_bucket_3",
    "z_bucket_4",
    "z_bucket_5",
    "z_bucket_6",
    "z_bucket_7",
    "z_flag_0",
    "z_flag_1",
    "z_flag_2",
    "z_flag_3",
    "z_flag_4",
    "z_flag_5",
    "z_flag_6",
    "z_flag_7",
    "z_polevault_0",
    "z_polevault_1",
    "z_polevault_2",
    "z_polevault_3",
    "z_polevault_4",
    "z_polevault_5",
    "z_polevault_6",
    "z_polevault_7",
    "z_screendoor_0",
    "z_screendoor_1",
    "z_screendoor_2",
    "z_screendoor_3",
    "z_screendoor_4",
    "z_screendoor_5",
    "z_screendoor_6",
    "z_screendoor_7",
    "z_football_0",
    "z_football_1",
    "z_football_2",
    "z_football_3",
    "z_football_4",
    "z_football_5",
    "z_football_6",
    "z_football_7",
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

fn live(key: &str) -> Option<HtmlImageElement> {
    let window = web_sys::window()?;
    let obj = js_sys::Reflect::get(&window, &"__pvzAssets".into()).ok()?;
    if obj.is_undefined() || obj.is_null() {
        return None;
    }
    get_img(&obj, key)
}

pub fn image(art: Art) -> Option<HtmlImageElement> {
    let cached = IMGS.with(|m| m.borrow().get(art as usize).cloned().flatten());
    if cached.is_some() {
        return cached;
    }
    let got = live(art.key());
    if let Some(img) = &got {
        IMGS.with(|m| {
            if let Some(slot) = m.borrow_mut().get_mut(art as usize) {
                *slot = Some(img.clone());
            }
        });
    }
    got
}

pub fn zombie(kind: u8, frame: u8) -> Option<HtmlImageElement> {
    let idx = kind as usize * 8 + (frame as usize & 7);
    if idx >= Z_KEYS.len() {
        return None;
    }
    let cached = ZOMBIES.with(|m| m.borrow().get(idx).cloned().flatten());
    if cached.is_some() {
        return cached;
    }
    let got = live(Z_KEYS[idx]);
    if let Some(img) = &got {
        ZOMBIES.with(|m| {
            if let Some(slot) = m.borrow_mut().get_mut(idx) {
                *slot = Some(img.clone());
            }
        });
    }
    got
}

pub fn bg(level: usize) -> Option<HtmlImageElement> {
    let idx = level.min(2);
    let cached = BGS.with(|m| m.borrow().get(idx).cloned().flatten());
    if cached.is_some() {
        return cached;
    }
    let got = live(BG_KEYS[idx]);
    if let Some(img) = &got {
        BGS.with(|m| {
            if let Some(slot) = m.borrow_mut().get_mut(idx) {
                *slot = Some(img.clone());
            }
        });
    }
    got
}
