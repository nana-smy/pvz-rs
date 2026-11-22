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
    LilyPad,
    FlowerPot,
    Chomper,
    PuffShroom,
    SunShroom,
    FumeShroom,
    GraveBuster,
    HypnoShroom,
    ScaredyShroom,
    IceShroom,
    DoomShroom,
    TangleKelp,
    Torchwood,
    TallNut,
    SeaShroom,
    Plantern,
    Cactus,
    Blover,
    SplitPea,
    Pumpkin,
    MagnetShroom,
    KernelPult,
    CoffeeBean,
    UmbrellaLeaf,
    Marigold,
    MelonPult,
    GatlingPea,
    TwinSunflower,
    GloomShroom,
    Cattail,
    WinterMelon,
    GoldMagnet,
    Spikerock,
    CobCannon,
    Imitater,
}

impl PlantKind {
    pub const ALL: [PlantKind; 49] = [
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
        PlantKind::LilyPad,
        PlantKind::FlowerPot,
        PlantKind::Chomper,
        PlantKind::PuffShroom,
        PlantKind::SunShroom,
        PlantKind::FumeShroom,
        PlantKind::GraveBuster,
        PlantKind::HypnoShroom,
        PlantKind::ScaredyShroom,
        PlantKind::IceShroom,
        PlantKind::DoomShroom,
        PlantKind::TangleKelp,
        PlantKind::Torchwood,
        PlantKind::TallNut,
        PlantKind::SeaShroom,
        PlantKind::Plantern,
        PlantKind::Cactus,
        PlantKind::Blover,
        PlantKind::SplitPea,
        PlantKind::Pumpkin,
        PlantKind::MagnetShroom,
        PlantKind::KernelPult,
        PlantKind::CoffeeBean,
        PlantKind::UmbrellaLeaf,
        PlantKind::Marigold,
        PlantKind::MelonPult,
        PlantKind::GatlingPea,
        PlantKind::TwinSunflower,
        PlantKind::GloomShroom,
        PlantKind::Cattail,
        PlantKind::WinterMelon,
        PlantKind::GoldMagnet,
        PlantKind::Spikerock,
        PlantKind::CobCannon,
        PlantKind::Imitater,
    ];

    pub fn idx(self) -> usize { self as usize }

