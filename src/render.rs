use crate::assets::{self, Art};
use crate::defs::*;
use crate::game::{Game, Mower, PKind, Particle, Plant, State, Sun, SunState, Zombie};
use std::f64::consts::PI;
use std::f32::consts::TAU;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

fn draw_img_rect(c: &CanvasRenderingContext2d, img: &HtmlImageElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) {
    use wasm_bindgen::{JsCast, JsValue};
    if let Ok(f) = js_sys::Reflect::get(c.as_ref(), &JsValue::from_str("drawImage"))
        .and_then(|v| v.dyn_into::<js_sys::Function>())
    {
        let _ = f.call9(
            c.as_ref(),
            img.as_ref(),
            &JsValue::from_f64(sx),
            &JsValue::from_f64(sy),
            &JsValue::from_f64(sw),
            &JsValue::from_f64(sh),
            &JsValue::from_f64(dx),
            &JsValue::from_f64(dy),
            &JsValue::from_f64(dw),
            &JsValue::from_f64(dh),
        );
    }
}

pub struct P<'a> {
    pub c: &'a CanvasRenderingContext2d,
}

fn rgba(r: u8, g: u8, b: u8, a: f32) -> String {
    format!("rgba({},{},{},{:.3})", r, g, b, a)
}

fn font(size: f32, bold: bool) -> String {
    format!(
        "{}{}px 'Trebuchet MS','Noto Sans SC','PingFang SC','Microsoft YaHei',sans-serif",
        if bold { "bold " } else { "" },
        size
    )
}

impl<'a> P<'a> {
    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32, col: impl AsRef<str>) {
        let _ = self.c.set_fill_style_str(col.as_ref());
        self.c.fill_rect(x as f64, y as f64, w as f64, h as f64);
    }
    pub fn circle(&self, x: f32, y: f32, r: f32, col: impl AsRef<str>) {
        let _ = self.c.set_fill_style_str(col.as_ref());
        let _ = self.c.begin_path();
        let _ = self.c.arc(x as f64, y as f64, r as f64, 0.0, TAU as f64);
        let _ = self.c.fill();
    }
    pub fn ell(&self, x: f32, y: f32, rx: f32, ry: f32, col: impl AsRef<str>) {
        self.ell_rot(x, y, rx, ry, 0.0, col.as_ref());
    }
    pub fn ell_rot(&self, x: f32, y: f32, rx: f32, ry: f32, rot: f32, col: impl AsRef<str>) {
        let _ = self.c.set_fill_style_str(col.as_ref());
        let _ = self.c.begin_path();
        let _ = self
            .c
            .ellipse(x as f64, y as f64, rx as f64, ry as f64, rot as f64, 0.0, TAU as f64);
        let _ = self.c.fill();
    }
    pub fn rr(&self, x: f32, y: f32, w: f32, h: f32, r: f32, col: impl AsRef<str>) {
        let _ = self.c.set_fill_style_str(col.as_ref());
        self.c.begin_path();
        self.rounded_path(x, y, w, h, r);
        let _ = self.c.fill();
    }
    pub fn rr_s(&self, x: f32, y: f32, w: f32, h: f32, r: f32, wid: f32, col: impl AsRef<str>) {
        let _ = self.c.set_stroke_style_str(col.as_ref());
        let _ = self.c.set_line_width(wid as f64);
        self.c.begin_path();
        self.rounded_path(x, y, w, h, r);
        let _ = self.c.stroke();
    }
    fn rounded_path(&self, x: f32, y: f32, w: f32, h: f32, r: f32) {
        let r = r.min(w * 0.5).min(h * 0.5);
        let c = self.c;
        c.move_to((x + r) as f64, y as f64);
        c.line_to((x + w - r) as f64, y as f64);
        c.arc(
            (x + w - r) as f64,
            (y + r) as f64,
            r as f64,
            -PI / 2.0,
            0.0,
        );
        c.line_to((x + w) as f64, (y + h - r) as f64);
        c.arc(
            (x + w - r) as f64,
            (y + h - r) as f64,
            r as f64,
            0.0,
            PI / 2.0,
        );
        c.line_to((x + r) as f64, (y + h) as f64);
        c.arc(
            (x + r) as f64,
            (y + h - r) as f64,
            r as f64,
            PI / 2.0,
            PI,
        );
        c.line_to(x as f64, (y + r) as f64);
        c.arc((x + r) as f64, (y + r) as f64, r as f64, PI, 1.5 * PI);
        let _ = c.close_path();
    }
    pub fn poly(&self, pts: &[(f32, f32)], col: impl AsRef<str>) {
        if pts.is_empty() {
            return;
        }
        let _ = self.c.set_fill_style_str(col.as_ref());
        self.c.begin_path();
        let _ = self.c.move_to(pts[0].0 as f64, pts[0].1 as f64);
        for p in &pts[1..] {
            self.c.line_to(p.0 as f64, p.1 as f64);
        }
        let _ = self.c.close_path();
        let _ = self.c.fill();
    }
    pub fn line(&self, pts: &[(f32, f32)], wid: f32, col: impl AsRef<str>) {
        if pts.is_empty() {
            return;
        }
        let _ = self.c.set_stroke_style_str(col.as_ref());
        let _ = self.c.set_line_width(wid as f64);
        self.c.begin_path();
        let _ = self.c.move_to(pts[0].0 as f64, pts[0].1 as f64);
        for p in &pts[1..] {
            self.c.line_to(p.0 as f64, p.1 as f64);
        }
        let _ = self.c.stroke();
    }
    pub fn arcw(&self, x: f32, y: f32, r: f32, a0: f32, a1: f32, wid: f32, col: impl AsRef<str>) {
        let _ = self.c.set_stroke_style_str(col.as_ref());
        let _ = self.c.set_line_width(wid as f64);
        let _ = self.c.begin_path();
        let _ = self.c.arc(x as f64, y as f64, r as f64, a0 as f64, a1 as f64);
        let _ = self.c.stroke();
    }
    pub fn text(&self, s: &str, x: f32, y: f32, size: f32, col: impl AsRef<str>, align: &str, bold: bool) {
        let _ = self.c.set_font(&font(size, bold));
        let _ = self.c.set_text_align(align);
        let _ = self.c.set_text_baseline("middle");
        let _ = self.c.set_fill_style_str(col.as_ref());
        let _ = self.c.fill_text(s, x as f64, y as f64);
    }
    pub fn text_out(
        &self,
        s: &str,
        x: f32,
        y: f32,
        size: f32,
        fill: impl AsRef<str>,
        stroke: impl AsRef<str>,
        align: &str,
        bold: bool,
    ) {
        let _ = self.c.set_font(&font(size, bold));
        let _ = self.c.set_text_align(align);
        let _ = self.c.set_text_baseline("middle");
        let _ = self.c.set_line_width((size * 0.16) as f64);
        let _ = self.c.set_stroke_style_str(stroke.as_ref());
        let _ = self.c.stroke_text(s, x as f64, y as f64);
        let _ = self.c.set_fill_style_str(fill.as_ref());
        let _ = self.c.fill_text(s, x as f64, y as f64);
    }
    pub fn measure(&self, s: &str, size: f32, bold: bool) -> f32 {
        let _ = self.c.set_font(&font(size, bold));
        self.c
            .measure_text(s)
            .map(|m| m.width() as f32)
            .unwrap_or(0.0)
    }
    pub fn alpha(&self, a: f32) {
        let _ = self.c.set_global_alpha(a as f64);
    }
    pub fn save(&self) {
        let _ = self.c.save();
    }
    pub fn restore(&self) {
        let _ = self.c.restore();
    }
}

// ================= 主入口 =================

pub fn render(g: &Game, ctx: &CanvasRenderingContext2d) {
    let p = P { c: ctx };
    if g.state == State::Menu {
        draw_menu(&p, g);
        return;
    }
    if g.state == State::Map {
        draw_map(&p, g);
        return;
    }
    if g.state == State::Choosing {
        draw_chooser(&p, g);
        return;
    }
    if g.shake > 0.0 {
        p.save();
        let dx = (g.time * 83.0).sin() * g.shake * 0.55;
        let dy = (g.time * 97.0).cos() * g.shake * 0.35;
        p.c.translate(dx as f64, dy as f64);
    }
    draw_scene(&p, g);
    if g.shake > 0.0 {
        p.restore();
    }
    let (tr, tg, tb, ta) = g.cfg.tint;
    if ta > 0.0 {
        p.rect(0.0, HUD_H, W, H - HUD_H, rgba(tr, tg, tb, ta));
    }
    draw_hud(&p, g);
    match g.state {
        State::Paused => draw_pause(&p, g),
        State::Won => draw_end(&p, g, true),
        State::Lost => draw_end(&p, g, false),
        _ => {}
    }
    if g.flash_screen > 0.0 {
        p.rect(0.0, 0.0, W, H, &rgba(255, 248, 214, g.flash_screen * 0.55));
    }
}

