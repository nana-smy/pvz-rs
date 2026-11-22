// 全局常量与数值定义
pub const W: f32 = 1280.0;
pub const H: f32 = 720.0;

pub const ROWS: usize = 5;
pub const COLS: usize = 9;

pub const GRID_X: f32 = 246.3;
pub const GRID_Y: f32 = 163.5;
pub const CELL_W: f32 = 86.4;
pub const CELL_H: f32 = 106.5;

pub const HUD_H: f32 = 104.0;
pub const LAWN_BOTTOM: f32 = 696.0;

pub const TOTAL_WAVES: u32 = 10;
pub const FIRST_WAVE_AT: f32 = 25.0;
pub const WAVE_GAPS: [f32; 9] = [24.0, 23.0, 22.0, 21.0, 21.0, 20.0, 20.0, 19.0, 19.0];

pub const MOWER_X: f32 = 214.0;
pub const DEATH_X: f32 = 188.0;

// 卡片 HUD 布局
pub const CARD_X0: f32 = 142.0;
pub const CARD_Y: f32 = 16.0;
pub const CARD_W: f32 = 68.0;
pub const CARD_H: f32 = 72.0;
pub const CARD_GAP: f32 = 4.0;

pub fn card_rect(i: usize) -> (f32, f32, f32, f32) {
    (
        CARD_X0 + i as f32 * (CARD_W + CARD_GAP),
        CARD_Y,
        CARD_W,
        CARD_H,
    )
}

pub const SHOVEL_X: f32 = 858.0;
pub const SHOVEL_Y: f32 = 16.0;
pub const SHOVEL_W: f32 = 62.0;
pub const SHOVEL_H: f32 = 72.0;

pub const WAVE_PANEL: (f32, f32, f32, f32) = (916.0, 14.0, 166.0, 76.0);
pub const BTN_PAUSE: (f32, f32, f32, f32) = (1092.0, 22.0, 46.0, 46.0);
pub const BTN_RESTART: (f32, f32, f32, f32) = (1148.0, 22.0, 46.0, 46.0);
pub const BTN_SOUND: (f32, f32, f32, f32) = (1204.0, 22.0, 46.0, 46.0);

pub const START_BTN: (f32, f32, f32, f32) = (490.0, 552.0, 300.0, 76.0);
pub const MENU_BTN: (f32, f32, f32, f32) = (490.0, 468.0, 300.0, 76.0);
pub const AGAIN_BTN: (f32, f32, f32, f32) = (490.0, 500.0, 300.0, 74.0);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlantKind {
    Sunflower,
    Peashooter,
    WallNut,
    Cherry,
    SnowPea,
    Repeater,
    PotatoMine,
    Jalapeno,
    Threepeater,
    Squash,
    Cabbage,
    Starfruit,
    Garlic,
    Spikeweed,
}

impl PlantKind {
    pub const ALL: [PlantKind; 14] = [
        PlantKind::Sunflower,
        PlantKind::Peashooter,
        PlantKind::WallNut,
        PlantKind::Cherry,
        PlantKind::SnowPea,
        PlantKind::Repeater,
        PlantKind::PotatoMine,
        PlantKind::Jalapeno,
        PlantKind::Threepeater,
        PlantKind::Squash,
        PlantKind::Cabbage,
        PlantKind::Starfruit,
        PlantKind::Garlic,
        PlantKind::Spikeweed,
    ];

    pub fn idx(self) -> usize {
        match self {
            PlantKind::Sunflower => 0,
            PlantKind::Peashooter => 1,
            PlantKind::WallNut => 2,
            PlantKind::Cherry => 3,
            PlantKind::SnowPea => 4,
            PlantKind::Repeater => 5,
            PlantKind::PotatoMine => 6,
            PlantKind::Jalapeno => 7,
            PlantKind::Threepeater => 8,
            PlantKind::Squash => 9,
            PlantKind::Cabbage => 10,
            PlantKind::Starfruit => 11,
            PlantKind::Garlic => 12,
            PlantKind::Spikeweed => 13,
        }
    }

