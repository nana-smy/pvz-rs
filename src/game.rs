use crate::defs::*;
use crate::sfx::Sfx;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Menu,
    Map,
    Choosing,
    Playing,
    Paused,
    Won,
    Lost,
}

pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub fn f(&mut self, a: f32, b: f32) -> f32 {
        let v = (self.next_u64() >> 11) as f32 / (1u64 << 53) as f32;
        a + (b - a) * v
    }
    pub fn range(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SunState {
    Falling,
    Idle,
    Collect,
}

pub struct Sun {
    pub x: f32,
    pub y: f32,
    pub ty: f32,
    pub value: i32,
    pub t: f32,
    pub life: f32,
    pub state: SunState,
}

pub struct Plant {
    pub kind: PlantKind,
    pub row: usize,
    pub col: usize,
    pub hp: f32,
    pub max_hp: f32,
    pub t: f32,
    pub timer: f32,
    pub fuse: f32,
    pub flash: f32,
    pub pop: f32,
    pub anim: f32,
}

pub struct Zombie {
    pub kind: ZombieKind,
    pub row: usize,
    pub x: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub t: f32,
    pub slow: f32,
    pub eating: bool,
    pub eat_t: f32,
    pub dying: bool,
    pub death_t: f32,
    pub flash: f32,
    pub groan: f32,
    pub hit_flash: f32,
    pub shield: f32,
    pub vaulted: bool,
    pub diverted: bool,
    pub phase: u8,
}

pub struct Pea {
    pub x: f32,
    pub y: f32,
    pub row: usize,
    pub dmg: f32,
    pub slow: bool,
    pub vx: f32,
    pub t: f32,
    pub pk: u8,
}

#[derive(Clone)]
pub enum PKind {
    Dot,
    Ring,
    Leaf,
    Text(String),
}// 兼容注释：粒子种类

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max: f32,
    pub size: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub gravity: f32,
    pub spin: f32,
    pub rot: f32,
    pub kind: PKind,
}

pub struct Mower {
    pub row: usize,
    pub x: f32,
    pub running: bool,
    pub used: bool,
}

pub struct Pending {
    pub at: f32,
    pub kind: ZombieKind,
    pub row: usize,
}

pub struct Game {
    pub state: State,
    pub time: f32,
    pub elapsed: f32,
    pub mouse: (f32, f32),
    pub sun: i32,
    pub sun_pop: f32,
    pub cooldowns: [f32; 49],
    pub seeds: Vec<PlantKind>,
    pub selected: Option<PlantKind>,
    pub shovel: bool,
    pub plants: Vec<Plant>,
    pub zombies: Vec<Zombie>,
    pub peas: Vec<Pea>,
    pub suns: Vec<Sun>,
    pub particles: Vec<Particle>,
    pub mowers: Vec<Mower>,
    pub pending: Vec<Pending>,
    pub wave: u32,
    pub next_wave_at: f32,
    pub banner: Option<(String, f32)>,
    pub toast: Option<(String, f32, bool)>,
    pub shake: f32,
    pub kills: u32,
    pub planted: u32,
    pub sun_timer: f32,
    pub win_delay: f32,
    pub sfx: Sfx,
    pub muted: bool,
    pub rng: Rng,
    pub flash_screen: f32,
    pub debug: bool,
    pub level: usize,
    pub map_page: usize,
    pub cfg: LevelCfg,
    pub last_planted: Option<PlantKind>,
}

fn part(
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    size: f32,
    color: (u8, u8, u8),
    gravity: f32,
    kind: PKind,
) -> Particle {
    Particle {
        x,
        y,
        vx,
        vy,
        life,
        max: life,
        size,
        r: color.0,
        g: color.1,
        b: color.2,
        gravity,
        spin: 0.0,
        rot: 0.0,
        kind,
    }
}

impl Game {
    pub fn rows(&self) -> usize { rows_for(self.cfg.terrain) }
    pub fn cell_h(&self) -> f32 { cell_h_for(self.cfg.terrain) }
    pub fn cell_center(&self, row: usize, col: usize) -> (f32, f32) { crate::defs::cell_center(row, col, self.cell_h()) }
    pub fn row_ground(&self, row: usize) -> f32 { crate::defs::row_ground(row, self.cell_h()) }
    pub fn new() -> Self {
        Self {
            state: State::Menu,
            time: 0.0,
            elapsed: 0.0,
            mouse: (0.0, 0.0),
            sun: 150,
            sun_pop: 0.0,
            cooldowns: [0.0; 49],
            seeds: Vec::new(),
            selected: None,
            shovel: false,
            plants: Vec::new(),
            zombies: Vec::new(),
            peas: Vec::new(),
            suns: Vec::new(),
            particles: Vec::new(),
            mowers: Vec::new(),
            pending: Vec::new(),
            wave: 0,
            next_wave_at: FIRST_WAVE_AT,
            banner: None,
            toast: None,
            shake: 0.0,
            kills: 0,
            planted: 0,
            sun_timer: 6.0,
            win_delay: 0.0,
            sfx: Sfx::new(),
            muted: false,
            rng: Rng::new(0x9E37_79B9_7F4A_7C15),
            flash_screen: 0.0,
            debug: false,
            level: 0,
            map_page: 0,
            cfg: level_cfg(0),
            last_planted: None,
        }
    }

    pub fn enter_choose(&mut self, level: usize) {
        self.level = level;
        self.cfg = level_cfg(level);
        self.state = State::Choosing;
        self.seeds.clear();
        self.selected = None;
        self.shovel = false;
        self.banner = None;
        self.toast = None;
        self.sfx.click();
        self.sfx.music(true);
    }

    pub fn toggle_seed(&mut self, kind: PlantKind) {
        if let Some(pos) = self.seeds.iter().position(|k| *k == kind) {
            self.seeds.remove(pos);
            self.sfx.click();
            return;
        }
        if self.seeds.len() >= self.cfg.slots {
            self.toast = Some((format!("最多只能带 {} 种植物", self.cfg.slots), 1.4, true));
            return;
        }
        self.seeds.push(kind);
        self.sfx.click();
    }

    pub fn start(&mut self) {
        if self.seeds.is_empty() {
            for k in self.cfg.plants.iter().take(self.cfg.slots) {
                self.seeds.push(*k);
            }
        }
        self.sfx.click();
        self.sfx.music(true);
        self.state = State::Playing;
        self.cfg = level_cfg(self.level);
        self.elapsed = 0.0;
        self.sun = self.cfg.sun;
        self.sun_pop = 0.0;
        self.cooldowns = [0.0; 49];
        self.last_planted = None;
        self.selected = None;
        self.shovel = false;
        self.plants.clear();
        self.zombies.clear();
        self.peas.clear();
        self.suns.clear();
        self.particles.clear();
        self.pending.clear();
        self.mowers.clear();
        for r in 0..self.rows() {
            self.mowers.push(Mower {
                row: r,
                x: MOWER_X,
                running: false,
                used: false,
            });
        }
        self.wave = 0;
        self.next_wave_at = self.cfg.first_wave;
        self.banner = Some(("准备迎战：点卡片选植物，再点草坪种下".to_string(), 3.2));
        self.toast = None;
        self.shake = 0.0;
        self.win_delay = 0.0;
        self.kills = 0;
        self.planted = 0;
        self.sun_timer = 4.5;
        self.rng = Rng::new(
            (js_sys::Date::now() as u64) ^ 0x51_7c_c1_b7_2d_12_9f_3d,
        );
        self.sfx.resume();
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
        if self.state != State::Playing {
            self.update_particles(dt);
            return;
        }
        self.elapsed += dt;
        for c in self.cooldowns.iter_mut() {
            if *c > 0.0 {
                *c = (*c - dt).max(0.0);
            }
        }
        if self.sun_pop > 0.0 {
            self.sun_pop = (self.sun_pop - dt * 1.6).max(0.0);
        }
        if self.shake > 0.0 {
            self.shake = (self.shake - dt * 42.0).max(0.0);
        }
        if self.flash_screen > 0.0 {
            self.flash_screen = (self.flash_screen - dt * 2.4).max(0.0);
        }
        if let Some((_, ref mut t)) = self.banner {
            *t -= dt;
            if *t <= 0.0 {
                self.banner = None;
            }
        }
        if let Some((_, ref mut t, _)) = self.toast {
            *t -= dt;
            if *t <= 0.0 {
                self.toast = None;
            }
        }
        self.update_suns(dt);
        self.update_plants(dt);
        self.update_peas(dt);
        self.update_zombies(dt);
        self.update_mowers(dt);
        self.update_particles(dt);
        self.update_waves(dt);
        self.check_end(dt);
    }