fn draw_scene(p: &P, g: &Game) {
    // 原版关卡背景（含房屋、草坪、道路）；未加载时退回程序绘制
    let bg = assets::bg(g.level);
    if let Some(img) = bg {
        p.rect(0.0, 0.0, W, H, "#1a2a12");
        // 1400×600 素材按高度铺满 HUD 以下区域，左侧房屋对齐，右侧道路自然裁切
        let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
            &img, 0.0, HUD_H as f64, 1437.0, (H - HUD_H) as f64,
        );
        // 草坪边缘轻微压暗，增强纵深
        p.rect(GRID_X, LAWN_BOTTOM, COLS as f32 * CELL_W, 2.0, rgba(20, 45, 10, 0.18));
    } else {
        p.rect(0.0, 0.0, W, H, "#4f8c2c");
        for col in 0..COLS {
            let c = if col % 2 == 0 { "#85c951" } else { "#77bc44" };
            p.rect(GRID_X + col as f32 * CELL_W, GRID_Y, CELL_W, g.cell_h() * g.rows() as f32, c);
        }
        draw_house(p, g);
    }

    if g.cfg.terrain == Terrain::Pool {
        let wx = GRID_X;
        let wy = GRID_Y + 2.25 * g.cell_h();
        let ww = COLS as f32 * CELL_W;
        let wh = 2.55 * g.cell_h();
        if let Some(img) = assets::water() {
            let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
                &img, wx as f64, wy as f64, ww as f64, wh as f64,
            );
        } else {
            p.rect(wx, wy, ww, wh, "#35b8dc");
        }
        if g.level / 10 == 1 || g.level / 10 == 3 {
            p.rect(wx, wy, ww, wh, rgba(14, 38, 72, 0.38));
        }
        p.rect(wx, wy, ww, 6.0, rgba(216, 250, 255, 0.55));
        p.rect(wx, wy + wh - 7.0, ww, 7.0, rgba(20, 95, 130, 0.30));
        for i in 0..7 {
            let bx = wx + 40.0 + i as f32 * 110.0 + (g.time * 12.0 + i as f32 * 9.0).sin() * 6.0;
            p.ell(bx, wy + 22.0 + (g.time + i as f32).sin().abs() * 8.0, 12.0, 3.0, rgba(225, 252, 255, 0.45));
        }
    }

    // 割草机
    for m in &g.mowers {
        draw_mower(p, m, g.time, g.cell_h());
    }

    // 植物
    for pl in &g.plants {
        draw_plant(p, pl, g.time, g.cell_h());
    }

    // 悬停格子
    if let Some((r, c)) = g.hover_cell() {
        let x = GRID_X + c as f32 * CELL_W;
        let y = GRID_Y + r as f32 * g.cell_h();
        if g.selected.is_some() || g.shovel {
            let valid = match g.selected {
                Some(k) => {
                    g.can_place(k, r, c).is_ok()
                        && g.sun >= k.cost()
                        && g.cooldowns[k.idx()] <= 0.0
                }
                None => true,
            };
            p.rect(
                x,
                y,
                CELL_W,
                g.cell_h(),
                if valid {
                    rgba(255, 255, 220, 0.22)
                } else {
                    rgba(255, 80, 60, 0.22)
                },
            );
            p.rr_s(
                x + 2.0,
                y + 2.0,
                CELL_W - 4.0,
                CELL_H - 4.0,
                6.0,
                2.0,
                if valid {
                    rgba(255, 255, 235, 0.65)
                } else {
                    rgba(255, 90, 70, 0.75)
                },
            );
            if let Some(k) = g.selected {
                p.save();
                p.alpha(0.42);
                draw_plant_kind(p, k, x + CELL_W * 0.5, y + g.cell_h() - 10.0, g.time, 1.0, 1.0, 0.0);
                p.restore();
                p.alpha(1.0);
            } else if g.shovel {
                p.save();
                p.alpha(0.6);
                draw_shovel_icon(p, x + CELL_W * 0.5, y + g.cell_h() * 0.5, 0.9, -0.3);
                p.restore();
                p.alpha(1.0);
            }
        } else {
            p.rect(x, y, CELL_W, g.cell_h(), rgba(255, 255, 255, 0.07));
        }
    }

    // 僵尸（按行排序，前排后画）
    let mut order: Vec<usize> = (0..g.zombies.len()).collect();
    order.sort_by(|a, b| {
        let za = &g.zombies[*a];
        let zb = &g.zombies[*b];
        za.row
            .cmp(&zb.row)
            .then(zb.x.partial_cmp(&za.x).unwrap_or(std::cmp::Ordering::Equal))
    });
    for i in order {
        draw_zombie(p, &g.zombies[i], g.time, g.cell_h());
    }

    // 豌豆
    for pea in &g.peas {
        if pea.t < 0.0 {
            continue;
        }
        if pea.pk == 2 {
            p.circle(pea.x, pea.y, 8.0, "#5f9e34");
            p.circle(pea.x - 2.0, pea.y - 3.0, 3.0, "#a6d96b");
            p.arcw(pea.x, pea.y, 9.0, 0.0, TAU, 1.2, "#3f7422");
        } else if pea.pk == 3 {
            for k in 0..5 {
                let a = -PI as f32 * 0.5 + k as f32 * TAU / 5.0;
                p.poly(
                    &[
                        (pea.x + a.cos() * 9.0, pea.y + a.sin() * 9.0),
                        (pea.x + (a + 2.0).cos() * 4.0, pea.y + (a + 2.0).sin() * 4.0),
                        (pea.x + (a - 2.0).cos() * 4.0, pea.y + (a - 2.0).sin() * 4.0),
                    ],
                    "#ffd23f",
                );
            }
        } else if let Some(img) = assets::projectile(pea.pk) {
            let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
                &img,
                (pea.x - 11.0) as f64,
                (pea.y - 11.0) as f64,
                22.0,
                22.0,
            );
        } else if pea.slow {
            p.circle(pea.x - 6.0, pea.y, 6.5, rgba(140, 220, 240, 0.35));
            p.circle(pea.x - 2.0, pea.y, 7.0, "#9fe6f5");
            p.circle(pea.x + 1.0, pea.y - 2.0, 2.4, "#e9ffff");
        } else {
            p.circle(pea.x, pea.y, 7.0, "#74c93c");
            p.circle(pea.x - 2.0, pea.y - 2.0, 2.6, "#b6ec7a");
        }
    }

    // 粒子
    for pt in &g.particles {
        draw_particle(p, pt);
    }

    // 阳光
    for s in &g.suns {
        draw_sun(p, s, g.time);
    }

    if g.debug {
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("mouse {:.0},{:.0}  sun {}", g.mouse.0, g.mouse.1, g.sun));
        for (i, z) in g.zombies.iter().enumerate().take(6) {
            lines.push(format!(
                "Z{} row{} x{:.0} hp{:.0} eat{} dy{}",
                i, z.row, z.x, z.hp, z.eating as i32, z.dying as i32
            ));
        }
        for (i, pl) in g.plants.iter().enumerate().take(4) {
            lines.push(format!("P{} {} r{} c{} hp{:.0}", i, pl.kind.name(), pl.row, pl.col, pl.hp));
        }
        for (k, line) in lines.iter().enumerate() {
            p.text(line, 316.0, 128.0 + k as f32 * 18.0, 13.0, "#ffffff", "left", false);
        }
    }

    // 横幅
    if let Some((text, t)) = &g.banner {
        let total = 2.8;
        let age = (total - *t).max(0.0);
        let a = (age / 0.22).min(1.0) * (*t / 0.55).min(1.0);
        let w = p.measure(text, 34.0, true) + 76.0;
        p.save();
        p.alpha(a);
        p.rr(W * 0.5 - w * 0.5, 150.0, w, 64.0, 12.0, rgba(18, 38, 12, 0.88));
        p.rr_s(
            W * 0.5 - w * 0.5,
            150.0,
            w,
            64.0,
            12.0,
            2.0,
            rgba(255, 220, 120, 0.7),
        );
        p.text_out(text, W * 0.5, 183.0, 34.0, "#ffe14d", "#3a2404", "center", true);
        p.restore();
        p.alpha(1.0);
    }

    // Toast
    if let Some((text, t, warn)) = &g.toast {
        let a = (*t / 0.25).min(1.0);
        let w = p.measure(text, 18.0, true) + 40.0;
        p.save();
        p.alpha(a);
        p.rr(
            W * 0.5 - w * 0.5,
            566.0,
            w,
            38.0,
            8.0,
            if *warn {
                rgba(120, 40, 30, 0.85)
            } else {
                rgba(30, 70, 24, 0.85)
            },
        );
        p.text(
            text,
            W * 0.5,
            585.0,
            18.0,
            if *warn { "#ffd9d0" } else { "#e9ffcf" },
            "center",
            true,
        );
        p.restore();
        p.alpha(1.0);
    }

    if g.level / 10 == 3 {
        let fx = GRID_X;
        let fy = GRID_Y;
        let fw = COLS as f32 * CELL_W;
        let fh = g.rows() as f32 * g.cell_h();
        if let Some(img) = assets::fog() {
            p.save();
            p.alpha(0.42);
            let dx = (g.time * 18.0).sin() * 26.0;
            let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
                &img, (fx + dx - 30.0) as f64, fy as f64, (fw + 60.0) as f64, fh as f64,
            );
            p.alpha(0.30);
            let dx2 = (g.time * 11.0 + 2.0).sin() * 40.0;
            let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
                &img, (fx + dx2 - 40.0) as f64, (fy + 12.0) as f64, (fw + 80.0) as f64, fh as f64,
            );
            p.restore();
            p.alpha(1.0);
        } else {
            p.rect(fx, fy, fw, fh, rgba(215, 230, 225, 0.18));
        }
    }
}

fn draw_house(p: &P, _g: &Game) {
    // 砖墙
    p.rect(0.0, 0.0, 252.0, H, "#d6c1a0");
    for y in (0..720).step_by(26) {
        p.rect(0.0, y as f32, 252.0, 2.0, rgba(122, 92, 62, 0.22));
        let off = if (y / 26) % 2 == 0 { 0.0 } else { 30.0 };
        let mut x = off - 30.0;
        while x < 252.0 {
            p.rect(x + 30.0, y as f32, 2.0, 26.0, rgba(122, 92, 62, 0.18));
            x += 60.0;
        }
    }
    // 阴影
    p.rect(238.0, 0.0, 14.0, H, rgba(60, 40, 20, 0.18));
    // 窗户
    p.rr(52.0, 136.0, 140.0, 116.0, 10.0, "#f4ebd6");
    p.rr(62.0, 146.0, 120.0, 96.0, 7.0, "#8fc9dd");
    p.poly(
        &[(66.0, 150.0), (130.0, 150.0), (98.0, 238.0), (66.0, 238.0)],
        rgba(255, 255, 255, 0.35),
    );
    p.rect(118.0, 146.0, 8.0, 96.0, "#f7f0dc");
    p.rect(62.0, 190.0, 120.0, 8.0, "#f7f0dc");
    // 窗台
    p.rr(44.0, 248.0, 156.0, 14.0, 6.0, "#b8a184");
    // 门
    p.rr(84.0, 452.0, 130.0, 268.0, 12.0, "#7a5233");
    p.rr(94.0, 462.0, 110.0, 110.0, 8.0, "#875b39");
    p.rr(94.0, 586.0, 110.0, 124.0, 8.0, "#875b39");
    p.rr_s(94.0, 462.0, 110.0, 110.0, 8.0, 3.0, "#5d3c23");
    p.rr_s(94.0, 586.0, 110.0, 124.0, 8.0, 3.0, "#5d3c23");
    p.circle(190.0, 592.0, 6.0, "#e2b64a");
    // 踢脚线
    p.rect(0.0, 700.0, 252.0, 20.0, "#6d4b2e");

    // 割草机跑道
    p.rect(252.0, HUD_H, 48.0, 616.0, "#6ea93e");
    p.rect(252.0, HUD_H, 6.0, 616.0, rgba(30, 55, 15, 0.18));
    p.rect(294.0, HUD_H, 6.0, 616.0, rgba(30, 55, 15, 0.14));
    for r in 0..ROWS {
        let y = row_ground(r, CELL_H);
        p.ell(276.0, y + 8.0, 22.0, 7.0, rgba(20, 45, 10, 0.12));
    }
}

// ================= 植物绘制 =================

fn draw_plant(p: &P, pl: &Plant, time: f32, h: f32) {
    let cx = GRID_X + pl.col as f32 * CELL_W + CELL_W * 0.5;
    let base = row_ground(pl.row, h);
    let s = 0.55 + 0.45 * pl.pop.min(1.0);
    p.ell(cx, base + 3.0, 30.0, 8.0, rgba(20, 45, 10, 0.20));
    let ratio = (pl.hp / pl.max_hp).clamp(0.0, 1.0);
    p.save();
    p.c.translate(cx as f64, base as f64);
    p.c.scale(s as f64, s as f64);
    p.c.translate(-cx as f64, -base as f64);
    if pl.kind == PlantKind::PotatoMine {
        draw_potato_mine(p, pl, time, h);
    } else {
        draw_plant_anim(p, pl.kind, cx, base, time, ratio, pl.fuse, pl.anim);
    }
    p.restore();
}

fn draw_potato_mine(p: &P, pl: &Plant, time: f32, h: f32) {
    let cx = GRID_X + pl.col as f32 * CELL_W + CELL_W * 0.5;
    let base = row_ground(pl.row, h);
    let idle = crate::plant_anim::IDLE[Art::PotatoMine as usize];
    let rise = crate::plant_anim::RISE_POTATO;
    let (img, clip, frame) = if pl.timer > 0.0 {
        // 未武装：停在原版 anim_rise 第一帧，只露出土堆和熄灭的引信。
        (assets::plant_rise(), rise, 0u32)
    } else if pl.timer > -1.55 {
        let k = (-pl.timer / 1.55).clamp(0.0, 1.0);
        (assets::plant_rise(), rise, ((k * (rise.frames - 1) as f32) as u32).min(rise.frames - 1))
    } else {
        let f = ((time * 12.0) as u32) % idle.frames;
        (assets::plant_sheet(Art::PotatoMine, false), idle, f)
    };
    let Some(img) = img else { return };
    let scale = art_height(Art::PotatoMine) / idle.h;
    let dw = clip.w * scale;
    let dh = clip.h * scale;
    p.save();
    p.c.translate(cx as f64, (base + 8.0 + 4.0 * scale) as f64);
    let sx = (frame * clip.w as u32) as f64;
    draw_img_rect(
        p.c,
        &img,
        sx,
        0.0,
        clip.w as f64,
        clip.h as f64,
        (-dw * 0.5) as f64,
        -dh as f64,
        dw as f64,
        dh as f64,
    );
    p.restore();
}

fn art_of(kind: PlantKind) -> Option<Art> {
    Some(match kind {
        PlantKind::Sunflower => Art::Sunflower,
        PlantKind::Peashooter => Art::Peashooter,
        PlantKind::SnowPea => Art::SnowPea,
        PlantKind::Repeater => Art::Repeater,
        PlantKind::WallNut => Art::Wallnut,
        PlantKind::Cherry => Art::Cherry,
        PlantKind::PotatoMine => Art::PotatoMine,
        PlantKind::Jalapeno => Art::Jalapeno,
        PlantKind::Threepeater => Art::Threepeater,
        PlantKind::Squash => Art::Squash,
        PlantKind::Cabbage => Art::Cabbage,
        PlantKind::Starfruit => Art::Starfruit,
        PlantKind::Garlic => Art::Garlic,
        PlantKind::Spikeweed => Art::Spikeweed,
        PlantKind::LilyPad => Art::LilyPad,
        PlantKind::FlowerPot => Art::FlowerPot,
        PlantKind::Chomper => Art::Chomper,
        PlantKind::PuffShroom => Art::PuffShroom,
        PlantKind::SunShroom => Art::SunShroom,
        PlantKind::FumeShroom => Art::FumeShroom,
        PlantKind::GraveBuster => Art::GraveBuster,
        PlantKind::HypnoShroom => Art::HypnoShroom,
        PlantKind::ScaredyShroom => Art::ScaredyShroom,
        PlantKind::IceShroom => Art::IceShroom,
        PlantKind::DoomShroom => Art::DoomShroom,
        PlantKind::TangleKelp => Art::TangleKelp,
        PlantKind::Torchwood => Art::Torchwood,
        PlantKind::TallNut => Art::TallNut,
        PlantKind::SeaShroom => Art::SeaShroom,
        PlantKind::Plantern => Art::Plantern,
        PlantKind::Cactus => Art::Cactus,
        PlantKind::Blover => Art::Blover,
        PlantKind::SplitPea => Art::SplitPea,
        PlantKind::Pumpkin => Art::Pumpkin,
        PlantKind::MagnetShroom => Art::MagnetShroom,
        PlantKind::KernelPult => Art::KernelPult,
        PlantKind::CoffeeBean => Art::CoffeeBean,
        PlantKind::UmbrellaLeaf => Art::UmbrellaLeaf,
        PlantKind::Marigold => Art::Marigold,
        PlantKind::MelonPult => Art::MelonPult,
        PlantKind::GatlingPea => Art::GatlingPea,
        PlantKind::TwinSunflower => Art::TwinSunflower,
        PlantKind::GloomShroom => Art::GloomShroom,
        PlantKind::Cattail => Art::Cattail,
        PlantKind::WinterMelon => Art::WinterMelon,
        PlantKind::GoldMagnet => Art::GoldMagnet,
        PlantKind::Spikerock => Art::Spikerock,
        PlantKind::CobCannon => Art::CobCannon,
        PlantKind::Imitater => Art::Imitater,
    })
}