    pub fn is_base(self) -> bool { matches!(self, PlantKind::LilyPad | PlantKind::FlowerPot) }

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
            PlantKind::LilyPad => 25,
            PlantKind::FlowerPot => 25,
            PlantKind::Chomper => 150,
            PlantKind::PuffShroom => 0,
            PlantKind::SunShroom => 25,
            PlantKind::FumeShroom => 75,
            PlantKind::GraveBuster => 75,
            PlantKind::HypnoShroom => 75,
            PlantKind::ScaredyShroom => 25,
            PlantKind::IceShroom => 75,
            PlantKind::DoomShroom => 125,
            PlantKind::TangleKelp => 25,
            PlantKind::Torchwood => 175,
            PlantKind::TallNut => 125,
            PlantKind::SeaShroom => 0,
            PlantKind::Plantern => 25,
            PlantKind::Cactus => 125,
            PlantKind::Blover => 100,
            PlantKind::SplitPea => 125,
            PlantKind::Pumpkin => 125,
            PlantKind::MagnetShroom => 100,
            PlantKind::KernelPult => 100,
            PlantKind::CoffeeBean => 75,
            PlantKind::UmbrellaLeaf => 100,
            PlantKind::Marigold => 50,
            PlantKind::MelonPult => 300,
            PlantKind::GatlingPea => 250,
            PlantKind::TwinSunflower => 150,
            PlantKind::GloomShroom => 150,
            PlantKind::Cattail => 225,
            PlantKind::WinterMelon => 200,
            PlantKind::GoldMagnet => 50,
            PlantKind::Spikerock => 125,
            PlantKind::CobCannon => 500,
            PlantKind::Imitater => 50,
        }
    }

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
            PlantKind::LilyPad => 0.75,
            PlantKind::FlowerPot => 0.75,
            PlantKind::Chomper => 0.75,
            PlantKind::PuffShroom => 0.75,
            PlantKind::SunShroom => 0.75,
            PlantKind::FumeShroom => 0.75,
            PlantKind::GraveBuster => 0.75,
            PlantKind::HypnoShroom => 3.0,
            PlantKind::ScaredyShroom => 0.75,
            PlantKind::IceShroom => 5.0,
            PlantKind::DoomShroom => 5.0,
            PlantKind::TangleKelp => 3.0,
            PlantKind::Torchwood => 0.75,
            PlantKind::TallNut => 3.0,
            PlantKind::SeaShroom => 3.0,
            PlantKind::Plantern => 3.0,
            PlantKind::Cactus => 0.75,
            PlantKind::Blover => 0.75,
            PlantKind::SplitPea => 0.75,
            PlantKind::Pumpkin => 3.0,
            PlantKind::MagnetShroom => 0.75,
            PlantKind::KernelPult => 0.75,
            PlantKind::CoffeeBean => 0.75,
            PlantKind::UmbrellaLeaf => 0.75,
            PlantKind::Marigold => 3.0,
            PlantKind::MelonPult => 0.75,
            PlantKind::GatlingPea => 5.0,
            PlantKind::TwinSunflower => 5.0,
            PlantKind::GloomShroom => 5.0,
            PlantKind::Cattail => 5.0,
            PlantKind::WinterMelon => 5.0,
            PlantKind::GoldMagnet => 5.0,
            PlantKind::Spikerock => 5.0,
            PlantKind::CobCannon => 5.0,
            PlantKind::Imitater => 0.75,
        }
    }

    pub fn hp(self) -> f32 {
        match self {
            PlantKind::Sunflower => 3e+02,
            PlantKind::Peashooter => 3e+02,
            PlantKind::SnowPea => 3e+02,
            PlantKind::Repeater => 3e+02,
            PlantKind::PotatoMine => 3e+02,
            PlantKind::Jalapeno => 3e+02,
            PlantKind::Threepeater => 3e+02,
            PlantKind::Squash => 3e+02,
            PlantKind::Cabbage => 3e+02,
            PlantKind::Starfruit => 3e+02,
            PlantKind::Garlic => 4e+02,
            PlantKind::Spikeweed => 3e+02,
            PlantKind::LilyPad => 4e+02,
            PlantKind::FlowerPot => 4e+02,
            PlantKind::Chomper => 3e+02,
            PlantKind::PuffShroom => 3e+02,
            PlantKind::SunShroom => 3e+02,
            PlantKind::FumeShroom => 3e+02,
            PlantKind::GraveBuster => 3e+02,
            PlantKind::HypnoShroom => 3e+02,
            PlantKind::ScaredyShroom => 3e+02,
            PlantKind::IceShroom => 3e+02,
            PlantKind::DoomShroom => 3e+02,
            PlantKind::TangleKelp => 3e+02,
            PlantKind::Torchwood => 3e+02,
            PlantKind::TallNut => 8e+03,
            PlantKind::SeaShroom => 3e+02,
            PlantKind::Plantern => 3e+02,
            PlantKind::Cactus => 3e+02,
            PlantKind::Blover => 3e+02,
            PlantKind::SplitPea => 3e+02,
            PlantKind::Pumpkin => 2e+03,
            PlantKind::MagnetShroom => 3e+02,
            PlantKind::KernelPult => 3e+02,
            PlantKind::CoffeeBean => 3e+02,
            PlantKind::UmbrellaLeaf => 3e+02,
            PlantKind::Marigold => 3e+02,
            PlantKind::MelonPult => 3e+02,
            PlantKind::GatlingPea => 3e+02,
            PlantKind::TwinSunflower => 3e+02,
            PlantKind::GloomShroom => 3e+02,
            PlantKind::Cattail => 3e+02,
            PlantKind::WinterMelon => 3e+02,
            PlantKind::GoldMagnet => 3e+02,
            PlantKind::Spikerock => 1e+03,
            PlantKind::CobCannon => 5e+02,
            PlantKind::Imitater => 3e+02,
            PlantKind::WallNut => 4000.0,
            PlantKind::Cherry => 9999.0,
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
            PlantKind::LilyPad => "睡莲",
            PlantKind::FlowerPot => "花盆",
            PlantKind::Chomper => "大嘴花",
            PlantKind::PuffShroom => "小喷菇",
            PlantKind::SunShroom => "阳光菇",
            PlantKind::FumeShroom => "大喷菇",
            PlantKind::GraveBuster => "墓碑吞噬者",
            PlantKind::HypnoShroom => "魅惑菇",
            PlantKind::ScaredyShroom => "胆小菇",
            PlantKind::IceShroom => "寒冰菇",
            PlantKind::DoomShroom => "毁灭菇",
            PlantKind::TangleKelp => "缠绕水草",
            PlantKind::Torchwood => "火炬树桩",
            PlantKind::TallNut => "高坚果",
            PlantKind::SeaShroom => "海蘑菇",
            PlantKind::Plantern => "路灯花",
            PlantKind::Cactus => "仙人掌",
            PlantKind::Blover => "三叶草",
            PlantKind::SplitPea => "裂荚射手",
            PlantKind::Pumpkin => "南瓜头",
            PlantKind::MagnetShroom => "磁力菇",
            PlantKind::KernelPult => "玉米投手",
            PlantKind::CoffeeBean => "咖啡豆",
            PlantKind::UmbrellaLeaf => "叶子保护伞",
            PlantKind::Marigold => "金盏花",
            PlantKind::MelonPult => "西瓜投手",
            PlantKind::GatlingPea => "机枪射手",
            PlantKind::TwinSunflower => "双子向日葵",
            PlantKind::GloomShroom => "忧郁菇",
            PlantKind::Cattail => "香蒲",
            PlantKind::WinterMelon => "冰西瓜",
            PlantKind::GoldMagnet => "吸金菇",
            PlantKind::Spikerock => "地刺王",
            PlantKind::CobCannon => "玉米加农炮",
            PlantKind::Imitater => "模仿者",
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
            PlantKind::LilyPad => "种在水面，承载水上植物",
            PlantKind::FlowerPot => "放在屋顶，承载屋顶植物",
            PlantKind::Chomper => "一口吞下整只僵尸，咀嚼时无法攻击",
            PlantKind::PuffShroom => "零阳光短射程蘑菇，前期救急",
            PlantKind::SunShroom => "长大后产出更多阳光",
            PlantKind::FumeShroom => "短距离穿透喷雾，可越过铁门",
            PlantKind::GraveBuster => "清除墓碑；本作中兼作廉价肉盾",
            PlantKind::HypnoShroom => "被啃食后魅惑僵尸，使其倒戈",
            PlantKind::ScaredyShroom => "僵尸靠近时缩起来，远处持续射击",
            PlantKind::IceShroom => "冻结全场僵尸并附带减速",
            PlantKind::DoomShroom => "大范围毁灭爆炸，留下焦土",
            PlantKind::TangleKelp => "把水面上的僵尸拖入水底",
            PlantKind::Torchwood => "豌豆穿过时点燃，伤害翻倍",
            PlantKind::TallNut => "比坚果墙更高更耐啃的屏障",
            PlantKind::SeaShroom => "零阳光水上射手，需种在睡莲上",
            PlantKind::Plantern => "照亮浓雾，扩大视野",
            PlantKind::Cactus => "发射尖刺，可击落空中目标",
            PlantKind::Blover => "吹退全场僵尸，清除飞行单位",
            PlantKind::SplitPea => "同时向前后两个方向射击",
            PlantKind::Pumpkin => "厚重外壳，保护格内植物",
            PlantKind::MagnetShroom => "吸走铁桶、铁门等金属护具",
            PlantKind::KernelPult => "抛射玉米粒，黄油可短暂定身",
            PlantKind::CoffeeBean => "唤醒睡眠中的蘑菇，使其立即工作",
            PlantKind::UmbrellaLeaf => "挡住投掷物，保护相邻植物",
            PlantKind::Marigold => "产出金币；本作折算为阳光",
            PlantKind::MelonPult => "抛射西瓜，命中后溅射周围僵尸",
            PlantKind::GatlingPea => "一次连发四颗豌豆，终极火力",
            PlantKind::TwinSunflower => "一次产出两份阳光",
            PlantKind::GloomShroom => "向四周喷射孢子，范围持续伤害",
            PlantKind::Cattail => "追踪全场僵尸，跨行发射尖刺",
            PlantKind::WinterMelon => "西瓜溅射并冻结减速整片僵尸",
            PlantKind::GoldMagnet => "吸取金币；本作定期产出阳光",
            PlantKind::Spikerock => "高伤害地刺，能扎破车轮",
            PlantKind::CobCannon => "发射玉米炮弹，大范围毁灭打击",
            PlantKind::Imitater => "复制上一株种下的植物",
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
    Newspaper,
    Dancing,
    BackupDancer,
    DuckyTube,
    Snorkel,
    Zomboni,
    Bobsled,
    DolphinRider,
    Jackbox,
    Balloon,
    Digger,
    Pogo,
    Yeti,
    Bungee,
    Catapult,
    Gargantuar,
    Imp,
    Boss,
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
            ZombieKind::Newspaper => 400.0,
            ZombieKind::Dancing => 500.0,
            ZombieKind::BackupDancer => 200.0,
            ZombieKind::DuckyTube => 200.0,
            ZombieKind::Snorkel => 300.0,
            ZombieKind::Zomboni => 1350.0,
            ZombieKind::Bobsled => 400.0,
            ZombieKind::DolphinRider => 500.0,
            ZombieKind::Jackbox => 400.0,
            ZombieKind::Balloon => 200.0,
            ZombieKind::Digger => 300.0,
            ZombieKind::Pogo => 500.0,
            ZombieKind::Yeti => 4000.0,
            ZombieKind::Bungee => 400.0,
            ZombieKind::Catapult => 500.0,
            ZombieKind::Gargantuar => 3000.0,
            ZombieKind::Imp => 200.0,
            ZombieKind::Boss => 40000.0,
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
            ZombieKind::Newspaper => (23.0, 37.0),
            ZombieKind::Dancing | ZombieKind::BackupDancer => (45.0, 45.0),
            ZombieKind::DuckyTube | ZombieKind::Snorkel => (23.0, 37.0),
            ZombieKind::Zomboni => (55.0, 60.0),
            ZombieKind::Bobsled => (75.0, 85.0),
            ZombieKind::DolphinRider => (60.0, 65.0),
            ZombieKind::Jackbox | ZombieKind::Pogo => (23.0, 37.0),
            ZombieKind::Balloon => (45.0, 50.0),
            ZombieKind::Digger => (45.0, 50.0),
            ZombieKind::Yeti => (40.0, 48.0),
            ZombieKind::Bungee => (0.0, 0.0),
            ZombieKind::Catapult => (23.0, 37.0),
            ZombieKind::Gargantuar => (23.0, 37.0),
            ZombieKind::Imp => (60.0, 70.0),
            ZombieKind::Boss => (20.0, 25.0),
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
            ZombieKind::Flag | ZombieKind::BackupDancer | ZombieKind::Imp => 0,
            ZombieKind::Newspaper => 2,
            ZombieKind::Dancing => 5,
            ZombieKind::DuckyTube => 1,
            ZombieKind::Snorkel => 2,
            ZombieKind::Zomboni => 6,
            ZombieKind::Bobsled => 4,
            ZombieKind::DolphinRider => 3,
            ZombieKind::Jackbox => 3,
            ZombieKind::Balloon => 2,
            ZombieKind::Digger => 3,
            ZombieKind::Pogo => 3,
            ZombieKind::Yeti => 10,
            ZombieKind::Bungee => 3,
            ZombieKind::Catapult => 4,
            ZombieKind::Gargantuar => 10,
            ZombieKind::Boss => 20,
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
            ZombieKind::Newspaper => 1500,
            ZombieKind::Dancing => 1000,
            ZombieKind::BackupDancer => 0,
            ZombieKind::DuckyTube => 1200,
            ZombieKind::Snorkel => 1200,
            ZombieKind::Zomboni => 700,
            ZombieKind::Bobsled => 900,
            ZombieKind::DolphinRider => 900,
            ZombieKind::Jackbox => 800,
            ZombieKind::Balloon => 1200,
            ZombieKind::Digger => 800,
            ZombieKind::Pogo => 900,
            ZombieKind::Yeti => 100,
            ZombieKind::Bungee => 700,
            ZombieKind::Catapult => 700,
            ZombieKind::Gargantuar => 500,
            ZombieKind::Imp => 0,
            ZombieKind::Boss => 100,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    Lawn,
    Pool,
    Roof,
}

pub fn rows_for(terrain: Terrain) -> usize {
    if terrain == Terrain::Pool { 6 } else { 5 }
}

pub fn cell_h_for(terrain: Terrain) -> f32 {
    (LAWN_BOTTOM - GRID_Y) / rows_for(terrain) as f32
}

pub fn is_water_row(terrain: Terrain, row: usize) -> bool {
    terrain == Terrain::Pool && (row == 2 || row == 3)
}

pub const LEVEL_COUNT: usize = 50;
pub const WORLD_COUNT: usize = 5;
pub const LEVELS_PER_WORLD: usize = 10;

pub const WORLD_NAMES: [&str; WORLD_COUNT] = [
    "白天 · 前院",
    "夜晚 · 前院",
    "泳池 · 白天",
    "浓雾 · 夜晚",
    "屋顶 · 黄昏",
];
pub const WORLD_SUBS: [&str; WORLD_COUNT] = [
    "第 1 章 · 1-1 至 1-10",
    "第 2 章 · 2-1 至 2-10",
    "第 3 章 · 3-1 至 3-10",
    "第 4 章 · 4-1 至 4-10",
    "第 5 章 · 5-1 至 5-10",
];
pub const WORLD_HINTS: [&str; WORLD_COUNT] = [
    "白天阳光充足，先铺向日葵再补输出",
    "夜晚阳光稀少，优先带足经济植物",
    "第 3、4 行是水面，先种睡莲再种植物",
    "浓雾中视野受限，带上范围清场植物",
    "屋顶先放花盆才能种植物，注意斜坡",
];
static WORLD_TINT: [(u8, u8, u8, f32); WORLD_COUNT] = [
    (0, 0, 0, 0.0),
    (10, 14, 45, 0.32),
    (0, 45, 55, 0.10),
    (55, 65, 65, 0.22),
    (55, 25, 10, 0.18),
];
static WORLD_SUN: [i32; WORLD_COUNT] = [75, 100, 100, 100, 100];
static WORLD_SLOTS: [usize; WORLD_COUNT] = [6, 7, 8, 8, 9];

/// 来自 PvZ-Portable 的 `gZombieWaves`：1-1 到 5-10 的原始波次数。
static WAVE_COUNTS: [u32; LEVEL_COUNT] = [
    4,6,8,10,8,10,20,10,20,20,
    10,20,10,20,10,10,20,10,20,20,
    10,20,20,30,20,20,30,20,30,30,
    10,20,10,20,20,10,20,10,20,20,
    10,20,20,30,20,20,30,20,30,30,
];
static GAPS_50: [f32; 30] = [
    22.0,21.0,20.0,19.0,18.0,18.0,17.0,17.0,16.0,16.0,
    15.0,15.0,15.0,14.0,14.0,14.0,13.0,13.0,13.0,12.0,
    12.0,12.0,11.0,11.0,11.0,10.0,10.0,10.0,9.0,9.0,
];

static POOL_00: [ZombieKind; 1] = [ZombieKind::Basic];
static POOL_01: [ZombieKind; 1] = [ZombieKind::Basic];
static POOL_02: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_03: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_04: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_05: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault];
static POOL_06: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault];
static POOL_07: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket];
static POOL_08: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Bucket];
static POOL_09: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Bucket];
static POOL_10: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_11: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket];
static POOL_12: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::ScreenDoor];
static POOL_13: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::ScreenDoor];
static POOL_14: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Bucket];
static POOL_15: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_16: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::ScreenDoor, ZombieKind::Football];
static POOL_17: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_18: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::ScreenDoor, ZombieKind::Football];
static POOL_19: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::ScreenDoor, ZombieKind::Football];
static POOL_20: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_21: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football];
static POOL_22: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_23: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Bucket];
static POOL_24: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_25: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_26: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football];
static POOL_27: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault];
static POOL_28: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Bucket, ZombieKind::Football];
static POOL_29: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football, ZombieKind::PoleVault];
static POOL_30: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_31: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_32: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Ladder];
static POOL_33: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Ladder];
static POOL_34: [ZombieKind; 2] = [ZombieKind::Basic, ZombieKind::Cone];
static POOL_35: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Ladder];
static POOL_36: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football, ZombieKind::Ladder];
static POOL_37: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Ladder];
static POOL_38: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Ladder];
static POOL_39: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football, ZombieKind::Ladder];
static POOL_40: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Ladder];
static POOL_41: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::PoleVault, ZombieKind::Bucket, ZombieKind::Ladder];
static POOL_42: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Ladder];
static POOL_43: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football, ZombieKind::Ladder];
static POOL_44: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Ladder];
static POOL_45: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_46: [ZombieKind; 4] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Ladder, ZombieKind::Football];
static POOL_47: [ZombieKind; 3] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Football];
static POOL_48: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football, ZombieKind::Ladder];
static POOL_49: [ZombieKind; 5] = [ZombieKind::Basic, ZombieKind::Cone, ZombieKind::Bucket, ZombieKind::Football, ZombieKind::Ladder];
static LEVEL_POOLS: [&'static [ZombieKind]; 50] = [
    &POOL_00, &POOL_01, &POOL_02, &POOL_03, &POOL_04, &POOL_05, &POOL_06, &POOL_07, &POOL_08, &POOL_09, &POOL_10, &POOL_11, &POOL_12, &POOL_13, &POOL_14, &POOL_15, &POOL_16, &POOL_17, &POOL_18, &POOL_19, &POOL_20, &POOL_21, &POOL_22, &POOL_23, &POOL_24, &POOL_25, &POOL_26, &POOL_27, &POOL_28, &POOL_29, &POOL_30, &POOL_31, &POOL_32, &POOL_33, &POOL_34, &POOL_35, &POOL_36, &POOL_37, &POOL_38, &POOL_39, &POOL_40, &POOL_41, &POOL_42, &POOL_43, &POOL_44, &POOL_45, &POOL_46, &POOL_47, &POOL_48, &POOL_49,
];

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
    pub terrain: Terrain,
    pub slots: usize,
    pub seed_hint: &'static str,
}

