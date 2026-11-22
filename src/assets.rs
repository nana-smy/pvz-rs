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
    Cabbage = 10,
    Starfruit = 11,
    Garlic = 12,
    Spikeweed = 13,
    LilyPad = 14,
    FlowerPot = 15,
    Chomper = 16,
    PuffShroom = 17,
    SunShroom = 18,
    FumeShroom = 19,
    GraveBuster = 20,
    HypnoShroom = 21,
    ScaredyShroom = 22,
    IceShroom = 23,
    DoomShroom = 24,
    TangleKelp = 25,
    Torchwood = 26,
    TallNut = 27,
    SeaShroom = 28,
    Plantern = 29,
    Cactus = 30,
    Blover = 31,
    SplitPea = 32,
    Pumpkin = 33,
    MagnetShroom = 34,
    KernelPult = 35,
    CoffeeBean = 36,
    UmbrellaLeaf = 37,
    Marigold = 38,
    MelonPult = 39,
    GatlingPea = 40,
    TwinSunflower = 41,
    GloomShroom = 42,
    Cattail = 43,
    WinterMelon = 44,
    GoldMagnet = 45,
    Spikerock = 46,
    CobCannon = 47,
    Imitater = 48,
}

pub const ALL: [Art; 49] = [
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
    Art::Cabbage,
    Art::Starfruit,
    Art::Garlic,
    Art::Spikeweed,
    Art::LilyPad,
    Art::FlowerPot,
    Art::Chomper,
    Art::PuffShroom,
    Art::SunShroom,
    Art::FumeShroom,
    Art::GraveBuster,
    Art::HypnoShroom,
    Art::ScaredyShroom,
    Art::IceShroom,
    Art::DoomShroom,
    Art::TangleKelp,
    Art::Torchwood,
    Art::TallNut,
    Art::SeaShroom,
    Art::Plantern,
    Art::Cactus,
    Art::Blover,
    Art::SplitPea,
    Art::Pumpkin,
    Art::MagnetShroom,
    Art::KernelPult,
    Art::CoffeeBean,
    Art::UmbrellaLeaf,
    Art::Marigold,
    Art::MelonPult,
    Art::GatlingPea,
    Art::TwinSunflower,
    Art::GloomShroom,
    Art::Cattail,
    Art::WinterMelon,
    Art::GoldMagnet,
    Art::Spikerock,
    Art::CobCannon,
    Art::Imitater,
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
            Art::Cabbage => "cabbagepult",
            Art::Starfruit => "starfruit",
            Art::Garlic => "garlic",
            Art::Spikeweed => "spikeweed",
            Art::LilyPad => "lilypad",
            Art::FlowerPot => "flowerpot",
            Art::Chomper => "chomper",
            Art::PuffShroom => "puffshroom",
            Art::SunShroom => "sunshroom",
            Art::FumeShroom => "fumeshroom",
            Art::GraveBuster => "gravebuster",
            Art::HypnoShroom => "hypnoshroom",
            Art::ScaredyShroom => "scaredyshroom",
            Art::IceShroom => "iceshroom",
            Art::DoomShroom => "doomshroom",
            Art::TangleKelp => "tanglekelp",
            Art::Torchwood => "torchwood",
            Art::TallNut => "tallnut",
            Art::SeaShroom => "seashroom",
            Art::Plantern => "plantern",
            Art::Cactus => "cactus",
            Art::Blover => "blover",
            Art::SplitPea => "splitpea",
            Art::Pumpkin => "pumpkin",
            Art::MagnetShroom => "magnetshroom",
            Art::KernelPult => "kernelpult",
            Art::CoffeeBean => "coffeebean",
            Art::UmbrellaLeaf => "umbrellaleaf",
            Art::Marigold => "marigold",
            Art::MelonPult => "melonpult",
            Art::GatlingPea => "gatlingpea",
            Art::TwinSunflower => "twinsunflower",
            Art::GloomShroom => "gloomshroom",
            Art::Cattail => "cattail",
            Art::WinterMelon => "wintermelon",
            Art::GoldMagnet => "goldmagnet",
            Art::Spikerock => "spikerock",
            Art::CobCannon => "cobcannon",
            Art::Imitater => "imitater",
        }
    }
}

thread_local! {
    static IMGS: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static ZOMBIES: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static BGS: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static PROJ: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static SHEETS: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
    static REANIM: RefCell<Vec<Option<HtmlImageElement>>> = const { RefCell::new(Vec::new()) };
}

/// 原版 reanim 图集的页面 key：`r_<role>/<clip>` 里的斜杠换成下划线。
pub fn reanim_key(slot: usize) -> String {
    format!("r_{}", crate::reanim::KEYS[slot].replace('/', "_"))
}