fn draw_plant_kind(p: &P, kind: PlantKind, cx: f32, base: f32, t: f32, hp: f32, fuse: f32, anim: f32) {
    if let Some(art) = art_of(kind) {
        if assets::ready() && draw_art_plant(p, kind, art, cx, base, t, hp, fuse, anim) {
            return;
        }
    }
    match kind {
        PlantKind::Sunflower => draw_sunflower(p, cx, base, t),
        PlantKind::Peashooter => draw_peashooter(p, cx, base, t, 0.0),
        PlantKind::SnowPea => draw_peashooter(p, cx, base, t, 1.0),
        PlantKind::Repeater => draw_peashooter(p, cx, base, t, 2.0),
        PlantKind::WallNut => draw_wallnut(p, cx, base, t, hp),
        PlantKind::Cherry => draw_cherry(p, cx, base, t, fuse),
        PlantKind::PotatoMine => {
            p.save();
            p.c.translate(cx as f64, base as f64);
            p.ell(0.0, -12.0, 28.0, 18.0, "#c99655");
            p.ell(-8.0, -18.0, 8.0, 5.0, "#e0b070");
            p.circle(-8.0, -16.0, 3.0, "#33220f");
            p.circle(9.0, -16.0, 3.0, "#33220f");
            p.rect(-2.0, -44.0, 4.0, 18.0, "#888");
            p.circle(0.0, -46.0, 8.0, if fuse > 0.5 { "#c33" } else { "#ff3b3b" });
            p.restore();
        }
        PlantKind::Jalapeno => {
            p.save();
            p.c.translate(cx as f64, base as f64);
            p.ell(0.0, -50.0, 18.0, 48.0, "#d8321f");
            p.poly(&[(-8.0, -92.0), (8.0, -92.0), (14.0, -104.0)], "#3f8f2a");
            p.circle(-6.0, -58.0, 4.0, "#fff");
            p.circle(8.0, -74.0, 4.0, "#fff");
            p.restore();
        }
        PlantKind::Threepeater => {
            for (i, dy) in [-26.0f32, 0.0, 26.0].iter().enumerate() {
                draw_peashooter(p, cx, base + *dy, t, 0.0);
                let _ = i;
            }
        }
        PlantKind::Squash => {
            p.save();
            p.c.translate(cx as f64, base as f64);
            p.ell(0.0, -30.0, 30.0, 34.0, "#b6c94c");
            p.ell(-10.0, -42.0, 4.0, 5.0, "#2b3a12");
            p.ell(10.0, -42.0, 4.0, 5.0, "#2b3a12");
            p.arcw(0.0, -32.0, 10.0, 0.2 * PI as f32, 0.8 * PI as f32, 2.4, "#66701f");
            p.restore();
        }
        PlantKind::Cabbage => {
            p.circle(cx, base - 96.0, 22.0, "#6fae3d");
            p.arcw(cx, base - 96.0, 18.0, 0.0, TAU, 3.0, "#9bd36a");
            p.rect(cx - 3.0, base - 82.0, 6.0, 66.0, "#4b7d2a");
        }
        PlantKind::Starfruit => {
            for k in 0..5 {
                let a = -PI as f32 * 0.5 + k as f32 * TAU / 5.0;
                p.poly(
                    &[
                        (cx + a.cos() * 40.0, base - 46.0 + a.sin() * 40.0),
                        (cx + (a + 2.0).cos() * 18.0, base - 46.0 + (a + 2.0).sin() * 18.0),
                        (cx + (a - 2.0).cos() * 18.0, base - 46.0 + (a - 2.0).sin() * 18.0),
                    ],
                    "#ffc93c",
                );
            }
        }
        PlantKind::Garlic => {
            p.ell(cx, base - 34.0, 28.0, 32.0, "#eee6d0");
            p.circle(cx - 9.0, base - 44.0, 4.0, "#7a6a59");
            p.circle(cx + 9.0, base - 44.0, 4.0, "#7a6a59");
            p.rect(cx - 2.0, base - 68.0, 4.0, 12.0, "#cfc5ac");
        }
        PlantKind::LilyPad => {
            p.ell(cx, base - 14.0, 38.0, 22.0, "#5d9e35");
            p.ell(cx, base - 18.0, 30.0, 16.0, "#79bd4a");
            p.poly(&[(cx - 2.0, base - 20.0), (cx + 10.0, base - 12.0), (cx, base - 8.0)], "#5d9e35");
        }
        PlantKind::FlowerPot => {
            p.poly(&[(cx - 24.0, base - 38.0), (cx + 24.0, base - 38.0), (cx + 17.0, base), (cx - 17.0, base)], "#a85f2d");
            p.ell(cx, base - 38.0, 24.0, 8.0, "#c7793d");
            p.ell(cx, base - 39.0, 19.0, 5.0, "#4b351f");
        }
        PlantKind::Spikeweed => {
            p.ell(cx, base - 8.0, 34.0, 10.0, "#5a8f2e");
            for k in 0..5 {
                let dx = -24.0 + k as f32 * 12.0;
                p.poly(&[(cx + dx, base - 8.0), (cx + dx + 5.0, base - 8.0), (cx + dx + 2.0, base - 30.0)], "#b9d48a");
            }
        }
        _ => {}
    }
}

fn art_height(art: Art) -> f32 {
    match art {
        Art::Sunflower => 116.0,
        Art::Peashooter => 108.0,
        Art::Repeater => 108.0,
        Art::Wallnut => 96.0,
        Art::Cherry => 92.0,
        Art::SnowPea => 108.0,
        Art::PotatoMine => 62.0,
        Art::Jalapeno => 94.0,
        Art::Threepeater => 98.0,
        Art::Squash => 98.0,
        Art::Cabbage => 104.0,
        Art::Starfruit => 86.0,
        Art::Garlic => 80.0,
        Art::Spikeweed => 42.0,
        Art::LilyPad => 72.0,
        Art::FlowerPot => 92.0,
        Art::Chomper => 104.0,
        Art::PuffShroom => 84.0,
        Art::SunShroom => 88.0,
        Art::FumeShroom => 96.0,
        Art::GraveBuster => 86.0,
        Art::HypnoShroom => 92.0,
        Art::ScaredyShroom => 92.0,
        Art::IceShroom => 86.0,
        Art::DoomShroom => 112.0,
        Art::TangleKelp => 80.0,
        Art::Torchwood => 104.0,
        Art::TallNut => 132.0,
        Art::SeaShroom => 76.0,
        Art::Plantern => 126.0,
        Art::Cactus => 112.0,
        Art::Blover => 96.0,
        Art::SplitPea => 110.0,
        Art::Pumpkin => 112.0,
        Art::MagnetShroom => 96.0,
        Art::KernelPult => 106.0,
        Art::CoffeeBean => 84.0,
        Art::UmbrellaLeaf => 118.0,
        Art::Marigold => 88.0,
        Art::MelonPult => 112.0,
        Art::GatlingPea => 114.0,
        Art::TwinSunflower => 120.0,
        Art::GloomShroom => 100.0,
        Art::Cattail => 116.0,
        Art::WinterMelon => 116.0,
        Art::GoldMagnet => 104.0,
        Art::Spikerock => 70.0,
        Art::CobCannon => 114.0,
        Art::Imitater => 76.0,
    }
}
// ===== 原版 reanim 图集绘制（12fps 逐帧烘焙的网格图集）=====
fn reanim_plant_name(art: Art) -> &'static str {
    match art {
        Art::Cherry => "cherry",
        Art::Cabbage => "cabbage",
        Art::UmbrellaLeaf => "umbrelleaf",
        _ => art.key(),
    }
}

fn reanim_zombie_name(kind: ZombieKind) -> &'static str {
    match kind {
        ZombieKind::Basic => "basic",
        ZombieKind::Cone => "cone",
        ZombieKind::Bucket => "bucket",
        ZombieKind::Flag => "flag",
        ZombieKind::PoleVault => "polevault",
        ZombieKind::ScreenDoor => "screendoor",
        ZombieKind::Football => "football",
        ZombieKind::Ladder => "ladder",
        ZombieKind::Newspaper => "newspaper",
        ZombieKind::Dancing => "dancing",
        ZombieKind::BackupDancer => "backup",
        ZombieKind::DuckyTube => "duckytube",
        ZombieKind::Snorkel => "snorkel",
        ZombieKind::Zomboni => "zamboni",
        ZombieKind::Bobsled => "bobsled",
        ZombieKind::DolphinRider => "dolphinrider",
        ZombieKind::Jackbox => "jackbox",
        ZombieKind::Balloon => "balloon",
        ZombieKind::Digger => "digger",
        ZombieKind::Pogo => "pogo",
        ZombieKind::Yeti => "yeti",
        ZombieKind::Bungee => "bungee",
        ZombieKind::Catapult => "catapult",
        ZombieKind::Gargantuar => "gargantuar",
        ZombieKind::Imp => "imp",
        ZombieKind::Boss => "boss",
    }
}

/// 从 cols x rows 网格图集取第 frame 帧，铺到 (dx, dy, dw, dh)。
fn draw_cell(c: &CanvasRenderingContext2d, img: &HtmlImageElement, clip: &crate::reanim::Clip, frame: u32, dx: f32, dy: f32, dw: f32, dh: f32) {
    let cols = clip.cols.max(1) as u32;
    let col = (frame % cols) as f64;
    let row = (frame / cols) as f64;
    draw_img_rect(
        c, img,
        col * clip.w as f64, row * clip.h as f64,
        clip.w as f64, clip.h as f64,
        dx as f64, dy as f64, dw as f64, dh as f64,
    );
}

fn draw_reanim_plant(p: &P, kind: PlantKind, art: Art, cx: f32, base: f32, t: f32, hp: f32, fuse: f32, anim: f32) -> bool {
    let Some(ch) = crate::reanim::by_name(reanim_plant_name(art)) else { return false };
    let shooting = anim > 0.0 && ch.has(&ch.shoot);
    let clip = if shooting { ch.shoot } else { ch.base };
    if !ch.has(&clip) || ch.h <= 0.0 {
        return false;
    }
    let Some(img) = assets::reanim_img(clip.slot) else { return false };
    let idx = art as usize;
    let scale = art_height(art) / ch.h;
    let dw = clip.w * scale;
    let dh = clip.h * scale;
    let frame = if shooting {
        let dur = if art == Art::Starfruit { 0.16 } else { 0.20 };
        let progress = (1.0 - (anim / dur).clamp(0.0, 1.0)) * clip.frames as f32;
        (progress as u32).min(clip.frames.saturating_sub(1) as u32)
    } else {
        ((t + idx as f32 * 0.17) * clip.fps.max(1.0) * 0.62) as u32 % clip.frames.max(1) as u32
    };
    let pulse = if kind == PlantKind::Cherry && fuse < 0.55 {
        1.0 + (t * 16.0).sin().abs() * 0.10
    } else {
        1.0
    };
    p.save();
    p.c.translate(cx as f64, base as f64);
    p.c.scale(pulse as f64, pulse as f64);
    // 锚点 (ax, ay) 是 reanim 原点（脚底/贴地点）在单元格内的位置。
    draw_cell(p.c, &img, &clip, frame, -ch.ax * scale, -ch.ay * scale, dw, dh);
    if kind == PlantKind::WallNut && hp < 0.7 {
        let a = (0.7 - hp) / 0.7;
        p.line(&[(-dw*0.22, -dh*0.72), (-dw*0.05, -dh*0.5), (-dw*0.16, -dh*0.36)], 2.0, rgba(70,40,15, 0.55*a));
        p.line(&[(dw*0.18, -dh*0.3), (dw*0.02, -dh*0.18), (dw*0.12, -dh*0.05)], 2.2, rgba(70,40,15, 0.5*a));
        if hp < 0.35 {
            p.line(&[(-dw*0.3, -dh*0.4), (-dw*0.12, -dh*0.26)], 2.4, rgba(60,32,10, 0.6));
        }
    }
    p.restore();
    true
}

// 旧版 16 帧条带（Repeater 等没有 reanim 源的角色继续用）。
// 射击时用 anim_shooting，嘴部开合和头部后坐都来自原版轨道，而不是整张图平移。
fn draw_anim_plant(p: &P, kind: PlantKind, art: Art, cx: f32, base: f32, t: f32, hp: f32, fuse: f32, anim: f32) -> bool {
    let idx = art as usize;
    let shooting = anim > 0.0 && crate::plant_anim::SHOOT[idx].frames > 0;
    let clip = if shooting {
        crate::plant_anim::SHOOT[idx]
    } else {
        crate::plant_anim::IDLE[idx]
    };
    let Some(img) = assets::plant_sheet(art, shooting) else { return false };
    let idle = crate::plant_anim::IDLE[idx];
    if clip.frames == 0 || clip.w <= 0.0 || clip.h <= 0.0 || idle.h <= 0.0 {
        return false;
    }
    let h = art_height(art);
    let scale = h / idle.h;
    let dw = clip.w * scale;
    let dh = clip.h * scale;
    let n = clip.frames;
    let frame = if shooting {
        let dur = if art == Art::Starfruit { 0.16 } else { 0.20 };
        let progress = (1.0 - (anim / dur).clamp(0.0, 1.0)) * n as f32;
        (progress as u32).min(n - 1)
    } else {
        (((t + idx as f32 * 0.17) * 12.0) as u32) % n
    };
    let pulse = if kind == PlantKind::Cherry && fuse < 0.55 {
        1.0 + (t * 16.0).sin().abs() * 0.10
    } else {
        1.0
    };
    p.save();
    let yoff = if art == Art::PotatoMine { 8.0 } else { 0.0 };
    // 单元四周各有 4px 烘焙留白，把留白折算到地面，避免整株悬空。
    p.c.translate(cx as f64, (base + yoff + 4.0 * scale) as f64);
    p.c.scale(pulse as f64, pulse as f64);
    let sx = (frame * clip.w as u32) as f64;
    draw_img_rect(
        p.c,
        &img,
        sx,
        0.0,
        clip.w as f64,
        clip.h as f64,
        (-dw * 0.5) as f64,
        -dh as f64,
        dw as f64,
        dh as f64,
    );
    if kind == PlantKind::WallNut && hp < 0.7 {
        let a = (0.7 - hp) / 0.7;
        p.line(&[(-dw*0.22, -dh*0.72), (-dw*0.05, -dh*0.5), (-dw*0.16, -dh*0.36)], 2.0, rgba(70,40,15, 0.55*a));
        p.line(&[(dw*0.18, -dh*0.3), (dw*0.02, -dh*0.18), (dw*0.12, -dh*0.05)], 2.2, rgba(70,40,15, 0.5*a));
        if hp < 0.35 {
            p.line(&[(-dw*0.3, -dh*0.4), (-dw*0.12, -dh*0.26)], 2.4, rgba(60,32,10, 0.6));
        }
    }
    p.restore();
    true
}

