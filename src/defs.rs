// 全局常量与数值定义
pub const W: f32 = 1280.0;
pub const H: f32 = 720.0;

pub const ROWS: usize = 5;
pub const COLS: usize = 9;

pub const GRID_X: f32 = 300.0;
pub const GRID_Y: f32 = 112.0;
pub const CELL_W: f32 = 96.0;
pub const CELL_H: f32 = 100.0;

pub const HUD_H: f32 = 104.0;
pub const LAWN_BOTTOM: f32 = 612.0;

pub const TOTAL_WAVES: u32 = 10;
pub const FIRST_WAVE_AT: f32 = 25.0;
pub const WAVE_GAPS: [f32; 9] = [24.0, 23.0, 22.0, 21.0, 21.0, 20.0, 20.0, 19.0, 19.0];

pub const MOWER_X: f32 = 268.0;
pub const DEATH_X: f32 = 238.0;

// 卡片 HUD 布局
pub const CARD_X0: f32 = 148.0;
pub const CARD_Y: f32 = 16.0;
pub const CARD_W: f32 = 86.0;
pub const CARD_H: f32 = 74.0;
pub const CARD_GAP: f32 = 8.0;

pub fn card_rect(i: usize) -> (f32, f32, f32, f32) {
    (
        CARD_X0 + i as f32 * (CARD_W + CARD_GAP),
        CARD_Y,
        CARD_W,
        CARD_H,
    )
}

pub const SHOVEL_X: f32 = 718.0;
pub const SHOVEL_Y: f32 = 16.0;
pub const SHOVEL_W: f32 = 74.0;
pub const SHOVEL_H: f32 = 74.0;

pub const WAVE_PANEL: (f32, f32, f32, f32) = (872.0, 14.0, 206.0, 76.0);
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
}

impl PlantKind {
    pub const ALL: [PlantKind; 6] = [
        PlantKind::Sunflower,
        PlantKind::Peashooter,
        PlantKind::WallNut,
        PlantKind::Cherry,
        PlantKind::SnowPea,
        PlantKind::Repeater,
    ];

    pub fn idx(self) -> usize {
        match self {
            PlantKind::Sunflower => 0,
            PlantKind::Peashooter => 1,
            PlantKind::WallNut => 2,
            PlantKind::Cherry => 3,
            PlantKind::SnowPea => 4,
            PlantKind::Repeater => 5,
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
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ZombieKind {
    Basic,
    Cone,
    Bucket,
    Flag,
}

impl ZombieKind {
    pub fn hp(self) -> f32 {
        match self {
            ZombieKind::Basic => 220.0,
            ZombieKind::Cone => 600.0,
            ZombieKind::Bucket => 1400.0,
            ZombieKind::Flag => 260.0,
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            ZombieKind::Basic => 20.0,
            ZombieKind::Cone => 19.5,
            ZombieKind::Bucket => 17.0,
            ZombieKind::Flag => 28.0,
        }
    }
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