const Z_KEYS: [&str; 416] = [
    "normal_0",
    "normal_1",
    "normal_2",
    "normal_3",
    "normal_4",
    "normal_5",
    "normal_6",
    "normal_7",
    "normal_8",
    "normal_9",
    "normal_10",
    "normal_11",
    "normal_12",
    "normal_13",
    "normal_14",
    "normal_15",
    "cone_0",
    "cone_1",
    "cone_2",
    "cone_3",
    "cone_4",
    "cone_5",
    "cone_6",
    "cone_7",
    "cone_8",
    "cone_9",
    "cone_10",
    "cone_11",
    "cone_12",
    "cone_13",
    "cone_14",
    "cone_15",
    "bucket_0",
    "bucket_1",
    "bucket_2",
    "bucket_3",
    "bucket_4",
    "bucket_5",
    "bucket_6",
    "bucket_7",
    "bucket_8",
    "bucket_9",
    "bucket_10",
    "bucket_11",
    "bucket_12",
    "bucket_13",
    "bucket_14",
    "bucket_15",
    "flag_0",
    "flag_1",
    "flag_2",
    "flag_3",
    "flag_4",
    "flag_5",
    "flag_6",
    "flag_7",
    "flag_8",
    "flag_9",
    "flag_10",
    "flag_11",
    "flag_12",
    "flag_13",
    "flag_14",
    "flag_15",
    "polevault_0",
    "polevault_1",
    "polevault_2",
    "polevault_3",
    "polevault_4",
    "polevault_5",
    "polevault_6",
    "polevault_7",
    "polevault_8",
    "polevault_9",
    "polevault_10",
    "polevault_11",
    "polevault_12",
    "polevault_13",
    "polevault_14",
    "polevault_15",
    "screendoor_0",
    "screendoor_1",
    "screendoor_2",
    "screendoor_3",
    "screendoor_4",
    "screendoor_5",
    "screendoor_6",
    "screendoor_7",
    "screendoor_8",
    "screendoor_9",
    "screendoor_10",
    "screendoor_11",
    "screendoor_12",
    "screendoor_13",
    "screendoor_14",
    "screendoor_15",
    "football_0",
    "football_1",
    "football_2",
    "football_3",
    "football_4",
    "football_5",
    "football_6",
    "football_7",
    "football_8",
    "football_9",
    "football_10",
    "football_11",
    "football_12",
    "football_13",
    "football_14",
    "football_15",
    "ladder_0",
    "ladder_1",
    "ladder_2",
    "ladder_3",
    "ladder_4",
    "ladder_5",
    "ladder_6",
    "ladder_7",
    "ladder_8",
    "ladder_9",
    "ladder_10",
    "ladder_11",
    "ladder_12",
    "ladder_13",
    "ladder_14",
    "ladder_15",
    "newspaper_0",
    "newspaper_1",
    "newspaper_2",
    "newspaper_3",
    "newspaper_4",
    "newspaper_5",
    "newspaper_6",
    "newspaper_7",
    "newspaper_8",
    "newspaper_9",
    "newspaper_10",
    "newspaper_11",
    "newspaper_12",
    "newspaper_13",
    "newspaper_14",
    "newspaper_15",
    "dancing_0",
    "dancing_1",
    "dancing_2",
    "dancing_3",
    "dancing_4",
    "dancing_5",
    "dancing_6",
    "dancing_7",
    "dancing_8",
    "dancing_9",
    "dancing_10",
    "dancing_11",
    "dancing_12",
    "dancing_13",
    "dancing_14",
    "dancing_15",
    "backupdancer_0",
    "backupdancer_1",
    "backupdancer_2",
    "backupdancer_3",
    "backupdancer_4",
    "backupdancer_5",
    "backupdancer_6",
    "backupdancer_7",
    "backupdancer_8",
    "backupdancer_9",
    "backupdancer_10",
    "backupdancer_11",
    "backupdancer_12",
    "backupdancer_13",
    "backupdancer_14",
    "backupdancer_15",
    "duckytube_0",
    "duckytube_1",
    "duckytube_2",
    "duckytube_3",
    "duckytube_4",
    "duckytube_5",
    "duckytube_6",
    "duckytube_7",
    "duckytube_8",
    "duckytube_9",
    "duckytube_10",
    "duckytube_11",
    "duckytube_12",
    "duckytube_13",
    "duckytube_14",
    "duckytube_15",
    "snorkel_0",
    "snorkel_1",
    "snorkel_2",
    "snorkel_3",
    "snorkel_4",
    "snorkel_5",
    "snorkel_6",
    "snorkel_7",
    "snorkel_8",
    "snorkel_9",
    "snorkel_10",
    "snorkel_11",
    "snorkel_12",
    "snorkel_13",
    "snorkel_14",
    "snorkel_15",
    "zomboni_0",
    "zomboni_1",
    "zomboni_2",
    "zomboni_3",
    "zomboni_4",
    "zomboni_5",
    "zomboni_6",
    "zomboni_7",
    "zomboni_8",
    "zomboni_9",
    "zomboni_10",
    "zomboni_11",
    "zomboni_12",
    "zomboni_13",
    "zomboni_14",
    "zomboni_15",
    "bobsled_0",
    "bobsled_1",
    "bobsled_2",
    "bobsled_3",
    "bobsled_4",
    "bobsled_5",
    "bobsled_6",
    "bobsled_7",
    "bobsled_8",
    "bobsled_9",
    "bobsled_10",
    "bobsled_11",
    "bobsled_12",
    "bobsled_13",
    "bobsled_14",
    "bobsled_15",
    "dolphinrider_0",
    "dolphinrider_1",
    "dolphinrider_2",
    "dolphinrider_3",
    "dolphinrider_4",
    "dolphinrider_5",
    "dolphinrider_6",
    "dolphinrider_7",
    "dolphinrider_8",
    "dolphinrider_9",
    "dolphinrider_10",
    "dolphinrider_11",
    "dolphinrider_12",
    "dolphinrider_13",
    "dolphinrider_14",
    "dolphinrider_15",
    "jackbox_0",
    "jackbox_1",
    "jackbox_2",
    "jackbox_3",
    "jackbox_4",
    "jackbox_5",
    "jackbox_6",
    "jackbox_7",
    "jackbox_8",
    "jackbox_9",
    "jackbox_10",
    "jackbox_11",
    "jackbox_12",
    "jackbox_13",
    "jackbox_14",
    "jackbox_15",
    "balloon_0",
    "balloon_1",
    "balloon_2",
    "balloon_3",
    "balloon_4",
    "balloon_5",
    "balloon_6",
    "balloon_7",
    "balloon_8",
    "balloon_9",
    "balloon_10",
    "balloon_11",
    "balloon_12",
    "balloon_13",
    "balloon_14",
    "balloon_15",
    "digger_0",
    "digger_1",
    "digger_2",
    "digger_3",
    "digger_4",
    "digger_5",
    "digger_6",
    "digger_7",
    "digger_8",
    "digger_9",
    "digger_10",
    "digger_11",
    "digger_12",
    "digger_13",
    "digger_14",
    "digger_15",
    "pogo_0",
    "pogo_1",
    "pogo_2",
    "pogo_3",
    "pogo_4",
    "pogo_5",
    "pogo_6",
    "pogo_7",
    "pogo_8",
    "pogo_9",
    "pogo_10",
    "pogo_11",
    "pogo_12",
    "pogo_13",
    "pogo_14",
    "pogo_15",
    "yeti_0",
    "yeti_1",
    "yeti_2",
    "yeti_3",
    "yeti_4",
    "yeti_5",
    "yeti_6",
    "yeti_7",
    "yeti_8",
    "yeti_9",
    "yeti_10",
    "yeti_11",
    "yeti_12",
    "yeti_13",
    "yeti_14",
    "yeti_15",
    "bungee_0",
    "bungee_1",
    "bungee_2",
    "bungee_3",
    "bungee_4",
    "bungee_5",
    "bungee_6",
    "bungee_7",
    "bungee_8",
    "bungee_9",
    "bungee_10",
    "bungee_11",
    "bungee_12",
    "bungee_13",
    "bungee_14",
    "bungee_15",
    "catapult_0",
    "catapult_1",
    "catapult_2",
    "catapult_3",
    "catapult_4",
    "catapult_5",
    "catapult_6",
    "catapult_7",
    "catapult_8",
    "catapult_9",
    "catapult_10",
    "catapult_11",
    "catapult_12",
    "catapult_13",
    "catapult_14",
    "catapult_15",
    "gargantuar_0",
    "gargantuar_1",
    "gargantuar_2",
    "gargantuar_3",
    "gargantuar_4",
    "gargantuar_5",
    "gargantuar_6",
    "gargantuar_7",
    "gargantuar_8",
    "gargantuar_9",
    "gargantuar_10",
    "gargantuar_11",
    "gargantuar_12",
    "gargantuar_13",
    "gargantuar_14",
    "gargantuar_15",
    "imp_0",
    "imp_1",
    "imp_2",
    "imp_3",
    "imp_4",
    "imp_5",
    "imp_6",
    "imp_7",
    "imp_8",
    "imp_9",
    "imp_10",
    "imp_11",
    "imp_12",
    "imp_13",
    "imp_14",
    "imp_15",
    "boss_0",
    "boss_1",
    "boss_2",
    "boss_3",
    "boss_4",
    "boss_5",
    "boss_6",
    "boss_7",
    "boss_8",
    "boss_9",
    "boss_10",
    "boss_11",
    "boss_12",
    "boss_13",
    "boss_14",
    "boss_15"
];
const BG_KEYS: [&str; 6] = ["bg_day", "bg_dusk", "bg_night", "bg_pool", "bg_pool_night", "bg_roof"];
const PROJ_KEYS: [&str; 2] = ["pea", "pea_ice"];

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
    let mut ps: Vec<Option<HtmlImageElement>> = vec![None; PROJ_KEYS.len()];
    let mut rs: Vec<Option<HtmlImageElement>> = vec![None; crate::reanim::KEYS.len()];
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
                for (i, k) in PROJ_KEYS.iter().enumerate() {
                    ps[i] = get_img(&obj, k);
                }
                for i in 0..crate::reanim::KEYS.len() {
                    rs[i] = get_img(&obj, &reanim_key(i));
                }
            }
        }
    }
    IMGS.with(|m| *m.borrow_mut() = imgs);
    ZOMBIES.with(|m| *m.borrow_mut() = zs);
    BGS.with(|m| *m.borrow_mut() = bgs);
    PROJ.with(|m| *m.borrow_mut() = ps);
    REANIM.with(|m| *m.borrow_mut() = rs);
}