fn draw_plant_anim(p: &P, kind: PlantKind, cx: f32, base: f32, t: f32, hp: f32, fuse: f32, anim: f32) {
    if let Some(art) = art_of(kind) {
        if draw_reanim_plant(p, kind, art, cx, base, t, hp, fuse, anim) {
            return;
        }
        if draw_anim_plant(p, kind, art, cx, base, t, hp, fuse, anim) {
            return;
        }
    }
    draw_plant_kind(p, kind, cx, base, t, hp, fuse, anim);
}

fn draw_art_plant(p: &P, kind: PlantKind, art: Art, cx: f32, base: f32, t: f32, hp: f32, fuse: f32, anim: f32) -> bool {
    let Some(img) = assets::image(art) else { return false };
    let ih = img.height() as f32;
    let iw = img.width() as f32;
    if ih <= 0.0 {
        return false;
    }
    let h = art_height(art);
    let w = h * iw / ih;
    let bob = (t * 2.1).sin() * 1.6;
    let sway = (t * 1.25).sin() * 0.012;
    let recoil = if matches!(kind, PlantKind::Peashooter | PlantKind::SnowPea | PlantKind::Repeater | PlantKind::Cabbage | PlantKind::Starfruit) {
        (anim / 0.20).clamp(0.0, 1.0) * 6.0
    } else {
        0.0
    };
    let pulse = if kind == PlantKind::Cherry && fuse < 0.55 {
        1.0 + (t * 16.0).sin().abs() * 0.10
    } else {
        1.0
    };
    p.save();
    let yoff = if art == Art::PotatoMine { 8.0 } else { 0.0 };
    p.c.translate((cx - recoil) as f64, (base + bob + yoff) as f64);
    p.c.rotate((sway - recoil * 0.008) as f64);
    p.c.scale(pulse as f64, pulse as f64);
    let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
        &img,
        (-w * 0.5) as f64,
        -h as f64,
        w as f64,
        h as f64,
    );
    // 坚果受伤：叠加裂纹提示
    if kind == PlantKind::WallNut && hp < 0.7 {
        let a = (0.7 - hp) / 0.7;
        p.line(&[(-w*0.22, -h*0.72), (-w*0.05, -h*0.5), (-w*0.16, -h*0.36)], 2.0, rgba(70,40,15, 0.55*a));
        p.line(&[(w*0.18, -h*0.3), (w*0.02, -h*0.18), (w*0.12, -h*0.05)], 2.2, rgba(70,40,15, 0.5*a));
        if hp < 0.35 {
            p.line(&[(-w*0.3, -h*0.4), (-w*0.12, -h*0.26)], 2.4, rgba(60,32,10, 0.6));
        }
    }
    p.restore();
    true
}

fn draw_sunflower(p: &P, cx: f32, base: f32, t: f32) {
    p.save();
    p.c.translate(cx as f64, base as f64);
    p.rr(-3.0, -42.0, 6.0, 44.0, 3.0, "#4e9b2c");
    p.ell(-15.0, -14.0, 13.0, 6.0, "#5db132");
    p.ell(14.0, -20.0, 12.0, 5.5, "#5db132");
    let hy = -60.0 + (t * 2.2).sin() * 2.0;
    for i in 0..12 {
        let a = i as f32 / 12.0 * TAU + (t * 0.2).sin() * 0.04;
        p.save();
        p.c.translate(0.0, hy as f64);
        p.c.rotate(a as f64);
        p.ell(0.0, -26.0, 8.0, 14.0, "#ffd23c");
        p.restore();
    }
    for i in 0..12 {
        let a = i as f32 / 12.0 * TAU + 0.26;
        p.save();
        p.c.translate(0.0, hy as f64);
        p.c.rotate(a as f64);
        p.ell(0.0, -24.0, 6.0, 11.0, "#ffe680");
        p.restore();
    }
    p.circle(0.0, hy, 15.0, "#a9631f");
    p.circle(0.0, hy, 12.0, "#c98a3d");
    p.circle(-6.0, hy - 2.0, 2.6, "#3a2410");
    p.circle(5.0, hy - 2.0, 2.6, "#3a2410");
    p.circle(-6.6, hy - 2.8, 1.0, "#fff6d8");
    p.circle(4.4, hy - 2.8, 1.0, "#fff6d8");
    p.arcw(0.0, hy + 5.0, 7.0, 0.15 * PI as f32, 0.85 * PI as f32, 2.0, "#5a3a18");
    p.restore();
}

fn draw_peashooter(p: &P, cx: f32, base: f32, t: f32, variant: f32) {
    // variant: 0 普通, 1 寒冰, 2 双发
    let (head, dark, snout, snout_dark, snout_col) = if variant > 0.5 && variant < 1.5 {
        ("#9ee7f3", "#6fc6d8", "#8adcec", "#63b8cb", "#c9f6ff")
    } else if variant >= 1.5 {
        ("#4aa32f", "#2f7d1e", "#3f9227", "#2c6c19", "#9be06a")
    } else {
        ("#5db63c", "#3f8f24", "#4fa734", "#367d1f", "#a8e878")
    };
    let breathe = (t * 2.4).sin() * 1.5;
    p.save();
    p.c.translate(cx as f64, base as f64);
    p.rr(-3.0, -38.0, 6.0, 40.0, 3.0, dark);
    p.ell(-15.0, -10.0, 13.0, 6.0, head);
    p.ell(14.0, -18.0, 11.0, 5.0, head);
    let hy = -48.0 + breathe;
    p.circle(1.0, hy, 19.0, dark);
    p.circle(2.0, hy - 1.0, 17.0, head);
    // 炮口
    p.rr(13.0, hy - 9.0, 20.0, 15.0, 7.0, snout_dark);
    p.rr(15.0, hy - 7.0, 18.0, 11.0, 5.0, snout);
    p.circle(33.0, hy - 1.5, 6.5, "#2f5f1c");
    p.circle(33.5, hy - 2.5, 3.2, snout_col);
    if variant >= 1.5 {
        p.rr(13.0, hy + 6.0, 20.0, 14.0, 7.0, snout_dark);
        p.rr(15.0, hy + 8.0, 18.0, 10.0, 5.0, snout);
        p.circle(33.0, hy + 12.5, 6.5, "#2f5f1c");
        p.circle(33.5, hy + 11.5, 3.2, snout_col);
    }
    // 眼睛
    p.ell(-3.0, hy - 6.0, 6.0, 5.2, "#f6ffee");
    p.circle(-5.0, hy - 6.0, 2.8, "#1d3a12");
    p.circle(-5.8, hy - 7.0, 1.0, "#ffffff");
    // 眉毛
    p.line(&[(-10.0, hy - 14.0), (-2.0, hy - 12.0)], 2.2, dark);
    // 冰霜点缀
    if variant > 0.5 && variant < 1.5 {
        p.circle(-12.0, hy - 12.0, 2.2, rgba(255, 255, 255, 0.75));
        p.circle(8.0, hy - 16.0, 1.8, rgba(255, 255, 255, 0.65));
        p.circle(22.0, hy + 12.0, 1.6, rgba(255, 255, 255, 0.6));
    }
    p.restore();
}

fn draw_wallnut(p: &P, cx: f32, base: f32, t: f32, hp: f32) {
    p.save();
    p.c.translate(cx as f64, base as f64);
    let sq = 1.0 + (t * 1.6).sin() * 0.02;
    p.c.scale(sq as f64, sq as f64);
    p.ell(0.0, -33.0, 28.0, 38.0, "#bd8047");
    p.ell(-8.0, -46.0, 15.0, 18.0, "#d59d61");
    p.ell(12.0, -20.0, 10.0, 12.0, "#a86d37");
    // 脸
    if hp > 0.45 {
        p.ell(-9.0, -38.0, 4.0, 5.6, "#3b2312");
        p.ell(9.0, -38.0, 4.0, 5.6, "#3b2312");
        p.circle(-10.0, -40.0, 1.2, "#ffffff");
        p.circle(8.0, -40.0, 1.2, "#ffffff");
        p.arcw(0.0, -25.0, 8.0, 0.2 * PI as f32, 0.8 * PI as f32, 2.0, "#6b4423");
    } else if hp > 0.15 {
        p.ell(-9.0, -38.0, 4.0, 5.6, "#3b2312");
        p.ell(9.0, -38.0, 4.0, 5.6, "#3b2312");
        p.line(&[(-13.0, -46.0), (-6.0, -44.0)], 2.0, "#5c3418");
        p.line(&[(13.0, -46.0), (6.0, -44.0)], 2.0, "#5c3418");
        p.line(&[(-7.0, -24.0), (0.0, -27.0), (7.0, -24.0)], 2.2, "#6b4423");
    } else {
        p.line(&[(-13.0, -40.0), (-5.0, -36.0)], 2.0, "#3b2312");
        p.line(&[(13.0, -40.0), (5.0, -36.0)], 2.0, "#3b2312");
        p.line(&[(-7.0, -22.0), (0.0, -26.0), (7.0, -22.0)], 2.2, "#6b4423");
    }
    // 裂纹
    if hp < 0.7 {
        p.line(&[(14.0, -66.0), (7.0, -54.0), (12.0, -46.0)], 2.0, "#7a4a22");
        p.line(&[(-18.0, -22.0), (-11.0, -14.0)], 2.0, "#7a4a22");
    }
    if hp < 0.4 {
        p.line(&[(-16.0, -60.0), (-8.0, -50.0), (-14.0, -40.0)], 2.4, "#6d3f1c");
        p.line(&[(20.0, -36.0), (11.0, -28.0), (18.0, -16.0)], 2.4, "#6d3f1c");
    }
    if hp < 0.2 {
        p.line(&[(-22.0, -38.0), (-13.0, -32.0)], 2.2, "#5e3517");
        p.line(&[(4.0, -66.0), (10.0, -58.0)], 2.2, "#5e3517");
    }
    p.restore();
}

fn draw_cherry(p: &P, cx: f32, base: f32, t: f32, fuse: f32) {
    let pulse = if fuse < 0.55 {
        1.0 + (t * 15.0).sin().abs() * 0.14
    } else {
        1.0
    };
    p.save();
    p.c.translate(cx as f64, base as f64);
    p.line(&[(-12.0, -36.0), (2.0, -58.0)], 3.0, "#4f8a2c");
    p.line(&[(10.0, -30.0), (2.0, -58.0)], 3.0, "#4f8a2c");
    p.ell(2.0, -58.0, 10.0, 4.0, "#63a83a");
    p.save();
    p.c.translate(0.0, -22.0 as f64);
    p.c.scale(pulse as f64, pulse as f64);
    p.circle(-12.0, 0.0, 18.0, "#c92f38");
    p.circle(11.0, 2.0, 16.0, "#e0453c");
    p.ell(-16.0, -7.0, 6.0, 8.0, rgba(255, 255, 255, 0.32));
    p.ell(7.0, -5.0, 4.0, 6.0, rgba(255, 255, 255, 0.22));
    p.circle(-15.0, -2.0, 2.4, "#ffffff");
    p.circle(-10.0, -1.0, 2.4, "#ffffff");
    p.circle(-15.0, -2.0, 1.2, "#3a1414");
    p.circle(-10.0, -1.0, 1.2, "#3a1414");
    p.arcw(-12.0, 7.0, 5.0, 0.1 * PI as f32, 0.9 * PI as f32, 1.8, "#5c1518");
    p.restore();
    // 引线火花
    let sp = (t * 22.0).sin() * 2.0;
    p.circle(2.0, -58.0 + sp, 5.0, rgba(255, 210, 70, 0.55));
    p.circle(2.0, -58.0 + sp, 2.6, "#fff3ad");
    p.restore();
}

fn draw_shovel_icon(p: &P, x: f32, y: f32, s: f32, rot: f32) {
    p.save();
    p.c.translate(x as f64, y as f64);
    p.c.rotate(rot as f64);
    p.c.scale(s as f64, s as f64);
    p.rr(-4.0, -30.0, 8.0, 44.0, 4.0, "#8a5a33");
    p.rr(-12.0, -34.0, 24.0, 7.0, 4.0, "#a06c3e");
    p.poly(
        &[(-12.0, 12.0), (12.0, 12.0), (8.0, 30.0), (-8.0, 30.0)],
        "#b9c4c9",
    );
    p.poly(&[(-8.0, 12.0), (0.0, 12.0), (-2.0, 28.0), (-8.0, 28.0)], "#d8dfe3");
    p.restore();
}

// ================= 僵尸绘制 =================