    fn update_suns(&mut self, dt: f32) {
        self.sun_timer -= dt;
        if self.sun_timer <= 0.0 && self.level != 2 {
            self.sun_timer = self.rng.f(4.25, 7.0);
            let x = self.rng.f(GRID_X + 30.0, GRID_X + 9.0 * CELL_W - 30.0);
            let ty = self.rng.f(GRID_Y + 70.0, LAWN_BOTTOM - 40.0);
            self.suns.push(Sun {
                x,
                y: -26.0,
                ty,
                value: 25,
                t: 0.0,
                life: 16.0,
                state: SunState::Falling,
            });
        }

        let mut gained = 0;
        let mut collected = false;
        for i in (0..self.suns.len()).rev() {
            let s = &mut self.suns[i];
            s.t += dt;
            match s.state {
                SunState::Falling => {
                    s.y += 58.0 * dt;
                    if s.y >= s.ty {
                        s.y = s.ty;
                        s.state = SunState::Idle;
                        s.life = 16.0;
                    }
                }
                SunState::Idle => {
                    s.life -= dt;
                    if s.life <= 0.0 {
                        self.suns.remove(i);
                    }
                }
                SunState::Collect => {
                    let tx = 80.0;
                    let ty = 52.0;
                    let dx = tx - s.x;
                    let dy = ty - s.y;
                    let k = (dt * 9.0).min(1.0);
                    s.x += dx * k;
                    s.y += dy * k;
                    if dx.abs() < 6.0 && dy.abs() < 6.0 {
                        gained += s.value;
                        self.suns.remove(i);
                        collected = true;
                    }
                }
            }
        }
        if gained > 0 {
            self.sun += gained;
            self.sun_pop = 0.7;
            if collected {
                self.sfx.sun();
            }
        }
    }