/// 取原版 reanim 图集：先看初始化时的缓存，未就绪则从 window.__pvzAssets 实时取。
pub fn reanim_img(slot: u16) -> Option<HtmlImageElement> {
    let i = slot as usize;
    if i >= crate::reanim::KEYS.len() {
        return None;
    }
    let cached = REANIM.with(|m| m.borrow().get(i).cloned().flatten());
    if cached.is_some() {
        return cached;
    }
    let img = live(&reanim_key(i))?;
    REANIM.with(|m| {
        let mut v = m.borrow_mut();
        if v.len() < crate::reanim::KEYS.len() {
            v.resize(crate::reanim::KEYS.len(), None);
        }
        v[i] = Some(img.clone());
    });
    Some(img)
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

pub fn plant_sheet(art: Art, shoot: bool) -> Option<HtmlImageElement> {
    let base = art as usize;
    let clip = if shoot {
        crate::plant_anim::SHOOT[base]
    } else {
        crate::plant_anim::IDLE[base]
    };
    if clip.key.is_empty() || clip.frames == 0 {
        return None;
    }
    let idx = base + if shoot { 16 } else { 0 };
    let cached = SHEETS.with(|m| m.borrow().get(idx).cloned().flatten());
    if cached.is_some() {
        return cached;
    }
    let got = live(clip.key);
    if let Some(img) = &got {
        SHEETS.with(|m| {
            let mut v = m.borrow_mut();
            if v.len() < 33 {
                v.resize(33, None);
            }
            v[idx] = Some(img.clone());
        });
    }
    got
}

pub fn plant_rise() -> Option<HtmlImageElement> {
    let idx = 32;
    let cached = SHEETS.with(|m| m.borrow().get(idx).cloned().flatten());
    if cached.is_some() { return cached; }
    let clip = crate::plant_anim::RISE_POTATO;
    let got = live(clip.key);
    if let Some(img) = &got {
        SHEETS.with(|m| { let mut v = m.borrow_mut(); if v.len() < 33 { v.resize(33, None); } v[idx] = Some(img.clone()); });
    }
    got
}

pub fn projectile(kind: u8) -> Option<HtmlImageElement> {    let idx = (kind as usize).min(1);
    let cached = PROJ.with(|m| m.borrow().get(idx).cloned().flatten());
    if cached.is_some() {
        return cached;
    }
    let got = live(PROJ_KEYS[idx]);
    if let Some(img) = &got {
        PROJ.with(|m| {
            if let Some(slot) = m.borrow_mut().get_mut(idx) {
                *slot = Some(img.clone());
            }
        });
    }
    got
}

pub fn zombie(kind: u8, frame: u8) -> Option<HtmlImageElement> {
    let idx = kind as usize * 16 + (frame as usize & 15);
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

pub fn water() -> Option<HtmlImageElement> {
    live("water")
}

pub fn fog() -> Option<HtmlImageElement> {
    live("fog")
}

pub fn bg(level: usize) -> Option<HtmlImageElement> {
    let idx = match (level / 10).min(4) {
        0 => 0, // 白天前院
        1 => 2, // 夜晚前院
        2 => 3, // 白天泳池
        3 => 4, // 夜晚泳池 · 浓雾
        _ => 5, // 屋顶
    };
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