fn draw_zombie(p: &P, z: &Zombie, time: f32, h: f32) {
    let gy = row_ground(z.row, h) + 2.0;
    p.ell(z.x, gy + 8.0, 26.0, 8.0, rgba(20, 40, 10, 0.22));
    let target_h = match z.kind {
        ZombieKind::Gargantuar => 330.0,
        ZombieKind::Boss => 380.0,
        ZombieKind::Bungee => 300.0,
        ZombieKind::Imp => 150.0,
        ZombieKind::Zomboni | ZombieKind::Catapult => 290.0,
        ZombieKind::Bobsled | ZombieKind::DolphinRider => 250.0,
        _ => 236.0,
    };
    // 原版 reanim 图集优先：走路 / 啃食 / 倒地分别对应原版 clip。
    if let Some(ch) = crate::reanim::by_name(reanim_zombie_name(z.kind)) {
        let clip = if z.dying && ch.has(&ch.die) {
            ch.die
        } else if z.eating && ch.has(&ch.eat) {
            ch.eat
        } else {
            ch.base
        };
        if ch.has(&clip) && ch.h > 0.0 {
            if let Some(img) = assets::reanim_img(clip.slot) {
                let scale = target_h / ch.h;
                let dw = clip.w * scale;
                let dh = clip.h * scale;
                let frame = if z.dying {
                    let k = (z.death_t / 0.8).clamp(0.0, 1.0);
                    ((k * clip.frames as f32) as u32).min(clip.frames.saturating_sub(1) as u32)
                } else {
                    let n = clip.frames.max(1) as i32;
                    let phase = (time * clip.fps.max(1.0) + z.x * 0.045 + z.row as f32 * 1.3) as i32;
                    phase.rem_euclid(n) as u32
                };
                p.save();
                p.c.translate(z.x as f64, gy as f64);
                if z.dying {
                    let k = (z.death_t / 0.8).min(1.0);
                    p.c.rotate((-1.25 * k) as f64);
                    p.alpha(1.0 - k * 0.85);
                }
                let lean = if z.eating { -0.06 } else { 0.0 };
                p.c.rotate(lean as f64);
                draw_cell(p.c, &img, &clip, frame, -ch.ax * scale, -ch.ay * scale, dw, dh);
                if z.slow > 0.0 {
                    p.rect(-dw * 0.42, -dh * 0.95, dw * 0.84, dh * 0.9, rgba(120, 210, 245, 0.20));
                }
                if z.flash > 0.0 {
                    p.save();
                    p.alpha((z.flash * 0.38).min(0.38));
                    p.rect(-dw * 0.42, -dh * 0.95, dw * 0.84, dh * 0.9, "#ffffff");
                    p.restore();
                    p.alpha(if z.dying { (1.0 - (z.death_t / 0.8).min(1.0) * 0.85).max(0.0) } else { 1.0 });
                }
                p.restore();
                p.alpha(1.0);
                return;
            }
        }
    }
    let frame = (((time * 12.0 + z.x * 0.045 + z.row as f32 * 1.3) as i32).rem_euclid(16)) as u8;
    if let Some(img) = assets::zombie(z.kind as u8, frame) {
        p.save();
        p.c.translate(z.x as f64, gy as f64);
        if z.dying {
            let k = (z.death_t / 0.8).min(1.0);
            p.c.rotate((-1.25 * k) as f64);
            p.alpha(1.0 - k * 0.85);
        }
        let walk = (time * 5.6 + z.row as f32 * 1.7 + z.x * 0.02).sin();
        let bob = walk * 1.4;
        let lean = if z.eating { -0.05 } else { 0.0 };
        p.c.rotate(lean as f64);
        let scale = target_h / img.height().max(1) as f32;
        let w = img.width() as f32 * scale;
        let h = target_h;
        let _ = p.c.draw_image_with_html_image_element_and_dw_and_dh(
            &img,
            (-w * 0.5) as f64,
            (-h + bob) as f64,
            w as f64,
            h as f64,
        );
        if z.slow > 0.0 {
            p.rect(-w * 0.42, -h + 8.0, w * 0.84, h - 12.0, rgba(120, 210, 245, 0.22));
        }
        if z.flash > 0.0 {
            p.save();
            p.alpha((z.flash * 0.38).min(0.38));
            p.rect(-w * 0.42, -h + 8.0, w * 0.84, h - 12.0, "#ffffff");
            p.restore();
            p.alpha(if z.dying { (1.0 - (z.death_t / 0.8).min(1.0) * 0.85).max(0.0) } else { 1.0 });
        }
        p.restore();
        p.alpha(1.0);
        return;
    }

    p.save();
    p.c.translate(z.x as f64, gy as f64);
    if z.dying {
        let k = (z.death_t / 0.8).min(1.0);
        p.c.rotate((-1.15 * k) as f64);
        p.alpha(1.0 - k * 0.85);
    }
    let t = z.t.max(time * 0.0);
    if matches!(z.kind, ZombieKind::Gargantuar) {
        let _ = p.c.scale(1.45, 1.45);
    } else if matches!(z.kind, ZombieKind::Imp) {
        let _ = p.c.scale(0.68, 0.68);
    } else if matches!(z.kind, ZombieKind::Boss) {
        let _ = p.c.scale(2.15, 2.15);
    }
    let walk = (t * 3.2).sin();
    let eat_bob = if z.eating { (t * 9.0).sin() * 2.0 } else { 0.0 };

    // 腿
    p.rr(-12.0 + walk * 3.0, -30.0, 11.0, 30.0, 3.0, "#3c434b");
    p.rr(4.0 - walk * 3.0, -30.0, 11.0, 30.0, 3.0, "#4a525b");
    p.rr(-14.0 + walk * 3.0, -6.0, 16.0, 8.0, 3.0, "#2f343a");
    p.rr(2.0 - walk * 3.0, -6.0, 16.0, 8.0, 3.0, "#2f343a");

    // 身体
    p.rr(-17.0, -70.0 + eat_bob * 0.3, 34.0, 44.0, 10.0, "#59676f");
    p.poly(
        &[
            (-17.0, -34.0),
            (-9.0, -40.0),
            (-3.0, -33.0),
            (3.0, -40.0),
            (10.0, -33.0),
            (17.0, -38.0),
            (17.0, -28.0),
            (-17.0, -28.0),
        ],
        "#4c5960",
    );
    p.rect(-17.0, -52.0, 34.0, 4.0, rgba(20, 26, 30, 0.35));

    // 后手臂
    p.rr(-30.0, -60.0, 26.0, 9.0, 4.0, "#4b5760");
    p.circle(-31.0, -55.5, 6.0, "#8ba968");

    // 头部
    p.circle(0.0, -88.0 + eat_bob, 17.0, "#a7bb8a");
    p.ell(3.0, -74.0 + eat_bob, 11.0, 6.0, "#93a878");
    // 眼窝
    p.ell(-6.0, -92.0 + eat_bob, 5.0, 4.2, "#eef4df");
    p.circle(-8.0, -92.0 + eat_bob, 2.2, "#20291a");
    p.ell(4.0, -93.0 + eat_bob, 5.0, 4.2, "#eef4df");
    p.circle(2.0, -93.0 + eat_bob, 2.2, "#20291a");
    // 眉骨
    p.line(&[(-13.0, -98.0 + eat_bob), (-3.0, -96.0 + eat_bob)], 2.0, "#5f6f4c");
    // 嘴
    let mo = if z.eating {
        4.5 + (t * 10.0).sin().abs() * 3.0
    } else {
        2.5
    };
    p.ell(-2.0, -78.0 + eat_bob, 7.0, mo, "#5b2323");
    p.rect(-7.0, -81.0 + eat_bob, 3.0, 2.0, "#e8e4cf");
    p.rect(-2.0, -81.0 + eat_bob, 3.0, 2.0, "#e8e4cf");
    // 头发
    p.line(&[(-10.0, -102.0 + eat_bob), (-7.0, -108.0 + eat_bob)], 2.2, "#4d5c3a");
    p.line(&[(-3.0, -103.0 + eat_bob), (-1.0, -109.0 + eat_bob)], 2.2, "#4d5c3a");
    p.line(&[(4.0, -102.0 + eat_bob), (6.0, -107.0 + eat_bob)], 2.2, "#4d5c3a");

    // 前手臂
    p.rr(-40.0, -52.0, 30.0, 10.0, 5.0, "#59676f");
    p.circle(-41.0, -47.0, 6.5, "#93b070");
    p.line(&[(-38.0, -49.0), (-20.0, -48.0)], 1.5, rgba(30, 40, 35, 0.3));

    // 帽子/道具
    match z.kind {
        ZombieKind::Cone => {
            p.poly(
                &[(-19.0, -102.0 + eat_bob), (19.0, -102.0 + eat_bob), (1.0, -140.0 + eat_bob)],
                "#df7526",
            );
            p.poly(
                &[(-13.0, -114.0 + eat_bob), (15.0, -114.0 + eat_bob), (11.0, -122.0 + eat_bob), (-9.0, -122.0 + eat_bob)],
                "#f5ead4",
            );
            p.poly(
                &[(-19.0, -102.0 + eat_bob), (19.0, -102.0 + eat_bob), (17.0, -98.0 + eat_bob), (-17.0, -98.0 + eat_bob)],
                "#b95a19",
            );
        }
        ZombieKind::Bucket => {
            p.rr(-21.0, -132.0 + eat_bob, 40.0, 32.0, 3.0, "#98a3a7");
            p.rect(-21.0, -110.0 + eat_bob, 40.0, 5.0, "#7f8b90");
            p.ell(0.0, -132.0 + eat_bob, 21.0, 5.0, "#bcc6c9");
            p.ell(0.0, -133.0 + eat_bob, 19.0, 3.6, "#6d7a7f");
        }
        ZombieKind::Flag => {
            p.line(&[(22.0, -70.0), (22.0, -150.0)], 3.0, "#6b4a2a");
            let wav = (t * 6.0).sin() * 4.0;
            p.poly(
                &[(24.0, -148.0), (52.0, -140.0 + wav), (24.0, -130.0)],
                "#d63c3c",
            );
            p.circle(24.0, -150.0, 3.0, "#e9c46a");
        }
        ZombieKind::Ladder => {
            p.line(&[(-42.0, -8.0), (-28.0, -126.0)], 6.0, "#c8d0d3");
            p.line(&[(-30.0, -8.0), (-16.0, -126.0)], 6.0, "#c8d0d3");
            for k in 0..6 {
                let y = -18.0 - k as f32 * 20.0;
                p.line(&[(-40.0, y), (-18.0, y)], 3.0, "#aeb7ba");
            }
        }
        ZombieKind::PoleVault => {
            p.line(&[(26.0, -6.0), (26.0, -120.0)], 4.0, "#b98b4a");
            p.line(&[(-34.0, -58.0), (30.0, -58.0)], 2.0, "#7d5a2e");
            p.rr(-19.0, -104.0 + eat_bob, 38.0, 12.0, 5.0, "#e4e8ea");
            p.rr(-14.0, -78.0 + eat_bob, 28.0, 5.0, 2.0, "#2f6da8");
        }
        ZombieKind::ScreenDoor => {
            p.save();
            p.c.translate(-34.0, -84.0);
            p.rr(0.0, 0.0, 22.0, 78.0, 3.0, "#9b6a3a");
            p.rr_s(0.0, 0.0, 22.0, 78.0, 3.0, 2.0, "#6b4524");
            p.rr(4.0, 8.0, 14.0, 30.0, 2.0, rgba(180, 220, 235, 0.35));
            p.circle(17.0, 44.0, 2.5, "#e9c46a");
            p.restore();
            p.ell(-8.0, -88.0 + eat_bob, 13.0, 3.0, "#7f8b90");
        }
        ZombieKind::Football => {
            p.rr(-22.0, -116.0 + eat_bob, 44.0, 24.0, 11.0, "#b02a2a");
            p.rect(-22.0, -104.0 + eat_bob, 44.0, 4.0, "#7d1b1b");
            p.line(&[(-16.0, -108.0 + eat_bob), (16.0, -108.0 + eat_bob)], 2.0, "#e3c15a");
            p.rr(-20.0, -72.0, 40.0, 42.0, 8.0, "#7d1b1b");
            p.rect(-20.0, -56.0, 40.0, 5.0, "#e3c15a");
        }
        ZombieKind::Newspaper if z.phase == 0 => {
            p.rr(16.0, -86.0 + eat_bob, 16.0, 46.0, 3.0, "#efe7d3");
            for k in 0..5 {
                p.rect(18.0, -80.0 + eat_bob + k as f32 * 8.0, 12.0, 2.0, "#a9a292");
            }
        }
        ZombieKind::Dancing | ZombieKind::BackupDancer => {
            p.rr(-18.0, -70.0, 36.0, 44.0, 8.0, "#5a3a7a");
            p.rect(-18.0, -54.0, 36.0, 4.0, "#d3b45c");
            p.circle(0.0, -102.0 + eat_bob, 18.0, "#c98b4e");
            p.rr(-20.0, -113.0 + eat_bob, 40.0, 9.0, 4.0, "#d26b2b");
            p.rr(-22.0, -59.0, 20.0, 8.0, 4.0, "#6d4a8f");
            p.rr(12.0, -59.0, 20.0, 8.0, 4.0, "#6d4a8f");
        }
        ZombieKind::DuckyTube => {
            p.ell(0.0, -34.0, 28.0, 12.0, "#e4c33d");
            p.ell(0.0, -36.0, 18.0, 7.0, "#f7e27a");
            p.circle(24.0, -38.0, 6.0, "#e4c33d");
        }
        ZombieKind::Snorkel => {
            p.rr(-14.0, -100.0 + eat_bob, 28.0, 14.0, 6.0, "#2f6f99");
            p.rect(-11.0, -98.0 + eat_bob, 22.0, 5.0, "#bfe8f2");
            p.line(&[(18.0, -94.0), (25.0, -124.0), (20.0, -131.0)], 4.0, "#e8d35a");
        }
        ZombieKind::Zomboni => {
            p.rr(-34.0, -44.0, 68.0, 40.0, 7.0, "#b8402f");
            p.rect(-34.0, -22.0, 68.0, 5.0, "#7e241a");
            p.circle(-20.0, -4.0, 10.0, "#2f343a");
            p.circle(20.0, -4.0, 10.0, "#2f343a");
            p.rr(34.0, -38.0, 18.0, 26.0, 3.0, "#9aa2a6");
            p.poly(&[(52.0, -38.0), (74.0, -20.0), (52.0, -6.0)], "#c9d2d5");
        }
        ZombieKind::Bobsled => {
            p.rr(-38.0, -12.0, 76.0, 14.0, 5.0, "#356f9e");
            p.line(&[(-34.0, -2.0), (34.0, -2.0)], 4.0, "#9fb6c4");
            p.rr(-28.0, -38.0, 56.0, 24.0, 5.0, "#4c89b8");
        }
        ZombieKind::DolphinRider => {
            p.ell(0.0, -8.0, 34.0, 12.0, "#6f9fb4");
            p.poly(&[(30.0, -8.0), (52.0, -18.0), (44.0, -2.0)], "#6f9fb4");
            p.circle(42.0, -12.0, 2.0, "#1f2a30");
        }
        ZombieKind::Jackbox => {
            p.rr(-30.0, -76.0, 24.0, 34.0, 4.0, "#c9a03e");
            p.rr(-27.0, -70.0, 18.0, 24.0, 3.0, "#7b2f2f");
            p.line(&[(-28.0, -76.0), (-38.0, -94.0), (-32.0, -108.0)], 3.0, "#d9d2bd");
            p.circle(-32.0, -110.0, 5.0, "#e6be44");
        }
        ZombieKind::Balloon if z.hp > z.max_hp * 0.55 => {
            p.line(&[(0.0, -105.0), (10.0, -150.0)], 2.0, "#8b8b8b");
            p.ell(10.0, -172.0, 22.0, 28.0, "#d94a4a");
            p.poly(&[(4.0, -206.0), (16.0, -206.0), (10.0, -214.0)], "#b93232");
        }
        ZombieKind::Digger => {
            p.ell(0.0, -8.0, 28.0, 10.0, "#6b4a2a");
            p.rr(-18.0, -42.0, 36.0, 14.0, 4.0, "#5b4632");
            p.circle(-14.0, -46.0, 4.0, "#f0d25a");
            p.line(&[(14.0, -44.0), (34.0, -20.0)], 5.0, "#8c8c8c");
        }
        ZombieKind::Pogo => {
            p.line(&[(0.0, -30.0), (0.0, 4.0)], 4.0, "#9aa2a6");
            p.rr(-10.0, -40.0, 20.0, 8.0, 3.0, "#c9d2d5");
            p.line(&[(-8.0, 4.0), (8.0, 4.0)], 3.0, "#6d7377");
        }
        ZombieKind::Yeti => {
            p.circle(0.0, -88.0 + eat_bob, 21.0, "#e8eef0");
            p.rr(-22.0, -72.0, 44.0, 40.0, 12.0, "#dfe7ea");
            p.circle(-7.0, -92.0 + eat_bob, 3.0, "#2b3036");
            p.circle(6.0, -92.0 + eat_bob, 3.0, "#2b3036");
        }
        ZombieKind::Bungee => {
            p.line(&[(0.0, -110.0), (0.0, -210.0)], 3.0, "#d9d2bd");
            p.rr(-26.0, -100.0, 52.0, 8.0, 4.0, "#7a3f2f");
        }
        ZombieKind::Catapult => {
            p.rr(-38.0, -34.0, 76.0, 12.0, 4.0, "#8b5a2b");
            p.line(&[(-30.0, -30.0), (0.0, -110.0), (30.0, -30.0)], 5.0, "#a9743c");
            p.rr(-8.0, -116.0, 18.0, 12.0, 3.0, "#6f8f3e");
        }
        ZombieKind::Gargantuar => {
            p.rr(28.0, -120.0, 10.0, 120.0, 4.0, "#7a5a34");
            p.circle(33.0, -126.0, 7.0, "#8b6a3f");
        }
        ZombieKind::Imp => {
            p.circle(0.0, -102.0 + eat_bob, 19.0, "#c7a06a");
            p.rr(-16.0, -70.0, 32.0, 40.0, 8.0, "#7a5a3a");
        }
        ZombieKind::Boss => {
            p.rr(-52.0, -60.0, 104.0, 58.0, 10.0, "#3c4652");
            p.rr(-44.0, -114.0, 88.0, 60.0, 10.0, "#59676f");
            p.circle(-18.0, -112.0, 5.0, "#f05a4a");
            p.circle(18.0, -112.0, 5.0, "#f05a4a");
            p.rect(-52.0, -62.0, 104.0, 8.0, "#9aa2a6");
        }
        ZombieKind::Newspaper | ZombieKind::Balloon => {}
        ZombieKind::Basic => {}
    }

    // 减速蓝光
    if z.slow > 0.0 {
        p.alpha(0.22);
        p.circle(0.0, -60.0, 30.0, "#7fd8ea");
        p.circle(0.0, -88.0, 18.0, "#9fe8f7");
        p.alpha(1.0);
    }
    // 受击闪白
    if z.flash > 0.0 {
        p.alpha(z.flash * 0.35);
        p.circle(0.0, -60.0, 30.0, "#ffffff");
        p.alpha(1.0);
    }
    p.restore();
    p.alpha(1.0);
}