    pub fn cost(self) -> i32 {
        match self {
            PlantKind::Sunflower => 50,
            PlantKind::Peashooter => 100,
            PlantKind::WallNut => 50,
            PlantKind::Cherry => 150,
            PlantKind::SnowPea => 175,
            PlantKind::Repeater => 200,
            PlantKind::PotatoMine => 25,
            PlantKind::Jalapeno => 125,
            PlantKind::Threepeater => 325,
            PlantKind::Squash => 50,
            PlantKind::Cabbage => 100,
            PlantKind::Starfruit => 125,
            PlantKind::Garlic => 50,
            PlantKind::Spikeweed => 100,
        }
    }

    /// 零售 1073 的种子包冷却，单位秒（原表毫秒换算）。
    pub fn cooldown(self) -> f32 {
        match self {
            PlantKind::Sunflower => 0.75,
            PlantKind::Peashooter => 0.75,
            PlantKind::WallNut => 3.0,
            PlantKind::Cherry => 5.0,
            PlantKind::SnowPea => 0.75,
            PlantKind::Repeater => 0.75,
            PlantKind::PotatoMine => 3.0,
            PlantKind::Jalapeno => 5.0,
            PlantKind::Threepeater => 0.75,
            PlantKind::Squash => 3.0,
            PlantKind::Cabbage => 0.75,
            PlantKind::Starfruit => 0.75,
            PlantKind::Garlic => 0.75,
            PlantKind::Spikeweed => 0.75,
        }
    }

    pub fn hp(self) -> f32 {
        match self {
            PlantKind::WallNut => 4000.0,
            PlantKind::Cherry => 9999.0,
            PlantKind::Garlic => 400.0,
            _ => 300.0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PlantKind::Sunflower => "向日葵",
            PlantKind::Peashooter => "豌豆射手",
            PlantKind::WallNut => "坚果墙",
            PlantKind::Cherry => "樱桃炸弹",
            PlantKind::SnowPea => "寒冰射手",
            PlantKind::Repeater => "双发射手",
            PlantKind::PotatoMine => "土豆雷",
            PlantKind::Jalapeno => "火爆辣椒",
            PlantKind::Threepeater => "三线射手",
            PlantKind::Squash => "窝瓜",
            PlantKind::Cabbage => "卷心菜投手",
            PlantKind::Starfruit => "星星果",
            PlantKind::Garlic => "大蒜",
            PlantKind::Spikeweed => "地刺",
        }
    }