/// 植物按关卡逐步解锁，顺序沿用原版冒险模式的获得节奏，映射到本作已有的 14 种植物。
static PLANT_UNLOCK_ORDER: [PlantKind; 49] = [
    PlantKind::Peashooter,
    PlantKind::Sunflower,
    PlantKind::Cherry,
    PlantKind::WallNut,
    PlantKind::PotatoMine,
    PlantKind::SnowPea,
    PlantKind::Chomper,
    PlantKind::Repeater,
    PlantKind::PuffShroom,
    PlantKind::SunShroom,
    PlantKind::FumeShroom,
    PlantKind::GraveBuster,
    PlantKind::HypnoShroom,
    PlantKind::ScaredyShroom,
    PlantKind::IceShroom,
    PlantKind::DoomShroom,
    PlantKind::LilyPad,
    PlantKind::Squash,
    PlantKind::Threepeater,
    PlantKind::TangleKelp,
    PlantKind::Jalapeno,
    PlantKind::Spikeweed,
    PlantKind::Torchwood,
    PlantKind::TallNut,
    PlantKind::SeaShroom,
    PlantKind::Plantern,
    PlantKind::Cactus,
    PlantKind::Blover,
    PlantKind::SplitPea,
    PlantKind::Starfruit,
    PlantKind::Pumpkin,
    PlantKind::MagnetShroom,
    PlantKind::Cabbage,
    PlantKind::FlowerPot,
    PlantKind::KernelPult,
    PlantKind::CoffeeBean,
    PlantKind::Garlic,
    PlantKind::UmbrellaLeaf,
    PlantKind::Marigold,
    PlantKind::MelonPult,
    PlantKind::GatlingPea,
    PlantKind::TwinSunflower,
    PlantKind::GloomShroom,
    PlantKind::Cattail,
    PlantKind::WinterMelon,
    PlantKind::GoldMagnet,
    PlantKind::Spikerock,
    PlantKind::CobCannon,
    PlantKind::Imitater,
];
static PLANT_UNLOCK_AT: [usize; 49] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
    13,
    14,
    15,
    16,
    17,
    18,
    19,
    20,
    21,
    22,
    23,
    24,
    25,
    26,
    27,
    28,
    29,
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    46,
    47,
    48,
];

