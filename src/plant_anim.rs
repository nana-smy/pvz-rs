// 由 assets/plants/reanim/manifest.json 生成：原版 reanim 逐帧烘焙的精灵表元数据

#[derive(Clone, Copy)]
pub struct Clip {
    pub key: &'static str,
    pub frames: u32,
    pub w: f32,
    pub h: f32,
}

pub const IDLE: [Clip; 16] = [
    Clip { key: "pf_sunflower_idle", frames: 25, w: 83.0, h: 83.0 },
    Clip { key: "pf_peashooter_idle", frames: 25, w: 83.0, h: 80.0 },
    Clip { key: "pf_repeater_idle", frames: 25, w: 83.0, h: 78.0 },
    Clip { key: "pf_wallnut_idle", frames: 17, w: 85.0, h: 82.0 },
    Clip { key: "pf_cherrybomb_idle", frames: 13, w: 98.0, h: 78.0 },
    Clip { key: "pf_snowpea_idle", frames: 25, w: 81.0, h: 80.0 },
    Clip { key: "pf_potatomine_armed", frames: 11, w: 104.0, h: 84.0 },
    Clip { key: "pf_jalapeno_idle", frames: 7, w: 56.0, h: 85.0 },
    Clip { key: "pf_threepeater_idle", frames: 25, w: 63.0, h: 69.0 },
    Clip { key: "pf_squash_idle", frames: 19, w: 81.0, h: 98.0 },
    Clip { key: "pf_cabbagepult_idle", frames: 31, w: 105.0, h: 96.0 },
    Clip { key: "pf_starfruit_idle", frames: 17, w: 86.0, h: 80.0 },
    Clip { key: "pf_garlic_idle", frames: 15, w: 69.0, h: 117.0 },
    Clip { key: "pf_spikeweed_idle", frames: 21, w: 92.0, h: 44.0 },
    Clip { key: "pf_lilypad_idle", frames: 1, w: 88.0, h: 88.0 },
    Clip { key: "pf_flowerpot_idle", frames: 1, w: 74.0, h: 72.0 },
];

pub const RISE_POTATO: Clip = Clip { key: "pf_potatomine_rise", frames: 19, w: 105.0, h: 87.0 };

pub const SHOOT: [Clip; 16] = [
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "pf_peashooter_shoot", frames: 25, w: 80.0, h: 52.0 },
    Clip { key: "pf_repeater_shoot", frames: 25, w: 82.0, h: 52.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "pf_snowpea_shoot", frames: 25, w: 78.0, h: 52.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "pf_cabbagepult_shoot", frames: 38, w: 110.0, h: 116.0 },
    Clip { key: "pf_starfruit_shoot", frames: 17, w: 88.0, h: 80.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
    Clip { key: "", frames: 0, w: 0.0, h: 0.0 },
];
