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
}

impl PlantKind {
    pub const ALL: [PlantKind; 10] = [
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
            PlantKind::Squash => 125,
        }
    }

    pub fn cooldown(self) -> f32 {
        match self {
            PlantKind::Sunflower => 5.0,
            PlantKind::Peashooter => 5.0,
            PlantKind::WallNut => 20.0,
            PlantKind::Cherry => 25.0,
            PlantKind::SnowPea => 8.0,
            PlantKind::Repeater => 10.0,
            PlantKind::PotatoMine => 18.0,
            PlantKind::Jalapeno => 30.0,
            PlantKind::Threepeater => 12.0,
            PlantKind::Squash => 20.0,
        }
    }

    pub fn hp(self) -> f32 {
        match self {
            PlantKind::WallNut => 3000.0,
            PlantKind::Cherry => 9999.0,
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
}

impl ZombieKind {
    pub fn hp(self) -> f32 {
        match self {
            ZombieKind::Basic => 220.0,
            ZombieKind::Cone => 600.0,
            ZombieKind::Bucket => 1400.0,
            ZombieKind::Flag => 260.0,
            ZombieKind::PoleVault => 500.0,
            ZombieKind::ScreenDoor => 240.0,
            ZombieKind::Football => 1600.0,
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            ZombieKind::Basic => 20.0,
            ZombieKind::Cone => 19.5,
            ZombieKind::Bucket => 17.0,
            ZombieKind::Flag => 28.0,
            ZombieKind::PoleVault => 34.0,
            ZombieKind::ScreenDoor => 18.0,
            ZombieKind::Football => 32.0,
        }
    }

    pub fn shield(self) -> f32 {
        match self {
            ZombieKind::ScreenDoor => 1100.0,
            _ => 0.0,
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
    pub gaps: &'static [f32],
    pub tint: (u8, u8, u8, f32),
    pub pool: &'static [ZombieKind],
}

static GAPS_A: [f32; 7] = [23.0, 22.0, 21.0, 20.0, 20.0, 19.0, 18.0];
static GAPS_B: [f32; 9] = [22.0, 21.0, 20.0, 20.0, 19.0, 19.0, 18.0, 18.0, 17.0];
static GAPS_C: [f32; 11] = [21.0, 20.0, 20.0, 19.0, 19.0, 18.0, 18.0, 17.0, 17.0, 16.0, 16.0];
static POOL_A: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_B: [ZombieKind; 4] = [
    ZombieKind::Basic,
    ZombieKind::Cone,
    ZombieKind::Bucket,
    ZombieKind::ScreenDoor,
];
static POOL_C: [ZombieKind; 6] = [
    ZombieKind::Basic,
    ZombieKind::Cone,
    ZombieKind::Bucket,
    ZombieKind::PoleVault,
    ZombieKind::Football,
    ZombieKind::ScreenDoor,
];

pub fn level_cfg(i: usize) -> LevelCfg {
    match i {
        1 => LevelCfg {
            name: "黄昏 · 前院",
            subtitle: "更多铁桶与铁门僵尸",
            sun: 175,
            waves: 10,
            first_wave: 22.0,
            gaps: &GAPS_B,
            tint: (0, 0, 0, 0.0),
            pool: &POOL_B,
        },
        2 => LevelCfg {
            name: "夜晚 · 前院",
            subtitle: "撑杆跳、橄榄球僵尸来袭",
            sun: 150,
            waves: 12,
            first_wave: 20.0,
            gaps: &GAPS_C,
            tint: (0, 0, 0, 0.0),
            pool: &POOL_C,
        },
        _ => LevelCfg {
            name: "白天 · 前院",
            subtitle: "适应草坪与阳光节奏",
            sun: 150,
            waves: 8,
            first_wave: 24.0,
            gaps: &GAPS_A,
            tint: (0, 0, 0, 0.0),
            pool: &POOL_A,
        },
    }
}

pub fn level_rect(i: usize) -> (f32, f32, f32, f32) {
    (172.0 + i as f32 * 262.0, 466.0, 238.0, 62.0)
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