pub fn plants_for_level(level: usize) -> &'static [PlantKind] {
    let n = PLANT_UNLOCK_AT.iter().filter(|x| **x <= level).count().max(1);
    &PLANT_UNLOCK_ORDER[..n]
}

pub fn level_cfg(i: usize) -> LevelCfg {
    let level = i.min(LEVEL_COUNT - 1);
    let world = (level / LEVELS_PER_WORLD).min(WORLD_COUNT - 1);
    let waves = WAVE_COUNTS[level];
    LevelCfg {
        name: WORLD_NAMES[world],
        subtitle: WORLD_SUBS[world],
        sun: if level < 2 { 150 } else { WORLD_SUN[world] },
        waves,
        first_wave: 18.0,
        flag_wave: if waves >= 10 { waves / 2 } else { 0 },
        gaps: &GAPS_50,
        tint: WORLD_TINT[world],
        pool: LEVEL_POOLS[level],
        plants: plants_for_level(level),
        terrain: match world {
            2 | 3 => Terrain::Pool,
            4 => Terrain::Roof,
            _ => Terrain::Lawn,
        },
        slots: WORLD_SLOTS[world],
        seed_hint: WORLD_HINTS[world],
    }
}

pub fn level_rect(i: usize) -> (f32, f32, f32, f32) {
    let row = i / 5;
    let raw = i % 5;
    let col = if row == 0 { raw } else { 4 - raw };
    (300.0 + col as f32 * 175.0, 408.0 + row as f32 * 126.0, 132.0, 58.0)
}