// ================= 阳光 / 割草机 / 粒子 =================

fn draw_sun(p: &P, s: &Sun, time: f32) {
    let bob = if s.state == SunState::Idle {
        (time * 2.6 + s.t).sin() * 2.0
    } else {
        0.0
    };
    let y = s.y + bob;
    let a = if s.state == SunState::Idle && s.life < 1.5 {
        (s.life / 1.5).clamp(0.0, 1.0)
    } else {
        1.0
    };
    p.save();
    p.alpha(a);
    p.circle(s.x, y, 30.0, rgba(255, 226, 90, 0.16));
    p.circle(s.x, y, 23.0, rgba(255, 226, 90, 0.28));
    for i in 0..8 {
        let ang = i as f32 / 8.0 * TAU + time * 0.6;
        let r0 = 19.0;
        let r1 = 27.0;
        p.line(
            &[
                (s.x + ang.cos() * r0, y + ang.sin() * r0),
                (s.x + ang.cos() * r1, y + ang.sin() * r1),
            ],
            3.0,
            rgba(255, 213, 60, 0.8),
        );
    }
    p.circle(s.x, y, 16.0, "#ffcf3f");
    p.circle(s.x, y, 13.0, "#ffdf6b");
    p.circle(s.x - 4.0, y - 4.0, 5.0, "#fff4c4");
    p.restore();
    p.alpha(1.0);
}

fn draw_mower(p: &P, m: &Mower, time: f32, h: f32) {
    let gy = row_ground(m.row, h) + 6.0;
    let x = if m.running { m.x } else { MOWER_X };
    if m.used && !m.running {
        return;
    }
    p.ell(x, gy, 30.0, 8.0, rgba(20, 40, 10, 0.25));
    // 车体
    p.rr(x - 20.0, gy - 28.0, 42.0, 22.0, 6.0, "#c53c2b");
    p.rr(x - 20.0, gy - 28.0, 42.0, 8.0, 5.0, "#e05540");
    p.rr(x - 12.0, gy - 34.0, 14.0, 8.0, 3.0, "#8f2a1e");
    // 手柄
    p.line(&[(x - 20.0, gy - 24.0), (x - 34.0, gy - 40.0)], 4.0, "#4a4f55");
    p.circle(x - 34.0, gy - 40.0, 3.5, "#2f3438");
    // 轮子
    p.circle(x - 12.0, gy - 3.0, 8.0, "#2f3438");
    p.circle(x + 12.0, gy - 3.0, 8.0, "#2f3438");
    p.circle(x - 12.0, gy - 3.0, 3.0, "#8b939a");
    p.circle(x + 12.0, gy - 3.0, 3.0, "#8b939a");
    // 滚刀
    let spin = if m.running { time * 22.0 } else { time * 0.4 };
    p.rr(x + 18.0, gy - 20.0, 8.0, 16.0, 3.0, "#9aa3a8");
    for i in 0..4 {
        let a = spin + i as f32 * PI as f32 * 0.5;
        let cx = x + 24.0;
        let cy = gy - 12.0;
        p.line(
            &[
                (cx - a.cos() * 8.0, cy - a.sin() * 8.0),
                (cx + a.cos() * 8.0, cy + a.sin() * 8.0),
            ],
            2.0,
            if i % 2 == 0 { "#d9e0e4" } else { "#8b949a" },
        );
    }
}

fn draw_particle(p: &P, pt: &Particle) {
    let a = (pt.life / pt.max).clamp(0.0, 1.0);
    match &pt.kind {
        PKind::Dot => {
            p.alpha(a);
            p.circle(pt.x, pt.y, pt.size * (0.4 + a * 0.6), &rgba(pt.r, pt.g, pt.b, a));
            p.alpha(1.0);
        }
        PKind::Leaf => {
            p.alpha(a);
            p.save();
            p.c.translate(pt.x as f64, pt.y as f64);
            p.c.rotate((pt.life * 6.0) as f64);
            p.ell(0.0, 0.0, pt.size, pt.size * 0.55, &rgba(pt.r, pt.g, pt.b, a));
            p.restore();
            p.alpha(1.0);
        }
        PKind::Ring => {
            let grow = 1.0 + (1.0 - a) * 2.4;
            p.alpha(a * 0.65);
            p.circle(pt.x, pt.y, pt.size * grow, &rgba(pt.r, pt.g, pt.b, a * 0.22));
            p.arf_ring(pt.x, pt.y, pt.size * grow, &rgba(pt.r, pt.g, pt.b, a * 0.75));
            p.alpha(1.0);
        }
        PKind::Text(txt) => {
            p.alpha(a);
            p.text(txt, pt.x, pt.y, pt.size, &rgba(pt.r, pt.g, pt.b, a), "center", true);
            p.alpha(1.0);
        }
    }
}

impl<'a> P<'a> {
    fn arf_ring(&self, x: f32, y: f32, r: f32, col: impl AsRef<str>) {
        let _ = self.c.set_stroke_style_str(col.as_ref());
        let _ = self.c.set_line_width(2.0);
        let _ = self.c.begin_path();
        let _ = self.c.arc(x as f64, y as f64, r.max(0.1) as f64, 0.0, TAU as f64);
        let _ = self.c.stroke();
    }
}

// ================= HUD =================