    pub fn desc(self) -> &'static str {
        match self {
            PlantKind::Sunflower => "生产阳光，草坪经济核心",
            PlantKind::Peashooter => "向前发射豌豆，基础输出",
            PlantKind::WallNut => "超高血量，替你挡住僵尸",
            PlantKind::Cherry => "引爆 3×3 范围，清场利器",
            PlantKind::SnowPea => "冰豌豆减速，伤害同为 20",
            PlantKind::Repeater => "一次射两颗豌豆，火力翻倍",
            PlantKind::PotatoMine => "埋下 14 秒后武装，踩中即炸",
            PlantKind::Jalapeno => "点燃后烧光整行僵尸",
            PlantKind::Threepeater => "同时向相邻三行发射豌豆",
            PlantKind::Squash => "压扁前方最近的僵尸",
            PlantKind::Cabbage => "抛射卷心菜，越过铁门直接砸后排",
            PlantKind::Starfruit => "向五个方向同时发射星星",
            PlantKind::Garlic => "被咬后把僵尸引到相邻一行",
            PlantKind::Spikeweed => "僵尸踩过持续受伤，不会被啃食",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ZombieKind {
    Basic,
    Cone,
    Bucket,
    Flag,
    PoleVault,
    ScreenDoor,
    Football,
    Ladder,
}

impl ZombieKind {
    pub fn hp(self) -> f32 {
        match self {
            ZombieKind::Basic => 200.0,
            ZombieKind::Cone => 570.0,
            ZombieKind::Bucket => 1300.0,
            ZombieKind::Flag => 200.0,
            ZombieKind::PoleVault => 500.0,
            ZombieKind::ScreenDoor => 200.0,
            ZombieKind::Football => 1600.0,
            ZombieKind::Ladder => 500.0,
        }
    }

    /// 零售 1073：普通/路障/铁桶/铁门随机 0.23~0.37 px/tick（23~37 px/s）；
    /// 旗帜固定 0.45；撑杆、橄榄球 0.66~0.68；扶梯 0.79~0.81。
    pub fn speed_range(self) -> (f32, f32) {
        match self {
            ZombieKind::Basic
            | ZombieKind::Cone
            | ZombieKind::Bucket
            | ZombieKind::ScreenDoor => (23.0, 37.0),
            ZombieKind::Flag => (45.0, 45.0),
            ZombieKind::PoleVault | ZombieKind::Football => (66.0, 68.0),
            ZombieKind::Ladder => (79.0, 81.0),
        }
    }

    pub fn shield(self) -> f32 {
        match self {
            ZombieKind::ScreenDoor => 1100.0,
            ZombieKind::Ladder => 500.0,
            _ => 0.0,
        }
    }

    /// 原版 gZombieDefs 的出怪点数：普通 1、路障 2、撑杆 2、铁桶 4、铁门 4、橄榄球 7、扶梯 4。
    pub fn wave_value(self) -> u32 {
        match self {
            ZombieKind::Basic => 1,
            ZombieKind::Cone | ZombieKind::PoleVault => 2,
            ZombieKind::Bucket | ZombieKind::ScreenDoor | ZombieKind::Ladder => 4,
            ZombieKind::Football => 7,
            ZombieKind::Flag => 0,
        }
    }

    /// 原版 gZombieDefs 的选取权重。
    pub fn pick_weight(self) -> u32 {
        match self {
            ZombieKind::Basic => 4000,
            ZombieKind::Cone => 4000,
            ZombieKind::PoleVault => 2000,
            ZombieKind::Bucket => 3000,
            ZombieKind::ScreenDoor => 3500,
            ZombieKind::Football => 2000,
            ZombieKind::Ladder => 1000,
            ZombieKind::Flag => 0,
        }
    }
}

pub const LEVEL_COUNT: usize = 3;

#[derive(Clone, Copy)]
pub struct LevelCfg {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub sun: i32,
    pub waves: u32,
    pub first_wave: f32,
    pub flag_wave: u32,
    pub gaps: &'static [f32],
    pub tint: (u8, u8, u8, f32),
    pub pool: &'static [ZombieKind],
    pub plants: &'static [PlantKind],
    pub slots: usize,
    pub seed_hint: &'static str,
}

static GAPS_A: [f32; 7] = [30.0, 28.0, 26.0, 24.0, 22.0, 20.0, 18.0];
static GAPS_B: [f32; 9] = [28.0, 26.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0];
static GAPS_C: [f32; 11] = [26.0, 25.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0];
static POOL_A: [ZombieKind; 1] = [ZombieKind::Basic];
static POOL_B: [ZombieKind; 4] = [
    ZombieKind::Basic,
    ZombieKind::Cone,
    ZombieKind::Bucket,
    ZombieKind::ScreenDoor,
];
static POOL_C: [ZombieKind; 7] = [
    ZombieKind::Basic,
    ZombieKind::Cone,
    ZombieKind::Bucket,
    ZombieKind::PoleVault,
    ZombieKind::Football,
    ZombieKind::ScreenDoor,
    ZombieKind::Ladder,
];

static PLANTS_A: [PlantKind; 8] = [
    PlantKind::Sunflower,
    PlantKind::Peashooter,
    PlantKind::WallNut,
    PlantKind::Cherry,
    PlantKind::PotatoMine,
    PlantKind::SnowPea,
    PlantKind::Cabbage,
    PlantKind::Garlic,
];
static PLANTS_B: [PlantKind; 11] = [
    PlantKind::Sunflower,
    PlantKind::Peashooter,
    PlantKind::WallNut,
    PlantKind::Cherry,
    PlantKind::PotatoMine,
    PlantKind::SnowPea,
    PlantKind::Cabbage,
    PlantKind::Garlic,
    PlantKind::Repeater,
    PlantKind::Threepeater,
    PlantKind::Spikeweed,
];
static PLANTS_C: [PlantKind; 14] = [
    PlantKind::Sunflower,
    PlantKind::Peashooter,
    PlantKind::WallNut,
    PlantKind::Cherry,
    PlantKind::PotatoMine,
    PlantKind::SnowPea,
    PlantKind::Cabbage,
    PlantKind::Garlic,
    PlantKind::Repeater,
    PlantKind::Threepeater,
    PlantKind::Spikeweed,
    PlantKind::Jalapeno,
    PlantKind::Starfruit,
    PlantKind::Squash,
];

pub fn level_cfg(i: usize) -> LevelCfg {
    match i {
        1 => LevelCfg {
            name: "黄昏 · 前院",
            subtitle: "更多铁桶与铁门僵尸",
            sun: 90,
            waves: 6,
            first_wave: 18.0,
            flag_wave: 6,
            gaps: &GAPS_B,
            tint: (0, 0, 0, 0.0),
            pool: &POOL_B,
            plants: &PLANTS_B,
            slots: 7,
            seed_hint: "黄昏波次更密，优先带群攻和地刺",
        },
        2 => LevelCfg {
            name: "夜晚 · 前院",
            subtitle: "撑杆跳、橄榄球僵尸来袭",
            sun: 100,
            waves: 8,
            first_wave: 18.0,
            flag_wave: 8,
            gaps: &GAPS_C,
            tint: (0, 0, 0, 0.0),
            pool: &POOL_C,
            plants: &PLANTS_C,
            slots: 8,
            seed_hint: "夜晚没有天降阳光，必须带足向日葵",
        },
        _ => LevelCfg {
            name: "白天 · 前院",
            subtitle: "适应草坪与阳光节奏",
            sun: 75,
            waves: 4,
            first_wave: 18.0,
            flag_wave: 0,
            gaps: &GAPS_A,
            tint: (0, 0, 0, 0.0),
            pool: &POOL_A,
            plants: &PLANTS_A,
            slots: 6,
            seed_hint: "先选向日葵和豌豆，再补防御与清场",
        },
    }
}

pub fn level_rect(i: usize) -> (f32, f32, f32, f32) {
    (172.0 + i as f32 * 262.0, 466.0, 238.0, 62.0)
}

pub const CHOOSE_COLS: usize = 7;
pub const CHOOSE_X: f32 = 196.0;
pub const CHOOSE_Y: f32 = 262.0;
pub const CHOOSE_W: f32 = 118.0;
pub const CHOOSE_H: f32 = 104.0;
pub const CHOOSE_GAP_X: f32 = 10.0;
pub const CHOOSE_GAP_Y: f32 = 10.0;
pub const CHOOSE_START: (f32, f32, f32, f32) = (470.0, 624.0, 340.0, 64.0);

pub fn choose_rect(i: usize) -> (f32, f32, f32, f32) {
    let col = i % CHOOSE_COLS;
    let row = i / CHOOSE_COLS;
    (
        CHOOSE_X + col as f32 * (CHOOSE_W + CHOOSE_GAP_X),
        CHOOSE_Y + row as f32 * (CHOOSE_H + CHOOSE_GAP_Y),
        CHOOSE_W,
        CHOOSE_H,
    )
}

pub fn seed_slot_rect(i: usize) -> (f32, f32, f32, f32) {
    (330.0 + i as f32 * 68.0, 180.0, 60.0, 64.0)
}

pub fn cell_center(row: usize, col: usize) -> (f32, f32) {
    (
        GRID_X + col as f32 * CELL_W + CELL_W * 0.5,
        GRID_Y + row as f32 * CELL_H + CELL_H * 0.5,
    )
}

pub fn row_ground(row: usize) -> f32 {
    GRID_Y + (row as f32 + 1.0) * CELL_H - 10.0
}

pub fn rect_contains(r: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
    x >= r.0 && x <= r.0 + r.2 && y >= r.1 && y <= r.1 + r.3
}