pub fn world_tab_rect(i: usize) -> (f32, f32, f32, f32) {
    (246.0 + i as f32 * 158.0, 160.0, 146.0, 38.0)
}

pub const BTN_ADVENTURE: (f32, f32, f32, f32) = (468.0, 414.0, 344.0, 82.0);
pub const MAP_BACK: (f32, f32, f32, f32) = (16.0, 14.0, 104.0, 38.0);

pub const CHOOSE_COLS: usize = 10;
pub const CHOOSE_X: f32 = 178.0;
pub const CHOOSE_Y: f32 = 248.0;
pub const CHOOSE_W: f32 = 96.0;
pub const CHOOSE_H: f32 = 74.0;
pub const CHOOSE_GAP_X: f32 = 6.0;
pub const CHOOSE_GAP_Y: f32 = 6.0;
pub const CHOOSE_START: (f32, f32, f32, f32) = (470.0, 660.0, 340.0, 52.0);

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
    (330.0 + i as f32 * 62.0, 180.0, 56.0, 64.0)
}

pub fn cell_center(row: usize, col: usize, h: f32) -> (f32, f32) {
    (
        GRID_X + col as f32 * CELL_W + CELL_W * 0.5,
        GRID_Y + row as f32 * h + h * 0.5,
    )
}

pub fn row_ground(row: usize, h: f32) -> f32 {
    GRID_Y + (row as f32 + 1.0) * h - 10.0
}

pub fn rect_contains(r: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
    x >= r.0 && x <= r.0 + r.2 && y >= r.1 && y <= r.1 + r.3
}