fn draw_hud(p: &P, g: &Game) {
    // 顶栏
    p.rect(0.0, 0.0, W, HUD_H, rgba(26, 48, 18, 0.93));
    p.rect(0.0, HUD_H - 3.0, W, 3.0, rgba(233, 196, 106, 0.55));
    p.rect(0.0, 0.0, W, 4.0, rgba(0, 0, 0, 0.25));

    // 阳光计数
    p.rr(14.0, 12.0, 120.0, 76.0, 10.0, "#f6ecd3");
    p.rr_s(14.0, 12.0, 120.0, 76.0, 10.0, 2.0, "#c8b184");
    let pulse = 1.0 + g.sun_pop * 0.18;
    p.save();
    p.c.translate(50.0 as f64, 50.0 as f64);
    p.c.scale(pulse as f64, pulse as f64);
    p.circle(0.0, 0.0, 22.0, rgba(255, 210, 60, 0.18));
    p.circle(0.0, 0.0, 17.0, "#ffcf3f");
    p.circle(-4.0, -4.0, 5.0, "#fff3bf");
    for i in 0..8 {
        let a = i as f32 / 8.0 * TAU;
        p.line(
            &[(a.cos() * 18.0, a.sin() * 18.0), (a.cos() * 23.0, a.sin() * 23.0)],
            2.5,
            "#ffcf3f",
        );
    }
    p.restore();
    p.text(&format!("{}", g.sun), 122.0, 52.0, 26.0, "#4a3010", "right", true);

    // 卡片：只画本关已选种子
    for i in 0..g.seeds.len() {
        draw_card(p, g, i);
    }
    draw_shovel_card(p, g);

    // 波次面板
    let (wx, wy, ww, wh) = WAVE_PANEL;
    p.rr(wx, wy, ww, wh, 10.0, rgba(16, 34, 12, 0.82));
    p.rr_s(wx, wy, ww, wh, 10.0, 2.0, rgba(233, 196, 106, 0.35));
    let label = if g.wave == 0 {
        "僵尸即将到来".to_string()
    } else {
        format!("第 {} / {} 波", g.wave, g.cfg.waves)
    };
    p.text(&label, wx + 10.0, wy + 20.0, 14.0, "#ffe8ad", "left", true);
    p.text(
        &format!("击杀 {}", g.kills),
        wx + ww - 10.0,
        wy + 20.0,
        11.0,
        "#c9e3a4",
        "right",
        false,
    );
    p.rr(wx + 12.0, wy + 42.0, ww - 24.0, 12.0, 6.0, "#0c1c09");
    let prog = (g.wave as f32 / g.cfg.waves as f32).clamp(0.0, 1.0);
    if prog > 0.0 {
        p.rr(wx + 12.0, wy + 42.0, (ww - 24.0) * prog, 12.0, 6.0, "#e9c46a");
        p.rr(wx + 12.0, wy + 42.0, (ww - 24.0) * prog, 5.0, 3.0, "#ffe9ad");
    }
    p.text(
        &format!("{}", time_str(g.elapsed)),
        wx + 12.0,
        wy + 68.0,
        12.0,
        "#a9c786",
        "left",
        false,
    );

    // 按钮
    draw_btn(p, BTN_PAUSE, g.mouse, |pp| {
        if g.state == State::Paused {
            pp.poly(&[(18.0, 14.0), (18.0, 32.0), (33.0, 23.0)], "#f6f0da");
        } else {
            pp.rect(16.0, 14.0, 6.0, 18.0, "#f6f0da");
            pp.rect(27.0, 14.0, 6.0, 18.0, "#f6f0da");
        }
    });
    draw_btn(p, BTN_RESTART, g.mouse, |pp| {
        pp.arcw(24.0, 24.0, 11.0, -0.4 * PI as f32, 1.3 * PI as f32, 3.0, "#f6f0da");
        pp.poly(&[(34.0, 10.0), (38.0, 20.0), (28.0, 19.0)], "#f6f0da");
    });
    draw_btn(p, BTN_SOUND, g.mouse, |pp| {
        pp.poly(
            &[(14.0, 19.0), (21.0, 19.0), (28.0, 12.0), (28.0, 34.0), (21.0, 27.0), (14.0, 27.0)],
            "#f6f0da",
        );
        if g.muted {
            pp.line(&[(31.0, 15.0), (39.0, 31.0)], 2.5, "#ff9d8a");
            pp.line(&[(39.0, 15.0), (31.0, 31.0)], 2.5, "#ff9d8a");
        } else {
            pp.arcw(30.0, 23.0, 8.0, -0.35 * PI as f32, 0.35 * PI as f32, 2.0, "#f6f0da");
            pp.arcw(30.0, 23.0, 12.0, -0.35 * PI as f32, 0.35 * PI as f32, 2.0, rgba(246, 240, 218, 0.7));
        }
    });

    // 底部提示
    let hint = if let Some(k) = g.selected {
        format!("已选择 {}（{} 阳光）· 点击草坪空格种下", k.name(), k.cost())
    } else if g.shovel {
        "铲子已选中 · 点击植物即可移除".to_string()
    } else {
        "点击卡片选择植物，点击阳光收集 · 右键取消选择".to_string()
    };
    p.text(&hint, 320.0, 650.0, 15.0, rgba(20, 40, 10, 0.75), "left", false);
    p.text(
        &format!("植物 {} · 用时 {}", g.planted, time_str(g.elapsed)),
        W - 16.0,
        650.0,
        14.0,
        rgba(20, 40, 10, 0.7),
        "right",
        false,
    );

    // 卡片悬停提示
    if let Some(i) = hovered_card(g) {
        let k = g.seeds[i];
        let (cx, cy, _, _) = card_rect(i);
        let w = 240.0;
        let x = (cx + CARD_W * 0.5 - w * 0.5).clamp(8.0, W - w - 8.0);
        p.rr(x, HUD_H + 8.0, w, 78.0, 10.0, rgba(18, 36, 12, 0.92));
        p.rr_s(x, HUD_H + 8.0, w, 78.0, 10.0, 1.0, rgba(255, 230, 140, 0.4));
        p.text(k.name(), x + 12.0, HUD_H + 28.0, 17.0, "#ffe8a8", "left", true);
        p.text(
            &format!("{} 阳光 · 冷却 {:.0}s", k.cost(), k.cooldown()),
            x + w - 12.0,
            HUD_H + 28.0,
            12.0,
            "#b7d596",
            "right",
            false,
        );
        p.text(k.desc(), x + 12.0, HUD_H + 55.0, 13.0, "#dfecc9", "left", false);
        let _ = cy;
    }
}

fn hovered_card(g: &Game) -> Option<usize> {
    for i in 0..g.seeds.len() {
        if rect_contains(card_rect(i), g.mouse.0, g.mouse.1) {
            return Some(i);
        }
    }
    None
}

fn draw_card(p: &P, g: &Game, i: usize) {
    let Some(&k) = g.seeds.get(i) else { return };
    let (x, y, w, h) = card_rect(i);
    let selected = g.selected == Some(k);
    let cd = g.cooldowns[k.idx()];
    let afford = g.sun >= k.cost();
    let hover = rect_contains((x, y, w, h), g.mouse.0, g.mouse.1);

    p.rr(x + 2.0, y + 3.0, w, h, 10.0, rgba(0, 0, 0, 0.28));
    p.rr(
        x,
        y,
        w,
        h,
        10.0,
        if selected {
            "#fff1b8"
        } else if hover {
            "#f7ecd0"
        } else {
            "#efe2bc"
        },
    );
    p.rr_s(
        x,
        y,
        w,
        h,
        10.0,
        if selected { 3.0 } else { 2.0 },
        if selected {
            "#ffd447"
        } else {
            "#a4814c"
        },
    );
    // 迷你植物
    p.save();
    if !afford && cd <= 0.0 {
        p.alpha(0.55);
    }
    p.c.translate((x + w * 0.5) as f64, (y + 57.0) as f64);
    let cs = if k.is_base() { 0.60 } else { 0.34 };
    p.c.scale(cs, cs);
    draw_plant_kind(p, k, 0.0, 0.0, g.time, 1.0, 1.0, 0.0);
    p.restore();
    p.alpha(1.0);

    // 名称与价格
    p.text(k.name(), x + w * 0.5, y + 63.0, 10.0, "#4d3312", "center", false);
    p.rr(x + w - 30.0, y + 4.0, 26.0, 17.0, 8.0, rgba(90, 60, 20, 0.14));
    p.text(
        &format!("{}", k.cost()),
        x + w - 17.0,
        y + 13.0,
        12.0,
        if afford { "#4d3312" } else { "#a33b2a" },
        "center",
        true,
    );

    // 冷却遮罩
    if cd > 0.0 {
        let total = k.cooldown();
        let frac = (cd / total).clamp(0.0, 1.0);
        let fh = h * frac;
        p.rr(x, y + h - fh, w, fh, 8.0, rgba(20, 28, 18, 0.52));
        p.text(&format!("{:.1}", cd), x + w * 0.5, y + h * 0.5, 18.0, "#ffffff", "center", true);
    } else if !afford {
        p.rr(x, y, w, h, 10.0, rgba(20, 28, 18, 0.32));
    }
}

fn draw_shovel_card(p: &P, g: &Game) {
    let (x, y, w, h) = (SHOVEL_X, SHOVEL_Y, SHOVEL_W, SHOVEL_H);
    let hover = rect_contains((x, y, w, h), g.mouse.0, g.mouse.1);
    let sel = g.shovel;
    p.rr(x + 2.0, y + 3.0, w, h, 10.0, rgba(0, 0, 0, 0.28));
    p.rr(
        x,
        y,
        w,
        h,
        10.0,
        if sel {
            "#fff1b8"
        } else if hover {
            "#f7ecd0"
        } else {
            "#efe2bc"
        },
    );
    p.rr_s(
        x,
        y,
        w,
        h,
        10.0,
        if sel { 3.0 } else { 2.0 },
        if sel { "#ffd447" } else { "#a4814c" },
    );
    draw_shovel_icon(p, x + w * 0.5, y + 40.0, 0.95, -0.28);
}

fn draw_btn(p: &P, r: (f32, f32, f32, f32), mouse: (f32, f32), f: impl Fn(&P)) {
    let (x, y, w, h) = r;
    let hover = rect_contains(r, mouse.0, mouse.1);
    p.rr(x, y, w, h, 8.0, if hover { "#3d6a28" } else { "#2a4a1c" });
    p.rr_s(x, y, w, h, 8.0, 1.5, rgba(233, 196, 106, 0.4));
    p.save();
    p.c.translate(x as f64, y as f64);
    f(p);
    p.restore();
}

fn time_str(sec: f32) -> String {
    let s = sec as u32;
    format!("{:02}:{:02}", s / 60, s % 60)
}

// ================= 菜单与结算 =================

fn draw_chooser(p: &P, g: &Game) {
    // 草坪背景
    p.rect(0.0, 0.0, W, H, "#315f1d");
    for col in 0..COLS {
        let c = if col % 2 == 0 { "#4c8a2c" } else { "#427d25" };
        p.rect(GRID_X + col as f32 * CELL_W, 0.0, CELL_W, H, c);
    }
    p.rect(0.0, 0.0, GRID_X, H, "#203d14");
    p.rect(GRID_X + COLS as f32 * CELL_W, 0.0, W, H, "#1c3511");
    p.rect(0.0, 0.0, W, H, rgba(8, 22, 6, 0.58));

    // 返回
    let back = (12.0, 12.0, 92.0, 40.0);
    let bh = rect_contains(back, g.mouse.0, g.mouse.1);
    p.rr(back.0, back.1, back.2, back.3, 10.0, if bh { "#6f9a41".to_string() } else { rgba(20, 38, 12, 0.82) });
    p.rr_s(back.0, back.1, back.2, back.3, 10.0, 1.5, rgba(220, 240, 180, 0.45));
    p.text("‹ 返回", back.0 + 46.0, back.1 + 26.0, 17.0, "#edf7dc", "center", false);

    p.text_out("选 择 种 子", W * 0.5, 54.0, 44.0, "#ffe14d", "#2a1804", "center", true);
    p.text(
        &format!("{} · {}", g.cfg.name, g.cfg.seed_hint),
        W * 0.5,
        88.0,
        16.0,
        "#dcefc4",
        "center",
        false,
    );

    // 已选槽位
    p.text(
        &format!("已带 {}/{}", g.seeds.len(), g.cfg.slots),
        246.0,
        214.0,
        16.0,
        if g.seeds.len() == g.cfg.slots { "#ffe08a" } else { "#cfe6b2" },
        "left",
        true,
    );
    for i in 0..g.cfg.slots {
        let (x, y, w, h) = seed_slot_rect(i);
        let filled = g.seeds.get(i).copied();
        p.rr(x, y, w, h, 9.0, rgba(20, 40, 14, 0.72));
        p.rr_s(x, y, w, h, 9.0, 1.5, if filled.is_some() { "#e9c46a".to_string() } else { rgba(210, 230, 180, 0.28) });
        if let Some(k) = filled {
            p.save();
            p.c.translate((x + w * 0.5) as f64, (y + h - 5.0) as f64);
            p.c.scale(0.34, 0.34);
            draw_plant_kind(p, k, 0.0, 0.0, g.time, 1.0, 1.0, 0.0);
            p.restore();
            p.circle(x + w - 7.0, y + 8.0, 7.0, rgba(20, 30, 14, 0.72));
            p.text("×", x + w - 7.0, y + 12.0, 11.0, "#ffd7c2", "center", true);
        } else {
            p.text(&format!("{}", i + 1), x + w * 0.5, y + h * 0.5 + 6.0, 16.0, rgba(215, 235, 190, 0.35), "center", false);
        }
    }

    // 可选植物
    for (i, k) in g.cfg.plants.iter().enumerate() {
        let (x, y, w, h) = choose_rect(i);
        let selected = g.seeds.contains(k);
        let hover = rect_contains((x, y, w, h), g.mouse.0, g.mouse.1);
        p.rr(x + 2.0, y + 3.0, w, h, 11.0, rgba(0, 0, 0, 0.3));
        p.rr(x, y, w, h, 11.0, if selected { "#fff0b0" } else if hover { "#f4e9cb" } else { "#e9dcb6" });
        p.rr_s(x, y, w, h, 11.0, if selected { 3.0 } else { 1.8 }, if selected { "#ffd34a" } else { "#977443" });
        p.save();
        p.c.translate((x + w * 0.5) as f64, (y + h - 9.0) as f64);
        p.c.scale(0.34, 0.34);
        draw_plant_kind(p, *k, 0.0, 0.0, g.time + i as f32 * 0.3, 1.0, 1.0, 0.0);
        p.restore();
        p.text(k.name(), x + w * 0.5, y + h - 2.0, 9.5, "#4b3211", "center", false);
        p.rr(x + w - 30.0, y + 3.0, 27.0, 14.0, 7.0, rgba(80, 54, 18, 0.16));
        p.text(&format!("{}", k.cost()), x + w - 16.5, y + 12.0, 9.5, "#4b3211", "center", true);
        if selected {
            let n = g.seeds.iter().position(|x| *x == *k).unwrap_or(0) + 1;
            p.circle(x + 13.0, y + 13.0, 10.0, "#ffd34a");
            p.text(&format!("{}", n), x + 13.0, y + 17.0, 12.0, "#3c2604", "center", true);
        }
    }

    // 开始
    let can = !g.seeds.is_empty();
    let sh = rect_contains(CHOOSE_START, g.mouse.0, g.mouse.1);
    p.rr(CHOOSE_START.0, CHOOSE_START.1 + 3.0, CHOOSE_START.2, CHOOSE_START.3, 14.0, rgba(0, 0, 0, 0.3));
    p.rr(CHOOSE_START.0, CHOOSE_START.1, CHOOSE_START.2, CHOOSE_START.3, 14.0, if can { if sh { "#7ec24a" } else { "#63a838" } } else { "#4a5a3a" });
    p.rr_s(CHOOSE_START.0, CHOOSE_START.1, CHOOSE_START.2, CHOOSE_START.3, 14.0, 3.0, if can { "#cdf39b" } else { "#6f7a63" });
    p.text_out(
        if can { "开 始 战 斗" } else { "请选择植物" },
        W * 0.5,
        CHOOSE_START.1 + 39.0,
        28.0,
        if can { "#ffffff" } else { "#aab39a" },
        "#2c4c18",
        "center",
        true,
    );

    if let Some((msg, _, bad)) = &g.toast {
        p.rr(W * 0.5 - 150.0, 690.0, 300.0, 34.0, 10.0, rgba(20, 32, 12, 0.92));
        p.text(msg, W * 0.5, 712.0, 14.0, if *bad { "#ffb59c" } else { "#dff0c5" }, "center", false);
    }
}

