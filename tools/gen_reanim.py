#!/usr/bin/env python3
"""把 assets/reanim/manifest.json 编译成 Rust 元数据与前端预载表。

产物：
  src/reanim.rs        每个角色的 clip 元数据（格子数、锚点、fps）+ 全局图片 key 表
  web/reanim_keys.js   window.__REANIM_SRC，供 index.html 合入 IMG_SRC
"""
import json
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MAN = os.path.join(ROOT, 'assets', 'reanim', 'manifest.json')

ROLES = ('walk', 'idle', 'special', 'shoot', 'eat', 'die')


def load():
    with open(MAN, encoding='utf-8') as fh:
        return json.load(fh)


def main():
    man = load()
    chars = man['chars']
    order = sorted(chars)
    keys = []
    for name in order:
        for clip in chars[name]['clips'].values():
            keys.append(clip['file'][len('reanim/'):-len('.webp')])
    keys = sorted(set(keys))
    slot = {k: i for i, k in enumerate(keys)}

    def lit(clip, curr, name):
        if not clip:
            return 'Clip::NONE'
        k = clip['file'][len('reanim/'):-len('.webp')]
        return ('Clip { ok: true, slot: %d, frames: %d, cols: %d, rows: %d, w: %.1f, h: %.1f, fps: %.2f }'
                % (slot[k], clip['frames'], clip['cols'], clip['rows'], curr['w'], curr['h'], clip['fps']))

    rows = []
    for name in order:
        c = chars[name]
        cl = c['clips']
        base = cl.get('walk') or cl.get('idle') or cl.get('special')
        rows.append('    Char { name: "%s", w: %.1f, h: %.1f, ax: %.1f, ay: %.1f, base: %s, shoot: %s, eat: %s, die: %s, special: %s },'
                    % (name, c['w'], c['h'], c['ax'], c['ay'],
                       lit(base, c, name), lit(cl.get('shoot'), c, name), lit(cl.get('eat'), c, name),
                       lit(cl.get('die'), c, name), lit(cl.get('special'), c, name)))
    key_rows = '\n'.join('    "%s",' % k for k in keys)

    src = '''// 由 tools/gen_reanim.py 从 assets/reanim/manifest.json 生成，请勿手改。
// 原版 reanim（12fps）逐帧烘焙的格子图集元数据：每个 clip 是 cols x rows 的网格，
// 单元格 w x h 像素；角色锚点 (ax, ay) 是 reanim 原点（脚底）在单元格内的位置。

#[derive(Clone, Copy)]
pub struct Clip {
    pub ok: bool,
    pub slot: u16,
    pub frames: u16,
    pub cols: u16,
    pub rows: u16,
    pub w: f32,
    pub h: f32,
    pub fps: f32,
}

impl Clip {
    pub const NONE: Clip = Clip { ok: false, slot: 0, frames: 0, cols: 1, rows: 1, w: 0.0, h: 0.0, fps: 12.0 };
}

pub struct Char {
    pub name: &'static str,
    pub w: f32,
    pub h: f32,
    pub ax: f32,
    pub ay: f32,
    pub base: Clip,
    pub shoot: Clip,
    pub eat: Clip,
    pub die: Clip,
    pub special: Clip,
}

impl Char {
    pub fn has(&self, clip: &Clip) -> bool {
        clip.ok && clip.frames > 0 && clip.w > 0.0 && self.h > 0.0
    }
}

pub static KEYS: &[&str] = &[
%s
];

pub static CHARS: &[Char] = &[
%s
];

pub fn by_name(name: &str) -> Option<&'static Char> {
    CHARS.iter().find(|c| c.name == name)
}
''' % (key_rows, '\n'.join(rows))

    with open(os.path.join(ROOT, 'src', 'reanim.rs'), 'w', encoding='utf-8') as fh:
        fh.write(src)

    js = ['window.__REANIM_SRC = {']
    for k in keys:
        js.append('  "r_%s": "assets/reanim/%s.webp",' % (k.replace('/', '_'), k))
    js.append('};')
    with open(os.path.join(ROOT, 'web', 'reanim_keys.js'), 'w', encoding='utf-8') as fh:
        fh.write('\n'.join(js) + '\n')

    total = sum(clip['bytes'] for c in chars.values() for clip in c['clips'].values())
    nclips = sum(len(c['clips']) for c in chars.values())
    print('reanim.rs: %d chars, %d clips, %d keys, %.2f MB'
          % (len(order), nclips, len(keys), total / 1048576.0))


if __name__ == '__main__':
    main()
