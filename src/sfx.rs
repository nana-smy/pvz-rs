// 音效：优先播放原版素材（index.html 暴露 __pvzSfx），缺失时回退 WebAudio 合成
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{AudioContext, OscillatorType};

fn js_call(name: &str, arg: JsValue) -> bool {
    let Some(w) = web_sys::window() else { return false };
    let Ok(f) = js_sys::Reflect::get(&w, &JsValue::from_str(name)) else {
        return false;
    };
    let Ok(f) = f.dyn_into::<js_sys::Function>() else {
        return false;
    };
    f.call1(&JsValue::NULL, &arg)
        .map(|v| v.as_bool().unwrap_or(true))
        .unwrap_or(false)
}

pub struct Sfx {
    ctx: Option<AudioContext>,
    pub enabled: bool,
}

impl Sfx {
    pub fn new() -> Self {
        let ctx = AudioContext::new().ok();
        Self { ctx, enabled: true }
    }

    pub fn resume(&self) {
        if let Some(c) = &self.ctx {
            let _ = c.resume();
        }
    }

    fn sample(&self, name: &str) -> bool {
        if !self.enabled {
            return true;
        }
        js_call("__pvzSfx", JsValue::from_str(name))
    }

    pub fn music(&self, on: bool) {
        js_call("__pvzMusic", JsValue::from_bool(on && self.enabled));
    }

    fn tone(&self, f0: f32, f1: f32, dur: f64, kind: OscillatorType, vol: f32) {
        if !self.enabled {
            return;
        }
        let Some(c) = &self.ctx else { return };
        let Ok(osc) = c.create_oscillator() else { return };
        let Ok(g) = c.create_gain() else { return };
        let t = c.current_time();
        osc.set_type(kind);
        let _ = osc.frequency().set_value_at_time(f0, t);
        let _ = osc
            .frequency()
            .exponential_ramp_to_value_at_time(f1.max(20.0), t + dur);
        let _ = g.gain().set_value_at_time(vol, t);
        let _ = g
            .gain()
            .exponential_ramp_to_value_at_time(0.0001, t + dur);
        let _ = osc.connect_with_audio_node(&g);
        let _ = g.connect_with_audio_node(&c.destination());
        let _ = osc.start();
        let _ = osc.stop_with_when(t + dur);
    }

    pub fn click(&self) {
        if self.sample("click") {
            return;
        }
        self.tone(520.0, 680.0, 0.06, OscillatorType::Triangle, 0.035);
    }

    pub fn pause(&self) {
        if self.sample("pause") {
            return;
        }
        self.tone(420.0, 260.0, 0.12, OscillatorType::Triangle, 0.04);
    }

    pub fn shoot(&self) {
        self.tone(680.0, 320.0, 0.08, OscillatorType::Square, 0.035);
    }

    pub fn ice_shoot(&self) {
        self.tone(980.0, 520.0, 0.09, OscillatorType::Sine, 0.04);
    }

    pub fn sun(&self) {
        if self.sample("sun") {
            return;
        }
        self.tone(760.0, 1320.0, 0.14, OscillatorType::Sine, 0.05);
    }

    pub fn plant(&self) {
        if self.sample("plant") {
            return;
        }
        self.tone(220.0, 130.0, 0.09, OscillatorType::Triangle, 0.05);
    }

    pub fn explode(&self) {
        self.tone(160.0, 38.0, 0.5, OscillatorType::Sawtooth, 0.11);
    }

    pub fn groan(&self) {
        self.tone(110.0, 78.0, 0.32, OscillatorType::Sawtooth, 0.03);
    }

    pub fn mower(&self) {
        self.tone(160.0, 420.0, 0.35, OscillatorType::Square, 0.06);
    }

    pub fn horn(&self) {
        if self.sample("wave") {
            return;
        }
        self.tone(180.0, 130.0, 0.5, OscillatorType::Triangle, 0.06);
    }

    pub fn lose(&self) {
        if self.sample("lose") {
            return;
        }
        self.tone(240.0, 70.0, 0.9, OscillatorType::Triangle, 0.07);
    }

    pub fn win(&self) {
        if self.sample("win") {
            return;
        }
        self.tone(520.0, 980.0, 0.45, OscillatorType::Sine, 0.06);
    }

    pub fn bite(&self) {
        self.tone(150.0, 90.0, 0.05, OscillatorType::Square, 0.02);
    }
}