fn draw_menu(p: &P, g: &Game) {
    draw_menu_backdrop(p, g);

    p.text_out("植物大战僵尸", W * 0.5, 176.0, 82.0, "#ffe14d", "#3c2404", "center", true);
    p.text_out("R U S T   重 制 版", W * 0.5, 232.0, 21.0, "#c6f08a", "#23400f", "center", true);
    p.text("守住草坪，别让僵尸走进房子", W * 0.5, 278.0, 17.0, rgba(240, 250, 222, 0.9), "center", false);

    let r = BTN_ADVENTURE;
    let hover = rect_contains(r, g.mouse.0, g.mouse.1);
    let y = r.1 + if hover { -3.0 } else { 0.0 };
    p.rr(r.0, y + 5.0, r.2, r.3, 16.0, rgba(0, 0, 0, 0.36));
    p.rr(r.0, y, r.2, r.3, 16.0, if hover { "#b87535" } else { "#96582a" });
    p.rr_s(r.0, y, r.2, r.3, 16.0, 3.0, if hover { "#ffe6a3" } else { "#e8c477" });
    p.text_out("冒 险 模 式", r.0 + r.2 * 0.5, y + 36.0, 34.0, "#fff6d2", "#5a2f0d", "center", true);
    p.text("ADVENTURE  ·  1-1 至 5-10", r.0 + r.2 * 0.5, y + 62.0, 12.0, rgba(255, 240, 200, 0.86), "center", false);
    let sr = BTN_SURVIVAL;
    let sh = rect_contains(sr, g.mouse.0, g.mouse.1);
    p.rr(sr.0, sr.1 + 3.0, sr.2, sr.3, 12.0, rgba(0, 0, 0, 0.32));
    p.rr(sr.0, sr.1, sr.2, sr.3, 12.0, if sh { "#6c8f3c" } else { "#5a7a32" });
    p.rr_s(sr.0, sr.1, sr.2, sr.3, 12.0, 2.0, if sh { "#e9f5bd" } else { "#c6dd8e" });
    p.text("生 存 模 式 · 无尽", sr.0 + sr.2 * 0.5, sr.1 + 31.0, 20.0, "#f8ffe8", "center", true);
    p.text("点击冒险模式 → 五章五十关地图 → 选关卡 → 选种子开战", W * 0.5, 588.0, 13.0, rgba(236, 248, 218, 0.9), "center", false);
    p.text("Rust + WebAssembly ｜ 正版植物立绘与原声素材 ｜ Canvas 2D", W * 0.5, 706.0, 13.0, rgba(230, 245, 210, 0.75), "center", false);
}

fn draw_menu_backdrop(p: &P, g: &Game) {
    p.rect(0.0, 0.0, W, H, "#4f8c2c");
    for col in 0..COLS {
        let c = if col % 2 == 0 { "#85c951" } else { "#77bc44" };
        p.rect(GRID_X + col as f32 * CELL_W, 0.0, CELL_W, H, c);
    }
    p.rect(GRID_X + COLS as f32 * CELL_W, 0.0, W, H, "#47822a");
    for i in 0..4 {
        let x = GRID_X + COLS as f32 * CELL_W + i as f32 * 30.0;
        p.rect(x, 0.0, 14.0, H, rgba(20, 55, 12, 0.10));
    }
    draw_house(p, g);
    p.rect(0.0, 0.0, W, H, rgba(10, 26, 8, 0.24));
    for (kind, cx, t) in [
        (PlantKind::Sunflower, 130.0f32, 1.0f32),
        (PlantKind::Peashooter, 270.0, 2.0),
        (PlantKind::WallNut, 400.0, 0.0),
        (PlantKind::Cherry, 880.0, 3.0),
        (PlantKind::SnowPea, 1050.0, 1.5),
    ] {
        p.save();
        p.c.translate(cx as f64, 672.0);
        p.c.scale(0.86, 0.86);
        draw_plant_anim(p, kind, 0.0, 0.0, g.time + t, 1.0, 1.0, 0.0);
        p.restore();
    }
    for (x, y, t) in [(1005.0f32, 190.0f32, 0.0f32), (820.0f32, 640.0f32, 2.0f32)] {
        let s = Sun { x, y, ty: y, value: 25, t, life: 9.0, state: SunState::Idle };
        draw_sun(p, &s, g.time);
    }
}

fn draw_map(p: &P, g: &Game) {
    let page = g.map_page.min(WORLD_COUNT - 1);
    p.rect(0.0, 0.0, W, H, "#3f7223");
    for col in 0..COLS {
        let c = if col % 2 == 0 { "#76b342" } else { "#69a83a" };
        p.rect(GRID_X + col as f32 * CELL_W, 0.0, CELL_W, H, c);
    }
    p.rect(GRID_X + COLS as f32 * CELL_W, 0.0, W, H, "#396b20");
    draw_house(p, g);
    let (tr, tg, tb, ta) = level_cfg(page * LEVELS_PER_WORLD).tint;
    if ta > 0.0 {
        p.rect(0.0, 0.0, W, H, rgba(tr, tg, tb, ta));
    }
    p.rect(0.0, 0.0, W, H, rgba(8, 24, 6, 0.22));

    p.rr(360.0, 40.0, 560.0, 86.0, 14.0, rgba(47, 26, 10, 0.84));
    p.rr_s(360.0, 40.0, 560.0, 86.0, 14.0, 3.0, "#d8a554");
    p.text_out("冒 险 模 式", W * 0.5, 76.0, 34.0, "#ffe88a", "#4b2406", "center", true);
    p.text(WORLD_SUBS[page], W * 0.5, 108.0, 15.0, "#f4dcae", "center", false);

    let hr = rect_contains(MAP_BACK, g.mouse.0, g.mouse.1);
    p.rr(MAP_BACK.0, MAP_BACK.1, MAP_BACK.2, MAP_BACK.3, 10.0, if hr { rgba(110, 74, 40, 1.0) } else { rgba(45, 28, 12, 0.82) });
    p.rr_s(MAP_BACK.0, MAP_BACK.1, MAP_BACK.2, MAP_BACK.3, 10.0, 2.0, "#dcb878");
    p.text("‹ 返回", MAP_BACK.0 + 52.0, MAP_BACK.1 + 26.0, 16.0, "#fff0c9", "center", false);

    // 五个章节标签
    for w in 0..WORLD_COUNT {
        let r = world_tab_rect(w);
        let act = w == page;
        let hover = rect_contains(r, g.mouse.0, g.mouse.1);
        p.rr(r.0, r.1, r.2, r.3, 9.0, if act { rgba(200, 135, 63, 1.0) } else if hover { rgba(109, 73, 38, 1.0) } else { rgba(38, 26, 14, 0.78) });
        p.rr_s(r.0, r.1, r.2, r.3, 9.0, 2.0, if act { "#ffdf94" } else { "#b28b56" });
        p.text(&format!("{}-{}  {}", w + 1, 1, ["白天", "夜晚", "泳池", "浓雾", "屋顶"][w]), r.0 + r.2 * 0.5, r.1 + 24.0, 14.0, if act { "#3f230b" } else { "#e7cfa3" }, "center", act);
    }

    // 两个折返行组成的关卡路径
    let mut prev: Option<(f32, f32)> = None;
    for j in 0..LEVELS_PER_WORLD {
        let r = level_rect(j);
        let cx = r.0 + r.2 * 0.5;
        let cy = r.1 + r.3 * 0.5;
        if let Some((px, py)) = prev {
            p.line(&[(px, py), (cx, cy)], 7.0, rgba(94, 68, 36, 0.85));
            p.line(&[(px, py), (cx, cy)], 3.0, rgba(205, 174, 120, 0.9));
        }
        prev = Some((cx, cy));
    }

    for j in 0..LEVELS_PER_WORLD {
        let r = level_rect(j);
        let cx = r.0 + r.2 * 0.5;
        let cy = r.1 + r.3 * 0.5;
        let cfg = level_cfg(page * LEVELS_PER_WORLD + j);
        let hover = rect_contains(r, g.mouse.0, g.mouse.1);
        let flag = ["#d84b37", "#df8a2e", "#4c9bd8", "#8a77c9", "#c94f9a"][page];
        p.ell(cx, cy + 30.0, 48.0, 10.0, rgba(20, 34, 10, 0.28));
        p.circle(cx, cy + 18.0, 26.0, "#8c6a3c");
        p.circle(cx, cy + 15.0, 23.0, "#d5b577");
        p.rr(r.0, r.1 + if hover { -3.0 } else { 0.0 }, r.2, r.3, 8.0, if hover { "#9b622f" } else { "#7f4e25" });
        p.rr_s(r.0 + 3.0, r.1 + 3.0 + if hover { -3.0 } else { 0.0 }, r.2 - 6.0, r.3 - 6.0, 6.0, 2.0, if hover { "#ffe6ad" } else { "#efd49a" });
        p.text_out(&format!("{}-{}", page + 1, j + 1), cx, cy - 2.0 + if hover { -3.0 } else { 0.0 }, 20.0, "#5b3412", "#f7e5bd", "center", true);
        p.text(&format!("{} 波", cfg.waves), cx, cy + 19.0 + if hover { -3.0 } else { 0.0 }, 9.0, "#7c5228", "center", false);
        // 节点小旗
        p.line(&[(r.0 + r.2 - 9.0, r.1 - 22.0), (r.0 + r.2 - 9.0, r.1 + 8.0)], 2.0, "#68451f");
        p.poly(&[(r.0 + r.2 - 8.0, r.1 - 21.0), (r.0 + r.2 + 14.0, r.1 - 15.0), (r.0 + r.2 - 8.0, r.1 - 9.0)], flag);
    }

    p.text(&format!("第 {} 章 · {}：点击关卡进入种子选择", page + 1, WORLD_NAMES[page]), W * 0.5, 660.0, 15.0, rgba(240, 249, 224, 0.92), "center", false);
    p.text("顶部标签切换章节 · 右键 / Esc 返回主菜单 · 共 50 关", W * 0.5, 686.0, 12.0, rgba(206, 229, 181, 0.8), "center", false);
}

fn draw_pause(p: &P, _g: &Game) {
    p.rect(0.0, 0.0, W, H, rgba(6, 14, 6, 0.58));
    p.rr(400.0, 210.0, 480.0, 430.0, 22.0, rgba(16, 34, 12, 0.9));
    p.rr_s(400.0, 210.0, 480.0, 430.0, 22.0, 2.0, rgba(233, 196, 106, 0.5));
    p.text_out("游戏暂停", W * 0.5, 300.0, 58.0, "#ffe14d", "#2a1a04", "center", true);
    p.text("按空格继续，或点击下方按钮", W * 0.5, 362.0, 18.0, "#dbeec4", "center", false);
    p.text("小提示：阳光点一下就收，别等它自己消失", W * 0.5, 430.0, 15.0, "#a9c98a", "center", false);
    p.rr(START_BTN.0, START_BTN.1, START_BTN.2, START_BTN.3, 14.0, "#63a838");
    p.rr_s(START_BTN.0, START_BTN.1, START_BTN.2, START_BTN.3, 14.0, 3.0, "#cdf39b");
    p.text_out("继 续", W * 0.5, START_BTN.1 + 39.0, 28.0, "#ffffff", "#2c4c18", "center", true);
}

fn draw_end(p: &P, g: &Game, win: bool) {
    p.rect(0.0, 0.0, W, H, rgba(6, 14, 6, 0.68));
    let title = if win {
        "草坪守住了！"
    } else {
        "僵尸吃掉了你的脑子…"
    };
    let sub = if win {
        if g.level + 1 < LEVEL_COUNT {
            "本关守住，下一关的僵尸更强"
        } else {
            "三个关卡全部守住，前院安全了"
        }
    } else {
        "下次多种点向日葵，早点布防线"
    };
    p.rr(320.0, 210.0, 640.0, 400.0, 22.0, rgba(16, 34, 12, 0.9));
    p.rr_s(320.0, 210.0, 640.0, 400.0, 22.0, 2.0, rgba(233, 196, 106, 0.5));
    p.text_out(
        title,
        W * 0.5,
        290.0,
        if win { 54.0 } else { 46.0 },
        if win { "#ffe14d" } else { "#ff9b7a" },
        "#2a1204",
        "center",
        true,
    );
    p.text(sub, W * 0.5, 348.0, 18.0, "#dfeec6", "center", false);
    p.text(
        &format!("击杀僵尸 {}  ·  种下植物 {}  ·  用时 {}", g.kills, g.planted, time_str(g.elapsed)),
        W * 0.5,
        398.0,
        16.0,
        "#b7d596",
        "center",
        false,
    );
    p.text(
        if win {
            "Rust 逻辑 + Canvas 绘制，全部在浏览器里跑"
        } else {
            "提示：坚果墙挡在前面，樱桃炸弹清一片"
        },
        W * 0.5,
        436.0,
        14.0,
        rgba(200, 220, 175, 0.8),
        "center",
        false,
    );
    p.rr(AGAIN_BTN.0, AGAIN_BTN.1, AGAIN_BTN.2, AGAIN_BTN.3, 14.0, "#63a838");
    p.rr_s(AGAIN_BTN.0, AGAIN_BTN.1, AGAIN_BTN.2, AGAIN_BTN.3, 14.0, 3.0, "#cdf39b");
    p.text_out(
        if win && g.level + 1 < LEVEL_COUNT { "下 一 关" } else { "再 来 一 局" },
        W * 0.5,
        AGAIN_BTN.1 + 38.0,
        28.0,
        "#ffffff",
        "#2c4c18",
        "center",
        true,
    );
}