    fn update_plants(&mut self, dt: f32) {
        let mut explosions: Vec<(f32, f32, usize)> = Vec::new();
        let mut sun_spawns: Vec<(f32, f32)> = Vec::new();
        let mut made_sounds: (bool, bool) = (false, false);
        let mut dead_plants: Vec<usize> = Vec::new();
        let mut row_fires: Vec<usize> = Vec::new();
        let mut squash_hits: Vec<(f32, usize)> = Vec::new();
        let mut mine_hits: Vec<(f32, usize)> = Vec::new();
        let mut spike_ticks: Vec<(f32, usize)> = Vec::new();
        let mut spike_hits: Vec<(f32, usize)> = Vec::new();
        let mut freeze_all = false;
        let mut doom_hits: Vec<(f32, usize)> = Vec::new();
        let mut chomp_hits: Vec<(f32, usize)> = Vec::new();
        let mut gloom_hits: Vec<(f32, usize)> = Vec::new();
        let mut blover_push = false;
        let mut magnet_pull = false;
        let h = self.cell_h();
        let rows = self.rows();
        {
            let plants = &mut self.plants;
            let zombies = &self.zombies;
            let peas = &mut self.peas;
            let particles = &mut self.particles;
            for i in 0..plants.len() {
                let p = &mut plants[i];
                p.t += dt;
                if p.flash > 0.0 {
                    p.flash = (p.flash - dt * 4.0).max(0.0);
                }
                if p.pop < 1.0 {
                    p.pop = (p.pop + dt * 3.5).min(1.0);
                }
                let (cx, _cy) = cell_center(p.row, p.col, h);
                let gy = row_ground(p.row, h);
                match p.kind {
                    PlantKind::LilyPad | PlantKind::FlowerPot => {}
                    PlantKind::Sunflower => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 25.0;
                            sun_spawns.push((cx + 26.0, gy - 86.0));
                        }
                    }
                    PlantKind::Threepeater => {
                        p.timer -= dt;
                        let has_target = zombies
                            .iter()
                            .any(|z| z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 1.5;
                            for dr in -1i32..=1 {
                                let r = (p.row as i32 + dr).clamp(0, rows as i32 - 1) as usize;
                                peas.push(Pea {
                                    x: cx + 34.0,
                                    y: row_ground(r, h) - 52.0,
                                    row: r,
                                    dmg: 20.0,
                                    slow: false,
                                    vx: 333.0,
                                    t: 0.6,
                                    pk: 0,
                                });
                            }
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::Peashooter | PlantKind::SnowPea | PlantKind::Repeater => {
                        p.timer -= dt;
                        let has_target = zombies
                            .iter()
                            .any(|z| z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 1.5;
                            let px = cx + 34.0;
                            let py = gy - 52.0;
                            peas.push(Pea {
                                x: px,
                                y: py,
                                row: p.row,
                                dmg: 20.0,
                                slow: p.kind == PlantKind::SnowPea,
                                vx: 333.0,
                                t: 0.0,
                                pk: if p.kind == PlantKind::SnowPea { 1 } else { 0 },
                            });
                            if p.kind == PlantKind::Repeater {
                                peas.push(Pea {
                                    x: px - 4.0,
                                    y: py + 3.0,
                                    row: p.row,
                                    dmg: 20.0,
                                    slow: false,
                                    vx: 333.0,
                                    t: -0.14,
                                    pk: 0,
                                });
                            }
                            if p.kind == PlantKind::SnowPea {
                                made_sounds.1 = true;
                            } else {
                                made_sounds.0 = true;
                            }
                            particles.push(part(
                                px + 6.0,
                                py,
                                40.0,
                                -rng_r(-6.0, 6.0),
                                0.18,
                                3.0,
                                (210, 255, 180),
                                -20.0,
                                PKind::Dot,
                            ));
                        }
                    }
                    PlantKind::Cherry => {
                        p.fuse -= dt;
                        if p.fuse <= 0.0 {
                            explosions.push((cx, gy - 30.0, p.row));
                            dead_plants.push(i);
                        }
                    }
                    PlantKind::PotatoMine => {
                        p.timer -= dt;
                        if p.timer <= 0.0
                            && zombies.iter().any(|z| {
                                z.row == p.row
                                    && !z.dying
                                    && (z.x - cx).abs() < CELL_W * 0.75
                            })
                        {
                            mine_hits.push((cx, p.row));
                            dead_plants.push(i);
                        }
                    }
                    PlantKind::Jalapeno => {
                        p.fuse -= dt;
                        if p.fuse <= 0.0 {
                            row_fires.push(p.row);
                            dead_plants.push(i);
                        }
                    }
                    PlantKind::Squash => {
                        let near = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && z.x > cx - 30.0 && z.x < cx + 150.0
                        });
                        if p.timer < 0.0 && near {
                            p.timer = 0.45;
                        }
                        if p.timer >= 0.0 {
                            p.timer -= dt;
                            if p.timer <= 0.0 {
                                squash_hits.push((cx + 60.0, p.row));
                                dead_plants.push(i);
                            }
                        }
                    }
                    PlantKind::Cabbage => {
                        p.timer -= dt;
                        let has_target = zombies
                            .iter()
                            .any(|z| z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 3.0;
                            p.anim = 0.20;
                            peas.push(Pea {
                                x: cx + 24.0,
                                y: gy - 58.0,
                                row: p.row,
                                dmg: 40.0,
                                slow: false,
                                vx: 360.0,
                                t: 9.0,
                                pk: 2,
                            });
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::Starfruit => {
                        p.timer -= dt;
                        let has_target = zombies.iter().any(|z| !z.dying && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 1.5;
                            p.anim = 0.16;
                            for dr in -1i32..=1 {
                                let r = (p.row as i32 + dr).clamp(0, rows as i32 - 1) as usize;
                                peas.push(Pea {
                                    x: cx,
                                    y: row_ground(r, h) - 48.0,
                                    row: r,
                                    dmg: 20.0,
                                    slow: false,
                                    vx: 380.0,
                                    t: 9.0,
                                    pk: 3,
                                });
                            }
                            peas.push(Pea {
                                x: cx,
                                y: gy - 48.0,
                                row: p.row,
                                dmg: 20.0,
                                slow: false,
                                vx: -300.0,
                                t: 9.0,
                                pk: 3,
                            });
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::Spikeweed => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 0.5;
                            spike_ticks.push((cx, p.row));
                        }
                    }
                    PlantKind::PuffShroom
                    | PlantKind::FumeShroom
                    | PlantKind::SeaShroom
                    | PlantKind::Cactus => {
                        p.timer -= dt;
                        let range = if p.kind == PlantKind::FumeShroom { 260.0 } else { 420.0 };
                        let has_target = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < cx + range
                        });
                        if has_target && p.timer <= 0.0 {
                            p.timer = if p.kind == PlantKind::FumeShroom { 1.2 } else { 1.5 };
                            p.anim = 0.18;
                            peas.push(Pea {
                                x: cx + 30.0,
                                y: gy - 48.0,
                                row: p.row,
                                dmg: if p.kind == PlantKind::FumeShroom { 26.0 } else { 18.0 },
                                slow: false,
                                vx: if p.kind == PlantKind::FumeShroom { 280.0 } else { 320.0 },
                                t: 0.0,
                                pk: 0,
                            });
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::ScaredyShroom => {
                        p.timer -= dt;
                        let nearby = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && (z.x - cx).abs() < 150.0
                        });
                        let target = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0
                        });
                        if !nearby && target && p.timer <= 0.0 {
                            p.timer = 1.5;
                            p.anim = 0.18;
                            peas.push(Pea {
                                x: cx + 28.0,
                                y: gy - 50.0,
                                row: p.row,
                                dmg: 18.0,
                                slow: false,
                                vx: 320.0,
                                t: 0.0,
                                pk: 0,
                            });
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::SplitPea => {
                        p.timer -= dt;
                        let forward = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0
                        });
                        let backward = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && z.x < cx - 6.0
                        });
                        if (forward || backward) && p.timer <= 0.0 {
                            p.timer = 1.5;
                            p.anim = 0.18;
                            if forward {
                                peas.push(Pea { x: cx + 34.0, y: gy - 52.0, row: p.row, dmg: 20.0, slow: false, vx: 330.0, t: 0.0, pk: 0 });
                            }
                            if backward {
                                peas.push(Pea { x: cx - 30.0, y: gy - 46.0, row: p.row, dmg: 20.0, slow: false, vx: -300.0, t: 0.0, pk: 0 });
                            }
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::GatlingPea => {
                        p.timer -= dt;
                        let has_target = zombies.iter().any(|z| {
                            z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0
                        });
                        if has_target && p.timer <= 0.0 {
                            p.timer = 1.6;
                            p.anim = 0.20;
                            for k in 0..4 {
                                peas.push(Pea {
                                    x: cx + 34.0,
                                    y: gy - 54.0 + k as f32 * 1.5,
                                    row: p.row,
                                    dmg: 20.0,
                                    slow: false,
                                    vx: 350.0,
                                    t: -(k as f32) * 0.12,
                                    pk: 0,
                                });
                            }
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::Cattail => {
                        p.timer -= dt;
                        let has_target = zombies.iter().any(|z| !z.dying && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 1.4;
                            p.anim = 0.16;
                            if let Some(z) = zombies.iter().filter(|z| !z.dying && z.x < W + 20.0).min_by(|a, b| (a.x - cx).abs().partial_cmp(&(b.x - cx).abs()).unwrap()) {
                                let rr = z.row;
                                peas.push(Pea { x: cx + 22.0, y: row_ground(rr, h) - 54.0, row: rr, dmg: 30.0, slow: false, vx: 380.0, t: 0.0, pk: 0 });
                            }
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::KernelPult => {
                        p.timer -= dt;
                        let has_target = zombies.iter().any(|z| z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 3.0;
                            p.anim = 0.20;
                            peas.push(Pea { x: cx + 24.0, y: gy - 66.0, row: p.row, dmg: 40.0, slow: false, vx: 360.0, t: 9.0, pk: 2 });
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::MelonPult | PlantKind::WinterMelon => {
                        p.timer -= dt;
                        let has_target = zombies.iter().any(|z| z.row == p.row && !z.dying && z.x > cx + 6.0 && z.x < W + 30.0);
                        if has_target && p.timer <= 0.0 {
                            p.timer = 3.5;
                            p.anim = 0.20;
                            peas.push(Pea {
                                x: cx + 24.0,
                                y: gy - 70.0,
                                row: p.row,
                                dmg: if p.kind == PlantKind::WinterMelon { 65.0 } else { 90.0 },
                                slow: p.kind == PlantKind::WinterMelon,
                                vx: 340.0,
                                t: 9.0,
                                pk: 2,
                            });
                            made_sounds.0 = true;
                        }
                    }
                    PlantKind::GloomShroom => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 2.2;
                            gloom_hits.push((cx, p.row));
                        }
                    }
                    PlantKind::SunShroom => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 22.0;
                            sun_spawns.push((cx + 18.0, gy - 56.0));
                        }
                    }
                    PlantKind::TwinSunflower => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 25.0;
                            sun_spawns.push((cx - 18.0, gy - 78.0));
                            sun_spawns.push((cx + 26.0, gy - 82.0));
                        }
                    }
                    PlantKind::Marigold | PlantKind::GoldMagnet => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = if p.kind == PlantKind::GoldMagnet { 12.0 } else { 18.0 };
                            sun_spawns.push((cx + 18.0, gy - 64.0));
                        }
                    }
                    PlantKind::Chomper => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            let target = zombies.iter().any(|z| z.row == p.row && !z.dying && (z.x - cx).abs() < 96.0);
                            if target {
                                chomp_hits.push((cx, p.row));
                                p.timer = 8.0;
                                p.anim = 0.35;
                            }
                        }
                    }
                    PlantKind::IceShroom => {
                        p.fuse -= dt;
                        if p.fuse <= 0.0 {
                            freeze_all = true;
                            dead_plants.push(i);
                        }
                    }
                    PlantKind::DoomShroom => {
                        p.fuse -= dt;
                        if p.fuse <= 0.0 {
                            doom_hits.push((cx, p.row));
                            dead_plants.push(i);
                        }
                    }
                    PlantKind::TangleKelp => {
                        p.fuse -= dt;
                        if p.fuse <= 0.0 {
                            chomp_hits.push((cx, p.row));
                            dead_plants.push(i);
                        }
                    }
                    PlantKind::Spikerock => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 0.4;
                            spike_hits.push((cx, p.row));
                        }
                    }
                    PlantKind::MagnetShroom => {
                        p.timer -= dt;
                        if p.timer <= 0.0 && zombies.iter().any(|z| z.row == p.row && !z.dying && z.shield > 0.0) {
                            p.timer = 7.0;
                            magnet_pull = true;
                        }
                    }
                    PlantKind::Blover => {
                        p.timer -= dt;
                        if p.timer <= 0.0 {
                            p.timer = 9.0;
                            blover_push = true;
                        }
                    }
                    PlantKind::CobCannon => {
                        p.timer -= dt;
                        if p.timer <= 0.0 && zombies.iter().any(|z| !z.dying && z.x < W + 10.0) {
                            p.timer = 12.0;
                            doom_hits.push((cx + 180.0, p.row));
                        }
                    }
                    PlantKind::GraveBuster
                    | PlantKind::HypnoShroom
                    | PlantKind::Torchwood
                    | PlantKind::TallNut
                    | PlantKind::Plantern
                    | PlantKind::Pumpkin
                    | PlantKind::CoffeeBean
                    | PlantKind::UmbrellaLeaf
                    | PlantKind::Imitater => {}
                    PlantKind::Garlic => {}
                    PlantKind::WallNut => {}
                }
            }
        }
        if made_sounds.0 {
            self.sfx.shoot();
        }
        if made_sounds.1 {
            self.sfx.ice_shoot();
        }
        for (x, y) in sun_spawns {
            self.suns.push(Sun {
                x,
                y,
                ty: y,
                value: 25,
                t: 0.0,
                life: 10.0,
                state: SunState::Idle,
            });
        }
        for (ex, ey, row) in explosions {
            let mut hit_any = false;
            for z in self.zombies.iter_mut() {
                if z.dying {
                    continue;
                }
                if (z.row as i32 - row as i32).abs() <= 1
                    && (z.x - ex).abs() < CELL_W * 1.5
                    && z.hp > 0.0
                {
                    hurt_zombie(z, 1800.0);
                    hit_any = true;
                }
            }
            for k in 0..22 {
                let a = k as f32 / 22.0 * std::f32::consts::TAU;
                self.particles.push(part(
                    ex,
                    ey,
                    a.cos() * 120.0,
                    a.sin() * 120.0,
                    0.5,
                    8.0,
                    (255, 190, 60),
                    0.0,
                    PKind::Dot,
                ));
            }
            for k in 0..8 {
                self.particles.push(part(
                    ex,
                    ey,
                    self.rng.f(-30.0, 30.0),
                    self.rng.f(-40.0, 10.0),
                    0.9,
                    30.0 - k as f32 * 2.0,
                    (255, 240, 180),
                    0.0,
                    PKind::Ring,
                ));
            }
            self.shake = 15.0;
            self.flash_screen = 0.35;
            self.sfx.explode();
            let _ = hit_any;
        }
        for row in row_fires {
            for z in self.zombies.iter_mut() {
                if z.row == row && !z.dying {
                    hurt_zombie(z, 1800.0);
                }
            }
            for k in 0..26 {
                let x = GRID_X + (k as f32 / 26.0) * COLS as f32 * CELL_W;
                self.particles.push(part(
                    x,
                    row_ground(row, h) - 30.0,
                    0.0,
                    -60.0,
                    0.7,
                    18.0,
                    (255, 150, 40),
                    0.0,
                    PKind::Dot,
                ));
            }
            self.shake = 12.0;
            self.flash_screen = 0.3;
            self.sfx.explode();
        }
        for (x, row) in mine_hits {
            let gy = row_ground(row, h) - 8.0;
            for z in self.zombies.iter_mut() {
                if z.row == row && !z.dying && (z.x - x).abs() < 96.0 {
                    hurt_zombie(z, 9000.0);
                }
            }
            // 土豆雷不是“悄悄消失”：中心火球 + 向外飞散的土块/火星，并伴随屏幕震动。
            self.shake = self.shake.max(13.0);
            self.flash_screen = self.flash_screen.max(0.16);
            for k in 0..18 {
                let a = k as f32 / 18.0 * std::f32::consts::TAU;
                self.particles.push(part(
                    x + a.cos() * 8.0,
                    gy + a.sin() * 5.0,
                    a.cos() * 150.0,
                    a.sin() * 95.0 - 35.0,
                    0.48,
                    if k % 3 == 0 { 11.0 } else { 7.0 },
                    if k % 2 == 0 { (255, 235, 140) } else { (255, 145, 35) },
                    70.0,
                    PKind::Dot,
                ));
            }
            for k in 0..10 {
                let a = k as f32 / 10.0 * std::f32::consts::TAU + 0.3;
                self.particles.push(part(
                    x + a.cos() * 10.0,
                    gy,
                    a.cos() * 115.0,
                    -60.0 + a.sin() * 55.0,
                    0.62,
                    8.0,
                    (132, 82, 42),
                    120.0,
                    PKind::Leaf,
                ));
            }
            self.particles.push(part(x, gy, 0.0, 0.0, 0.85, 26.0, (255, 190, 60), 0.0, PKind::Ring));
            // 残留烟尘，保证爆炸不是一闪而过。
            for k in 0..9 {
                let a = k as f32 / 9.0 * std::f32::consts::TAU;
                self.particles.push(part(
                    x + a.cos() * 12.0,
                    gy - 6.0,
                    a.cos() * 18.0,
                    -24.0 + a.sin() * 10.0,
                    1.15,
                    12.0 + (k % 3) as f32 * 3.0,
                    (105, 92, 74),
                    -18.0,
                    PKind::Dot,
                ));
            }
            self.sfx.explode();
        }
        for (x, row) in spike_ticks {
            let mut hit = false;
            for z in self.zombies.iter_mut() {
                if z.row == row && !z.dying && (z.x - x).abs() < 40.0 {
                    hurt_zombie(z, 20.0);
                    hit = true;
                }
            }
            if hit {
                for k in 0..4 {
                    let a = self.rng.f(0.0, std::f32::consts::TAU);
                    self.particles.push(part(
                        x + self.rng.f(-28.0, 28.0),
                        row_ground(row, h) - 6.0,
                        a.cos() * 35.0,
                        -40.0 + k as f32 * 8.0,
                        0.28,
                        4.0,
                        (190, 230, 120),
                        40.0,
                        PKind::Dot,
                    ));
                }
            }
        }
        for (x, row) in squash_hits {
            for z in self.zombies.iter_mut() {
                if z.row == row && !z.dying && (z.x - x).abs() < 90.0 {
                    hurt_zombie(z, 1800.0);
                }
            }
            self.shake = 10.0;
            self.sfx.explode();
        }
        if freeze_all {
            for z in self.zombies.iter_mut() {
                if !z.dying {
                    z.slow = z.slow.max(10.0);
                    z.flash = 1.0;
                }
            }
            self.flash_screen = 0.45;
            self.shake = 8.0;
            self.sfx.explode();
            for k in 0..40 {
                self.particles.push(part(
                    self.rng.f(0.0, W),
                    self.rng.f(120.0, 620.0),
                    self.rng.f(-20.0, 20.0),
                    self.rng.f(-60.0, 20.0),
                    0.9,
                    self.rng.f(3.0, 8.0),
                    (150, 230, 255),
                    10.0,
                    PKind::Dot,
                ));
            }
        }
        for (x, row) in doom_hits {
            for z in self.zombies.iter_mut() {
                if !z.dying && (z.x - x).abs() < 260.0 {
                    hurt_zombie(z, 4000.0);
                }
            }
            for k in 0..34 {
                let a = k as f32 / 34.0 * std::f32::consts::TAU;
                self.particles.push(part(
                    x,
                    row_ground(row, h) - 40.0,
                    a.cos() * 220.0,
                    a.sin() * 130.0 - 40.0,
                    0.8,
                    14.0,
                    (255, 150, 45),
                    20.0,
                    PKind::Dot,
                ));
            }
            self.particles.push(part(x, row_ground(row, h) - 30.0, 0.0, 0.0, 1.2, 120.0, (255, 220, 120), 0.0, PKind::Ring));
            self.shake = 24.0;
            self.flash_screen = 0.6;
            self.sfx.explode();
        }
        for (x, row) in chomp_hits {
            let mut ate = false;
            for z in self.zombies.iter_mut() {
                if !z.dying && z.row == row && (z.x - x).abs() < 110.0 && z.hp < 9000.0 {
                    hurt_zombie(z, 9000.0);
                    ate = true;
                    break;
                }
            }
            if ate {
                for k in 0..10 {
                    let a = -std::f32::consts::PI * 0.5 + k as f32 * 0.6;
                    self.particles.push(part(
                        x + 20.0,
                        row_ground(row, h) - 70.0,
                        a.cos() * 70.0,
                        a.sin() * 70.0,
                        0.45,
                        7.0,
                        (160, 235, 90),
                        20.0,
                        PKind::Leaf,
                    ));
                }
                self.sfx.explode();
            }
        }
        for (x, row) in gloom_hits {
            for z in self.zombies.iter_mut() {
                if !z.dying && (z.row as i32 - row as i32).abs() <= 1 && (z.x - x).abs() < 150.0 {
                    hurt_zombie(z, 26.0);
                }
            }
            for k in 0..12 {
                let a = k as f32 / 12.0 * std::f32::consts::TAU;
                self.particles.push(part(x, row_ground(row, h) - 55.0, a.cos() * 90.0, a.sin() * 60.0, 0.45, 8.0, (155, 85, 210), 10.0, PKind::Dot));
            }
        }
        for (x, row) in spike_hits {
            let mut hit = false;
            for z in self.zombies.iter_mut() {
                if z.row == row && !z.dying && (z.x - x).abs() < 48.0 {
                    hurt_zombie(z, 45.0);
                    hit = true;
                }
            }
            if hit {
                for k in 0..5 {
                    self.particles.push(part(x + self.rng.f(-26.0, 26.0), row_ground(row, h) - 8.0, self.rng.f(-30.0, 30.0), -55.0, 0.32, 5.0, (215, 240, 170), 30.0, PKind::Dot));
                }
            }
        }
        if magnet_pull {
            for z in self.zombies.iter_mut() {
                if !z.dying && z.shield > 0.0 {
                    z.shield = 0.0;
                }
            }
        }
        if blover_push {
            for z in self.zombies.iter_mut() {
                if !z.dying && z.x < W - 10.0 {
                    z.x = (z.x + 90.0).min(W - 10.0);
                }
            }
            for k in 0..20 {
                self.particles.push(part(self.rng.f(200.0, W), self.rng.f(160.0, 640.0), 110.0, -20.0, 0.6, 6.0, (210, 245, 190), 5.0, PKind::Leaf));
            }
        }
        for i in dead_plants {
            if i < self.plants.len() {
                self.plants.remove(i);
            }
        }
        // 植物被打死
        let dead: Vec<usize> = self
            .plants
            .iter()
            .enumerate()
            .filter(|(_, p)| p.hp <= 0.0)
            .map(|(i, _)| i)
            .collect();
        if !dead.is_empty() {
            for i in dead.iter().rev() {
                let p = self.plants.remove(*i);
                let (x, y) = cell_center(p.row, p.col, h);
                for _ in 0..8 {
                    self.particles.push(part(
                        x + self.rng.f(-24.0, 24.0),
                        y + self.rng.f(-20.0, 20.0),
                        self.rng.f(-20.0, 20.0),
                        self.rng.f(-80.0, -20.0),
                        0.7,
                        self.rng.f(3.0, 7.0),
                        (86, 160, 48),
                        90.0,
                        PKind::Leaf,
                    ));
                }
            }
        }
    }

    fn update_peas(&mut self, dt: f32) {
        let torch: Vec<(f32, usize)> = self.plants.iter()
            .filter(|p| p.kind == PlantKind::Torchwood)
            .map(|p| { let (cx, _) = cell_center(p.row, p.col, self.cell_h()); (cx, p.row) })
            .collect();
        let peas = &mut self.peas;
        let zombies = &mut self.zombies;
        let particles = &mut self.particles;
        let kills = &mut self.kills;
        let mut hit_sound = false;
        for i in (0..peas.len()).rev() {
            let p = &mut peas[i];
            if p.t < 0.0 {
                p.t += dt;
                continue;
            }
            p.t += dt;
            p.x += p.vx * dt;
            if p.pk == 0 && p.dmg < 40.0 {
                if torch.iter().any(|(tx, tr)| *tr == p.row && (p.x - *tx).abs() < 24.0) {
                    p.dmg = 40.0;
                }
            }
            if p.x > W + 20.0 || p.x < -30.0 {
                peas.remove(i);
                continue;
            }
            let mut hit = None;
            for (j, z) in zombies.iter_mut().enumerate() {
                if z.dying || z.row != p.row {
                    continue;
                }
                if (p.x - z.x).abs() < 22.0 && p.x > z.x - 34.0 {
                    if p.pk == 2 {
                        // 抛射物越过铁门/梯子，直接打本体
                        z.hp -= p.dmg;
                        z.flash = 1.0;
                    } else {
                        hurt_zombie(z, p.dmg);
                    }
                    if p.slow {
                        z.slow = z.slow.max(6.0);
                    }
                    if z.hp <= 0.0 {
                        z.dying = true;
                        z.death_t = 0.0;
                        *kills += 1;
                    }
                    hit = Some(j);
                    break;
                }
            }
            if hit.is_some() {
                let col = if p.slow { (150, 225, 245) } else { (150, 235, 90) };
                for _ in 0..5 {
                    particles.push(part(
                        p.x,
                        p.y,
                        rng_r(-40.0, 40.0),
                        rng_r(-40.0, 40.0),
                        0.3,
                        4.0,
                        col,
                        40.0,
                        PKind::Dot,
                    ));
                }
                peas.remove(i);
                hit_sound = true;
            }
        }
        let _ = hit_sound;
    }

    fn update_zombies(&mut self, dt: f32) {
        let h = self.cell_h();
        let rows = self.rows();
        let plants = &mut self.plants;
        let zombies = &mut self.zombies;
        let particles = &mut self.particles;
        let kills = &mut self.kills;
        let mut bite_sound = false;
        let mut groan = false;
        let mut spawns: Vec<(ZombieKind, usize, f32)> = Vec::new();

        for i in 0..zombies.len() {
            let z = &mut zombies[i];
            z.t += dt;
            if z.dying {
                z.death_t += dt;
                continue;
            }
            if z.flash > 0.0 {
                z.flash = (z.flash - dt * 3.5).max(0.0);
            }
            let slow = z.slow > 0.0;
            if slow {
                z.slow -= dt;
            }
            z.groan -= dt;
            if z.groan <= 0.0 && z.x < W + 20.0 {
                z.groan = 4.0 + (z.x % 3.0);
                groan = true;
            }
            let mouth = z.x - 22.0;
            let mut target: Option<usize> = None;
            let mut base_target: Option<usize> = None;
            for (pi, p) in plants.iter().enumerate() {
                if p.row != z.row || p.hp <= 0.0 || p.kind == PlantKind::Spikeweed {
                    continue;
                }
                let px = GRID_X + p.col as f32 * CELL_W + 12.0;
                if mouth <= px || mouth >= px + CELL_W - 12.0 {
                    continue;
                }
                if p.kind.is_base() {
                    base_target.get_or_insert(pi);
                } else {
                    target = Some(pi);
                    break;
                }
            }
            let pass_over = match z.kind {
                ZombieKind::Balloon => z.hp > z.max_hp * 0.55,
                ZombieKind::Pogo => z.x > 250.0,
                ZombieKind::Digger => z.x > 300.0,
                _ => false,
            };
            if pass_over {
                target = None;
            }
            if target.is_none() {
                target = base_target;
            }
            if let Some(pi) = target {
                if plants[pi].kind == PlantKind::Garlic && !z.diverted {
                    z.diverted = true;
                    let nr = if z.row + 1 < rows { z.row + 1 } else { z.row.saturating_sub(1) };
                    z.row = nr;
                    z.x += 24.0;
                    z.eating = false;
                    plants[pi].flash = 1.0;
                    continue;
                }
                if z.kind == ZombieKind::PoleVault && !z.vaulted {
                    z.vaulted = true;
                    let px = GRID_X + plants[pi].col as f32 * CELL_W + 12.0;
                    z.x = px - 44.0;
                    z.speed = self.rng.f(23.0, 37.0);
                    z.eating = false;
                    continue;
                }
                z.eating = true;
                z.eat_t += dt;
                let crush = match z.kind {
                    ZombieKind::Gargantuar => 1000.0,
                    ZombieKind::Zomboni => 600.0,
                    ZombieKind::Bobsled => 450.0,
                    _ => 55.0,
                };
                let dmg = crush * dt * if slow { 0.5 } else { 1.0 };
                plants[pi].hp -= dmg;
                plants[pi].flash = 1.0;
                if (z.eat_t * 2.0) as i32 % 2 == 0 && z.eat_t < 0.2 {
                    bite_sound = true;
                }
            } else {
                z.eating = false;
                let spd = z.speed * if slow { 0.5 } else { 1.0 };
                z.x -= spd * dt;
            }
            // 僵尸专属机制：报纸狂暴、玩偶匣爆炸、矿工出土、蹦极偷植物、巨人扔小鬼、僵王分阶段。
            match z.kind {
                ZombieKind::Newspaper if z.phase == 0 && z.hp <= z.max_hp * 0.5 => {
                    z.phase = 1;
                    z.speed *= 1.6;
                    z.flash = 1.0;
                }
                ZombieKind::Digger if z.phase == 0 && z.x <= 300.0 => {
                    z.phase = 1;
                    z.speed = 45.0;
                    z.flash = 0.8;
                }
                ZombieKind::Jackbox if z.phase == 0 && z.x <= 360.0 => {
                    z.phase = 1;
                    let col = (((z.x - GRID_X) / CELL_W).round() as isize).clamp(0, 8) as usize;
                    for p in plants.iter_mut() {
                        if p.row.abs_diff(z.row) <= 1 && p.col.abs_diff(col) <= 1 {
                            p.hp = 0.0;
                            p.flash = 1.0;
                        }
                    }
                    for _ in 0..18 {
                        particles.push(part(
                            z.x, -70.0 + z.row as f32 * h, rng_r(-50.0, 50.0), rng_r(-90.0, 20.0),
                            0.7, 5.0, (250, 160, 50), 160.0, PKind::Dot,
                        ));
                    }
                    z.hp = 0.0;
                }
                ZombieKind::Bungee if z.phase == 0 && z.x <= W - 20.0 => {
                    z.phase = 1;
                    if let Some(pi) = plants.iter().enumerate()
                        .filter(|(_, p)| p.row == z.row && p.hp > 0.0)
                        .max_by_key(|(_, p)| p.col)
                        .map(|(i, _)| i)
                    {
                        plants[pi].hp = 0.0;
                        plants[pi].flash = 1.0;
                    }
                    for _ in 0..8 {
                        particles.push(part(
                            z.x, -90.0, rng_r(-20.0, 20.0), rng_r(-30.0, 30.0),
                            0.6, 4.0, (210, 210, 220), 40.0, PKind::Dot,
                        ));
                    }
                    z.hp = 0.0;
                }
                ZombieKind::Gargantuar if z.phase == 0 && z.hp <= z.max_hp * 0.5 => {
                    z.phase = 1;
                    spawns.push((ZombieKind::Imp, z.row, z.x - 10.0));
                    z.flash = 1.0;
                }
                ZombieKind::Boss if z.phase == 0 && z.hp <= z.max_hp * 2.0 / 3.0 => {
                    z.phase = 1;
                    spawns.push((ZombieKind::Cone, z.row, z.x + 40.0));
                    spawns.push((ZombieKind::Bucket, (z.row + 1) % rows, z.x + 70.0));
                }
                ZombieKind::Boss if z.phase == 1 && z.hp <= z.max_hp / 3.0 => {
                    z.phase = 2;
                    spawns.push((ZombieKind::Football, z.row, z.x + 50.0));
                    z.speed *= 1.3;
                }
                _ => {}
            }
            if z.hp <= 0.0 {
                z.dying = true;
                z.death_t = 0.0;
                *kills += 1;
            }
        }
        for (kind, row, x) in spawns {
            zombies.push(Zombie {
                kind, row, x,
                hp: kind.hp(), max_hp: kind.hp(),
                speed: self.rng.f(kind.speed_range().0, kind.speed_range().1),
                t: 0.0, slow: 0.0, eating: false, eat_t: 0.0,
                dying: false, death_t: 0.0, flash: 0.7,
                groan: 99.0, hit_flash: 0.0, shield: kind.shield(),
                vaulted: false, diverted: false, phase: 0,
            });
        }
        if groan && !bite_sound {
            self.sfx.groan();
        }
        if bite_sound {
            self.sfx.bite();
        }

        // 死亡残骸消失
        let len_before = zombies.len();
        zombies.retain(|z| !(z.dying && z.death_t > 0.9));
        if zombies.len() != len_before {
            // 无需处理
        }

        // 僵尸倒下时掉落零件
        if !zombies.is_empty() {
            for z in zombies.iter() {
                if z.dying && z.death_t < 0.02 {
                    let gy = row_ground(z.row, h);
                    for _ in 0..6 {
                        particles.push(part(
                            z.x + rng_r(-14.0, 14.0),
                            gy - 50.0 + rng_r(-20.0, 20.0),
                            rng_r(-50.0, 50.0),
                            rng_r(-60.0, -10.0),
                            0.55,
                            rng_r(3.0, 5.0),
                            (120, 140, 90),
                            60.0,
                            PKind::Dot,
                        ));
                    }
                }
            }
        }
    }

    fn update_mowers(&mut self, dt: f32) {
        let h = self.cell_h();
        let mowers = &mut self.mowers;
        let zombies = &mut self.zombies;
        let particles = &mut self.particles;
        let kills = &mut self.kills;
        let mut triggered = false;
        for m in mowers.iter_mut() {
            if !m.used {
                let hit = zombies
                    .iter()
                    .any(|z| z.row == m.row && !z.dying && z.x < GRID_X + 26.0);
                if hit {
                    m.used = true;
                    m.running = true;
                    m.x = MOWER_X;
                    triggered = true;
                }
            }
            if m.running {
                m.x += 520.0 * dt;
                let mx = m.x;
                let row = m.row;
                for z in zombies.iter_mut() {
                    if z.row == row && !z.dying && z.x < mx + 24.0 && z.x > mx - 60.0 {
                        z.hp = 0.0;
                        z.dying = true;
                        z.death_t = 0.0;
                        *kills += 1;
                        for _ in 0..7 {
                            particles.push(part(
                                z.x,
                                row_ground(row, h) - 40.0,
                                rng_r(-30.0, 30.0),
                                rng_r(-90.0, -30.0),
                                0.6,
                                5.0,
                                (140, 160, 100),
                                120.0,
                                PKind::Dot,
                            ));
                        }
                    }
                }
                if m.x > W + 60.0 {
                    m.running = false;
                }
            }
        }
        if triggered {
            self.sfx.mower();
            self.shake = 9.0;
        }
    }

    fn update_particles(&mut self, dt: f32) {
        let particles = &mut self.particles;
        particles.retain_mut(|p| {
            p.life -= dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += p.gravity * dt;
            p.rot += p.spin * dt;
            p.life > 0.0
        });
    }

    fn is_flag_wave(&self, w: u32) -> bool {
        self.cfg.flag_wave != 0 && w == self.cfg.flag_wave
    }

    /// 原版 1073 的波次点数：`aWave / 3 + 1`，旗帜波先乘 2.5。
    fn wave_points(&self, w: u32) -> u32 {
        let mut points = (w - 1) / 3 + 1;
        if self.is_flag_wave(w) {
            points = points * 5 / 2;
        }
        points
    }

    /// 在剩余点数内按原版权重挑选僵尸；随波次逐步放开本关僵尸池。
    fn pick_kind(&mut self, w: u32, budget: &mut u32) -> Option<ZombieKind> {
        let full = self.cfg.pool;
        if full.is_empty() {
            return None;
        }
        let waves = self.cfg.waves.max(1) as usize;
        let unlocked = (((w as usize - 1) * full.len()) / waves + 1).min(full.len());

        let mut kinds: Vec<ZombieKind> = full[..unlocked].to_vec();
        // 原版按章节逐步放开的僵尸，不塞进每关静态池，避免 50 个池子重复维护。
        let l = self.level;
        if w >= 2 {
            if l >= 5 && l < 10 { kinds.push(ZombieKind::Newspaper); }
            if l >= 8 && l < 10 { kinds.push(ZombieKind::Dancing); }
            if l >= 10 && l < 20 {
                if l >= 11 { kinds.push(ZombieKind::Jackbox); }
                if l >= 12 { kinds.push(ZombieKind::Balloon); }
                if l >= 14 { kinds.push(ZombieKind::Pogo); }
                if l >= 16 { kinds.push(ZombieKind::Digger); }
                if l >= 19 { kinds.push(ZombieKind::Yeti); }
            }
            if l >= 20 && l < 30 {
                kinds.push(ZombieKind::DuckyTube);
                if l >= 21 { kinds.push(ZombieKind::Snorkel); }
                if l >= 23 { kinds.push(ZombieKind::DolphinRider); }
                if l >= 25 { kinds.push(ZombieKind::Zomboni); }
                if l >= 27 { kinds.push(ZombieKind::Bobsled); }
            }
            if l >= 30 && l < 40 {
                if l >= 31 { kinds.push(ZombieKind::Bungee); }
                if l >= 33 { kinds.push(ZombieKind::Balloon); }
                if l >= 35 { kinds.push(ZombieKind::Digger); }
            }
            if l >= 40 {
                if l >= 41 { kinds.push(ZombieKind::Catapult); }
                if l >= 45 { kinds.push(ZombieKind::Gargantuar); }
                if l == 49 && w == self.cfg.waves { kinds.push(ZombieKind::Boss); }
            }
        }
        let mut candidates: Vec<(ZombieKind, u32)> = Vec::new();
        let mut total = 0u32;
        for kind in kinds {
            if kind == ZombieKind::Flag || kind.wave_value() == 0 || kind.wave_value() > *budget {
                continue;
            }
            if candidates.iter().any(|(k, _)| *k == kind) { continue; }
            let weight = kind.pick_weight().max(1);
            candidates.push((kind, weight));
            total += weight;
        }
        if candidates.is_empty() {
            return None;
        }

        let mut roll = self.rng.f(0.0, total as f32);
        for (kind, weight) in candidates {
            if roll < weight as f32 {
                *budget -= kind.wave_value();
                return Some(kind);
            }
            roll -= weight as f32;
        }
        None
    }

    fn start_wave(&mut self) {
        self.wave += 1;
        let w = self.wave;
        let flag = self.is_flag_wave(w);
        let mut budget = self.wave_points(w);
        let mut items: Vec<(f32, ZombieKind, usize)> = Vec::new();

        // 零售 1073 的旗帜波：先放 min(点数, 8) 只普通僵尸，再放旗帜僵尸。
        if flag {
            let plain = budget.min(8);
            for _ in 0..plain {
                budget -= ZombieKind::Basic.wave_value();
                items.push((self.rng.f(0.1, 0.6), ZombieKind::Basic, self.rng.range(self.rows())));
            }
            items.push((0.2, ZombieKind::Flag, self.rng.range(self.rows())));
        }

        while budget > 0 {
            let Some(kind) = self.pick_kind(w, &mut budget) else {
                break;
            };
            let idx = items.len();
            items.push((0.15 + idx as f32 * 0.42 + self.rng.f(0.0, 0.9), kind, self.rng.range(self.rows())));
        }

        // 冒险关末波补齐本关允许但还没出场的僵尸，和原版 PutInMissingZombies 一致。
        if w == self.cfg.waves {
            for &kind in self.cfg.pool {
                if !items.iter().any(|(_, k, _)| *k == kind) {
                    let idx = items.len();
                    items.push((0.2 + idx as f32 * 0.45 + self.rng.f(0.0, 0.6), kind, self.rng.range(self.rows())));
                }
            }
        }

        for (delay, kind, row) in items {
            if kind == ZombieKind::Dancing {
                for j in 0..2 {
                    self.pending.push(Pending {
                        at: self.elapsed + delay + 0.25 + j as f32 * 0.18,
                        kind: ZombieKind::BackupDancer,
                        row,
                    });
                }
            }
            self.pending.push(Pending {
                at: self.elapsed + delay,
                kind,
                row,
            });
        }

        let text = if w == self.cfg.waves {
            "最后一大波僵尸来袭！".to_string()
        } else if flag {
            "一大波僵尸正在接近！".to_string()
        } else {
            format!("第 {} 波僵尸来袭", w)
        };
        self.banner = Some((text, 2.8));
        if flag {
            self.sfx.horn();
        }

        self.next_wave_at = if w >= self.cfg.waves {
            f32::MAX
        } else if self.is_flag_wave(w + 1) {
            self.elapsed + 45.0
        } else {
            let idx = (w as usize).min(self.cfg.gaps.len().saturating_sub(1));
            self.elapsed + self.cfg.gaps[idx] + self.rng.f(0.0, 3.0)
        };
    }

    fn update_waves(&mut self, dt: f32) {
        let _ = dt;
        if self.wave < self.cfg.waves && self.elapsed >= self.next_wave_at {
            self.start_wave();
        }
        let mut spawned: Vec<(ZombieKind, usize)> = Vec::new();
        let mut i = 0;
        while i < self.pending.len() {
            if self.elapsed >= self.pending[i].at {
                let p = self.pending.remove(i);
                spawned.push((p.kind, p.row));
            } else {
                i += 1;
            }
        }
        for (kind, row) in spawned {
            self.zombies.push(Zombie {
                kind,
                row,
                x: W + 40.0 + self.rng.f(0.0, 90.0),
                hp: kind.hp(),
                max_hp: kind.hp(),
                speed: self.rng.f(kind.speed_range().0, kind.speed_range().1),
                t: self.rng.f(0.0, 5.0),
                slow: 0.0,
                eating: false,
                eat_t: 0.0,
                dying: false,
                death_t: 0.0,
                flash: 0.0,
                groan: self.rng.f(1.0, 4.0),
                hit_flash: 0.0,
                shield: kind.shield(),
                vaulted: false,
                diverted: false,
                phase: 0,
            });
        }
    }

    fn check_end(&mut self, dt: f32) {
        if self.state != State::Playing {
            return;
        }
        // 失败：僵尸越过防线，且该行割草机已用
        for z in self.zombies.iter() {
            if !z.dying && z.x < DEATH_X {
                let used = self.mowers.iter().any(|m| m.row == z.row && m.used);
                if used {
                    self.state = State::Lost;
                    self.sfx.music(false);
                    self.sfx.lose();
                    return;
                }
            }
        }
        // 胜利：最后一波结束且场上清空
        if self.wave >= self.cfg.waves && self.pending.is_empty() {
            let clear = self.zombies.iter().all(|z| z.dying);
            if clear {
                self.win_delay += dt;
                if self.win_delay > 1.4 {
                    self.state = State::Won;
                    self.sfx.music(false);
                    self.sfx.win();
                }
            } else {
                self.win_delay = 0.0;
            }
        }
    }

    // ---------------- 交互 ----------------

    pub fn pointer_move(&mut self, x: f32, y: f32) {
        self.mouse = (x, y);
    }

    pub fn pointer_down(&mut self, x: f32, y: f32, button: i16) {
        self.mouse = (x, y);
        match self.state {
            State::Menu => {
                if button != 2 && rect_contains(BTN_ADVENTURE, x, y) {
                    self.state = State::Map;
                    self.sfx.click();
                    return;
                }
            }
            State::Map => {
                if button == 2 || rect_contains(MAP_BACK, x, y) {
                    self.state = State::Menu;
                    self.sfx.click();
                    return;
                }
                for w in 0..WORLD_COUNT {
                    if rect_contains(world_tab_rect(w), x, y) {
                        self.map_page = w;
                        self.sfx.click();
                        return;
                    }
                }
                for j in 0..LEVELS_PER_WORLD {
                    if rect_contains(level_rect(j), x, y) {
                        self.enter_choose(self.map_page * LEVELS_PER_WORLD + j);
                        return;
                    }
                }
            }
            State::Choosing => {
                if button == 2 || rect_contains((12.0, 12.0, 92.0, 40.0), x, y) {
                    self.state = State::Map;
                    self.sfx.click();
                    return;
                }
                for i in 0..self.cfg.plants.len() {
                    if rect_contains(choose_rect(i), x, y) {
                        let k = self.cfg.plants[i];
                        self.toggle_seed(k);
                        return;
                    }
                }
                for i in 0..self.seeds.len() {
                    if rect_contains(seed_slot_rect(i), x, y) {
                        self.seeds.remove(i);
                        self.sfx.click();
                        return;
                    }
                }
                if rect_contains(CHOOSE_START, x, y) {
                    if self.seeds.is_empty() {
                        self.toast = Some(("请先选择至少一种植物".into(), 1.4, true));
                    } else {
                        self.start();
                    }
                    return;
                }
            }
            State::Playing => {
                if button == 2 {
                    self.selected = None;
                    self.shovel = false;
                    return;
                }
                if rect_contains(BTN_PAUSE, x, y) {
                    self.state = State::Paused;
                    self.sfx.pause();
                    self.sfx.music(false);
                    return;
                }
                if rect_contains(BTN_RESTART, x, y) {
                    self.start();
                    return;
                }
                if rect_contains(BTN_SOUND, x, y) {
                    self.muted = !self.muted;
                    self.sfx.enabled = !self.muted;
                    self.sfx.click();
                    self.sfx.music(!self.muted);
                    return;
                }
                // 阳光优先
                let mut got = false;
                for i in (0..self.suns.len()).rev() {
                    let s = &self.suns[i];
                    if s.state != SunState::Collect {
                        let dx = s.x - x;
                        let dy = s.y - y;
                        if dx * dx + dy * dy < 40.0 * 40.0 {
                            self.suns[i].state = SunState::Collect;
                            got = true;
                            
                            break;
                        }
                    }
                }
                if got {
                    return;
                }
                // 卡片
                for i in 0..self.seeds.len() {
                    if rect_contains(card_rect(i), x, y) {
                        let k = self.seeds[i];
                        if self.selected == Some(k) {
                            self.selected = None;
                        } else if self.cooldowns[k.idx()] > 0.0 {
                            self.toast = Some(("冷却中…".into(), 1.2, true));
                        } else if self.sun < k.cost() {
                            self.toast = Some(("阳光不足".into(), 1.2, true));
                        } else {
                            self.selected = Some(k);
                            self.shovel = false;
                        }
                        return;
                    }
                }
                if rect_contains((SHOVEL_X, SHOVEL_Y, SHOVEL_W, SHOVEL_H), x, y) {
                    self.shovel = !self.shovel;
                    self.selected = None;
                    return;
                }
                // 草坪
                if let Some((row, col)) = self.grid_at(x, y) {
                    if self.shovel {
                        let top = self
                            .plants
                            .iter()
                            .position(|p| p.row == row && p.col == col && !p.kind.is_base())
                            .or_else(|| {
                                self.plants
                                    .iter()
                                    .position(|p| p.row == row && p.col == col && p.kind.is_base())
                            });
                        if let Some(i) = top {
                            let p = self.plants.remove(i);
                            let (cx, cy) = self.cell_center(row, col);
                            for _ in 0..8 {
                                self.particles.push(part(
                                    cx + self.rng.f(-20.0, 20.0),
                                    cy + self.rng.f(-16.0, 16.0),
                                    self.rng.f(-30.0, 30.0),
                                    self.rng.f(-50.0, -10.0),
                                    0.6,
                                    5.0,
                                    (90, 150, 50),
                                    80.0,
                                    PKind::Leaf,
                                ));
                            }
                            let _ = p;
                            self.sfx.plant();
                        }
                        return;
                    }
                    if let Some(k) = self.selected {
                        if let Err(msg) = self.can_place(k, row, col) {
                            self.toast = Some((msg.into(), 1.2, true));
                        } else if self.sun < k.cost() {
                            self.toast = Some(("阳光不足".into(), 1.2, true));
                        } else if self.cooldowns[k.idx()] > 0.0 {
                            self.toast = Some(("冷却中…".into(), 1.2, true));
                        } else {
                            self.plant_at(k, row, col);
                        }
                        return;
                    }
                } else {
                    self.selected = None;
                    self.shovel = false;
                }
            }
            State::Paused => {
                if rect_contains(BTN_PAUSE, x, y) || rect_contains(START_BTN, x, y) {
                    self.state = State::Playing;
                    self.sfx.click();
                    self.sfx.music(true);
                }
            }
            State::Won | State::Lost => {
                if rect_contains(AGAIN_BTN, x, y) {
                    let next = if self.state == State::Won {
                        (self.level + 1).min(LEVEL_COUNT - 1)
                    } else {
                        self.level
                    };
                    self.enter_choose(next);
                }
            }
        }
    }

    fn cell_base(&self, row: usize, col: usize) -> Option<PlantKind> {
        self.plants
            .iter()
            .find(|p| p.row == row && p.col == col && p.kind.is_base())
            .map(|p| p.kind)
    }

    fn cell_normal(&self, row: usize, col: usize) -> bool {
        self.plants
            .iter()
            .any(|p| p.row == row && p.col == col && !p.kind.is_base())
    }

    pub fn can_place(&self, kind: PlantKind, row: usize, col: usize) -> Result<(), &'static str> {
        let base = self.cell_base(row, col);
        let normal = self.cell_normal(row, col);
        if kind.is_base() {
            if base.is_some() || normal {
                return Err("这里已经有植物了");
            }
            return match self.cfg.terrain {
                Terrain::Pool if is_water_row(self.cfg.terrain, row) => Ok(()),
                Terrain::Pool => Err("睡莲只能种在水面上"),
                Terrain::Roof => Ok(()),
                Terrain::Lawn => Err("这种地形不需要底座"),
            };
        }
        if normal {
            return Err("这里已经有植物了");
        }
        match self.cfg.terrain {
            Terrain::Pool if is_water_row(self.cfg.terrain, row) => {
                if base == Some(PlantKind::LilyPad) { Ok(()) } else { Err("水面上需要先种睡莲") }
            }
            Terrain::Roof => {
                if base == Some(PlantKind::FlowerPot) { Ok(()) } else { Err("屋顶上需要先放花盆") }
            }
            _ => Ok(()),
        }
    }

    fn plant_at(&mut self, mut kind: PlantKind, row: usize, col: usize) {
        if kind == PlantKind::Imitater {
            kind = self.last_planted.unwrap_or(PlantKind::Peashooter);
        }
        self.last_planted = Some(kind);
        let (cx, cy) = self.cell_center(row, col);
        self.plants.push(Plant {
            kind,
            row,
            col,
            hp: kind.hp(),
            max_hp: kind.hp(),
            t: 0.0,
            timer: match kind {
                PlantKind::Sunflower => self.rng.f(3.0, 12.5),
                PlantKind::SunShroom => self.rng.f(2.0, 7.0),
                PlantKind::TwinSunflower => 8.0,
                PlantKind::Marigold | PlantKind::GoldMagnet => 8.0,
                PlantKind::Peashooter
                | PlantKind::SnowPea
                | PlantKind::Repeater
                | PlantKind::PuffShroom
                | PlantKind::FumeShroom
                | PlantKind::SeaShroom
                | PlantKind::Cactus
                | PlantKind::SplitPea
                | PlantKind::GatlingPea
                | PlantKind::Cattail
                | PlantKind::ScaredyShroom => 0.25,
                PlantKind::Cabbage | PlantKind::Starfruit | PlantKind::KernelPult | PlantKind::MelonPult | PlantKind::WinterMelon => 0.25,
                PlantKind::GloomShroom => 1.2,
                PlantKind::Chomper => 0.4,
                PlantKind::MagnetShroom => 3.0,
                PlantKind::Blover => 3.0,
                PlantKind::CobCannon => 6.0,
                PlantKind::Spikeweed => 0.45,
                PlantKind::Spikerock => 0.4,
                PlantKind::Threepeater => 0.4,
                PlantKind::PotatoMine => 14.0,
                PlantKind::Squash => -1.0,
                _ => 0.0,
            },
            fuse: match kind {
                PlantKind::Cherry => 1.15,
                PlantKind::Jalapeno => 1.0,
                PlantKind::IceShroom => 0.9,
                PlantKind::DoomShroom => 1.2,
                PlantKind::TangleKelp => 0.7,
                _ => 0.0,
            },
            flash: 0.0,
            pop: 0.0,
            anim: 0.0,
        });
        self.sun -= kind.cost();
        self.cooldowns[kind.idx()] = kind.cooldown();
        self.selected = None;
        self.planted += 1;
        self.sfx.plant();
        for _ in 0..6 {
            self.particles.push(part(
                cx + self.rng.f(-18.0, 18.0),
                cy + 30.0,
                self.rng.f(-16.0, 16.0),
                self.rng.f(-40.0, -12.0),
                0.45,
                3.0,
                (120, 90, 50),
                60.0,
                PKind::Dot,
            ));
        }
    }

    pub fn grid_at(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if x < GRID_X || y < GRID_Y || x >= GRID_X + COLS as f32 * CELL_W || y >= GRID_Y + self.rows() as f32 * self.cell_h() {
            return None;
        }
        let col = ((x - GRID_X) / CELL_W) as usize;
        let row = ((y - GRID_Y) / self.cell_h()) as usize;
        if row < self.rows() && col < COLS {
            Some((row, col))
        } else {
            None
        }
    }

    pub fn hover_cell(&self) -> Option<(usize, usize)> {
        self.grid_at(self.mouse.0, self.mouse.1)
    }

    pub fn key(&mut self, code: &str) {
        match code {
            "KeyR" => {
                if matches!(self.state, State::Playing | State::Paused | State::Won | State::Lost) {
                    self.start();
                }
            }
            "Space" => {
                if self.state == State::Choosing {
                    if !self.seeds.is_empty() {
                        self.start();
                    }
                } else if self.state == State::Playing {
                    self.state = State::Paused;
                    self.sfx.pause();
                    self.sfx.music(false);
                } else if self.state == State::Paused {
                    self.state = State::Playing;
                    self.sfx.music(true);
                } else if self.state == State::Menu {
                    self.state = State::Map;
                    self.sfx.click();
                }
            }
            "Escape" => {
                self.selected = None;
                self.shovel = false;
                if self.state == State::Choosing {
                    self.state = State::Map;
                } else if self.state == State::Map {
                    self.state = State::Menu;
                } else if self.state == State::Playing {
                    self.state = State::Paused;
                    self.sfx.pause();
                    self.sfx.music(false);
                }
            }
            "KeyM" => {
                self.muted = !self.muted;
                self.sfx.enabled = !self.muted;
                self.sfx.music(!self.muted);
            }
            "KeyD" => {
                self.debug = !self.debug;
            }
            _ => {}
        }
    }
}

fn rng_r(a: f32, b: f32) -> f32 {
    // 简易随机（用于粒子抖动，不需要高质量）
    let t = js_sys::Date::now() as f32;
    let v = (t * 0.001).fract() * (b - a);
    a + v
}


fn hurt_zombie(z: &mut Zombie, dmg: f32) {
    if z.shield > 0.0 {
        let absorbed = dmg.min(z.shield);
        z.shield -= absorbed;
        z.hp -= dmg - absorbed;
    } else {
        z.hp -= dmg;
    }
    z.flash = 1.0;
}
