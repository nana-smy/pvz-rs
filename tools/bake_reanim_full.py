#!/usr/bin/env python3
"""把原版 .reanim 逐帧烘焙成网格 WebP 图集 + manifest.json。

只做离线烘焙；运行时仍按 12fps 播图集。坐标语义沿用已验证的旧算法：
x/y 是部件左上角的世界坐标，y 向下，不翻转；sx/sy 直接缩放；
旋转 -(kx+ky)/2 度，绕缩放后包围盒中心旋转。
"""
import json
import math
import os
import re
import sys
from io import BytesIO
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.environ.get('REANIM_SRC', '/tmp/userassets/ext2/6b02f084/reanim')
OUT = os.path.join(ROOT, 'assets', 'reanim')
FPS = 12

# 输出角色 -> 原版 reanim 文件名（不含 .reanim）
PLANTS = {
    'lilypad': 'LilyPad', 'flowerpot': 'Pot', 'chomper': 'Chomper',
    'threepeater': 'ThreePeater', 'spikeweed': 'Caltrop',
    'puffshroom': 'PuffShroom', 'sunshroom': 'SunShroom', 'fumeshroom': 'FumeShroom',
    'gravebuster': 'Gravebuster', 'hypnoshroom': 'HypnoShroom', 'scaredyshroom': 'ScaredyShroom',
    'iceshroom': 'IceShroom', 'doomshroom': 'DoomShroom', 'tanglekelp': 'Tanglekelp',
    'torchwood': 'Torchwood', 'tallnut': 'Tallnut', 'seashroom': 'SeaShroom',
    'plantern': 'Plantern', 'cactus': 'Cactus', 'blover': 'Blover',
    'splitpea': 'SplitPea', 'pumpkin': 'Pumpkin', 'magnetshroom': 'Magnetshroom',
    'kernelpult': 'Cornpult', 'coffeebean': 'Coffeebean', 'umbrelleaf': 'Umbrellaleaf',
    'marigold': 'Marigold', 'melonpult': 'Melonpult', 'gatlingpea': 'GatlingPea',
    'twinsunflower': 'TwinSunflower', 'gloomshroom': 'GloomShroom', 'cattail': 'Cattail',
    'wintermelon': 'WinterMelon', 'goldmagnet': 'GoldMagnet', 'spikerock': 'SpikeRock',
    'cobcannon': 'CobCannon', 'imitater': 'Imitater',
}

ZOMBIES = {
    'basic': 'Zombie', 'cone': 'Zombie', 'bucket': 'Zombie', 'screendoor': 'Zombie',
    'duckytube': 'Zombie', 'flag': 'Zombie', 'polevault': 'Zombie_polevaulter',
    'football': 'Zombie_football', 'ladder': 'Zombie_ladder', 'newspaper': 'Zombie_paper',
    'dancing': 'Zombie_dancer', 'backup': 'Zombie_backup', 'snorkel': 'Zombie_snorkle',
    'zamboni': 'Zombie_zamboni', 'bobsled': 'Zombie_bobsled', 'dolphinrider': 'Zombie_dolphinrider',
    'jackbox': 'Zombie_jackbox', 'balloon': 'Zombie_balloon', 'digger': 'Zombie_digger',
    'pogo': 'Zombie_pogo', 'yeti': 'Zombie_yeti', 'bungee': 'Zombie_bungi',
    'catapult': 'Zombie_catapult', 'gargantuar': 'Zombie_gargantuar', 'imp': 'Zombie_imp',
    'boss': 'Zombie_boss',
}

# 旧的可信植物条带；这些不重烘，直接转 WebP，避免再次踩几何坑。
GOLDEN = {
    'sunflower': 'sunflower', 'peashooter': 'peashooter', 'snowpea': 'snowpea',
    'wallnut': 'wallnut', 'cherry': 'cherrybomb', 'potatomine': 'potatomine',
    'jalapeno': 'jalapeno', 'squash': 'squash',
    'cabbage': 'cabbagepult', 'starfruit': 'starfruit', 'garlic': 'garlic',

}

PLANT_CLIPS = {
    'shoot': ['anim_shooting'], 'idle': ['anim_idle', 'anim_full_idle'],
    'special': ['anim_special'],
}
ZOMBIE_CLIPS = {
    'walk': ['anim_walk', 'anim_drive', 'anim_walkdolphin', 'anim_armraise', 'anim_ride', 'anim_push', 'anim_idle'],
    'eat': ['anim_eat', 'anim_eat_nopaper'],
    'die': ['anim_death', 'anim_death2'],
    'special': ['anim_smash', 'anim_shoot', 'anim_stomp_1', 'anim_pop', 'anim_swing',
                'anim_pogo', 'anim_dig', 'anim_grab', 'anim_jump', 'anim_thrown',
                'anim_armraise', 'anim_ladderwalk', 'anim_placeladder', 'anim_swim',
                'anim_wheelie1', 'anim_ride'],
}

HIDE_BASE = ['Zombie_mustache', 'anim_tongue', 'Zombie_tongue', 'Zombie_flaghand',
             'Zombie_whitewater', 'Zombie_snorkle_whitewater']
HIDE_SCREENDOOR = ['Zombie_innerarm_screendoor', 'Zombie_innerarm_screendoor_hand',
                   'Zombie_screendoor1', 'anim_screendoor', 'Zombie_outerarm_screendoor']
HIDE_CONE = ['anim_cone']
HIDE_BUCKET = ['anim_bucket']
HIDE_DUCK = ['anim_duckytube', 'Zombie_duckytube']

def norm(s):
    return re.sub(r'[^a-z0-9]', '', s.lower())

def load_images():
    idx = {}
    for fn in os.listdir(SRC):
        if fn.lower().endswith('.png'):
            idx[norm(fn[:-4])] = os.path.join(SRC, fn)
    return idx

IMG_IDX = load_images()
_cache = {}

def get_img(image_id):
    if image_id in _cache:
        return _cache[image_id]
    key = norm(image_id.replace('IMAGE_REANIM_', ''))
    path = IMG_IDX.get(key)
    if not path:
        # 允许前缀差异：取末段匹配
        for k, p in IMG_IDX.items():
            if k.endswith(key) or key.endswith(k):
                path = p
                break
    if not path:
        return None
    im = Image.open(path).convert('RGBA')
    _cache[image_id] = im
    return im

TRACK_RE = re.compile(r'<track>(.*?)</track>', re.S)

def parse(path, carry=False):
    text = open(path, encoding='utf-8', errors='ignore').read()
    tracks = []
    for raw in TRACK_RE.findall(text):
        m = re.search(r'<name>([^<]+)</name>', raw)
        if not m:
            continue
        name = m.group(1)
        ts = re.findall(r'<t>(.*?)</t>', raw, re.S)
        states = []
        st = dict(i=None, x=0.0, y=0.0, a=0.0, sx=1.0, sy=1.0, kx=0.0, ky=0.0, f=None)
        # 原版稀疏关键帧语义：没写的字段沿用上一帧，空 <t> 表示不变。
        for t in ts:
            for key in ('x', 'y', 'a', 'sx', 'sy', 'kx', 'ky', 'f'):
                mm = re.search(r'<%s>([^<]+)</%s>' % (key, key), t)
                if mm:
                    try:
                        st[key] = float(mm.group(1))
                    except ValueError:
                        pass
            mm = re.search(r'<i>([^<]+)</i>', t)
            if mm:
                st['i'] = mm.group(1).strip()
            states.append(dict(st))
        tracks.append((name, states))
    return tracks

def first_image_frame(states):
    for i, st in enumerate(states):
        if st.get('i'):
            return i
    return 0

def anim_range(states, name):
    """返回 (start,end)，闭区间。f=0 为激活，f=-1 为关闭。"""
    has_f = [st for st in states if st.get('f') is not None]
    zero = [i for i, st in enumerate(states) if st.get('f') == 0]
    if not zero:
        # 只有 -1 表示该动画轨在该角色上从未启用；无 f 字段才是全程有效
        if has_f:
            return None
        return 0, len(states) - 1
    start = zero[0]
    end = len(states) - 1
    for i in range(start + 1, len(states)):
        if states[i].get('f') == -1:
            end = i - 1
            break
    if end - start < 2:
        end = min(len(states) - 1, start + 12)
    return start, end

def lerp(a, b, t):
    return a + (b - a) * t

def sample_state(states, frame):
    # 逐帧烘焙直接取该帧的 carried state；reanim 的每个 <t> 本身就是关键帧，
    # 原版在关键帧之间线性插值，但 12fps 采样下直接取帧更稳，不会提前跳一帧。
    return states[min(max(frame, 0), len(states) - 1)]

def _tan_clamp(deg):
    if abs(deg) < 1e-6:
        return 0.0
    return max(-3.0, min(3.0, math.tan(math.radians(deg))))

class Xform(object):
    """reanim 空间：y 向下，部件原点=左上角，kx/ky 是剪切角而非旋转角。"""
    def __init__(self, fr, scale):
        self.sx = fr['sx'] * scale
        self.sy = fr['sy'] * scale
        # 原版 Reanimator::MatrixFromTransform：kx/ky 分别是 X/Y 方向的倾斜角，
        # 矩阵为 [cos(-kx), sin(-ky); -sin(-kx), cos(-ky)]，不是 tan 剪切。
        thx = math.radians(fr['kx'])
        thy = math.radians(fr['ky'])
        self.m00 = math.cos(thx) * self.sx
        self.m10 = math.sin(thx) * self.sx
        self.m01 = -math.sin(thy) * self.sy
        self.m11 = math.cos(thy) * self.sy
        self.x = fr['x'] * scale
        self.y = fr['y'] * scale

    def map_bbox(self, w, h, ox, oy):
        pts = []
        for u, v in ((0.0, 0.0), (w, 0.0), (0.0, h), (w, h)):
            xc = ox + self.x + self.m00 * u + self.m01 * v
            yc = oy + self.y + self.m10 * u + self.m11 * v
            pts.append((xc, yc))
        return (min(p[0] for p in pts), min(p[1] for p in pts),
                max(p[0] for p in pts), max(p[1] for p in pts))

    def affine(self, w, h, ox, oy, offx, offy):
        # 原版把图片左上角 (0,0) 作为旋转/倾斜原点，而不是包围盒中心
        cx = ox + self.x
        cy = oy + self.y
        det = self.m00 * self.m11 - self.m01 * self.m10
        if abs(det) < 1e-5:
            det = 1e-5
        i00 = self.m11 / det
        i01 = self.m01 / det
        i10 = self.m10 / det
        i11 = self.m00 / det
        return (i00, i01,
                i00 * (offx - cx) + i01 * (offy - cy),
                i10, i11,
                i10 * (offx - cx) + i11 * (offy - cy))

def paste_part(canvas, part, fr, scale, ox, oy):
    w, h = part.size
    xf = Xform(fr, scale)
    bb = xf.map_bbox(w, h, ox, oy)
    dw = int(math.ceil(bb[2] - bb[0])) + 1
    dh = int(math.ceil(bb[3] - bb[1])) + 1
    if dw <= 0 or dh <= 0 or dw > 3000 or dh > 3000:
        return False
    out = part.transform((dw, dh), Image.AFFINE, xf.affine(w, h, ox, oy, bb[0], bb[1]),
                         resample=Image.BICUBIC)
    if out.getchannel('A').getbbox() is None:
        return False
    canvas.alpha_composite(out, (int(round(bb[0])), int(round(bb[1]))))
    return True

def draw_frame(tracks, frame, hide=(), origin=(1000, 1000)):
    canvas = Image.new('RGBA', (2400, 2400), (0, 0, 0, 0))
    ox, oy = origin
    for name, states in reversed(tracks):
        if any(h in name for h in hide):
            continue
        st = sample_state(states, frame)
        iid = st.get('i')
        if not iid or (st.get('f') is not None and st['f'] < 0):
            continue
        im = get_img(iid)
        if im is None:
            continue
        if abs(st['sx']) < 1e-4 or abs(st['sy']) < 1e-4:
            continue
        paste_part(canvas, im, st, 1.0, ox, oy)
    return canvas

def bbox_union(canvases):
    box = None
    for c in canvases:
        b = c.getbbox()
        if not b:
            continue
        box = b if box is None else (min(box[0], b[0]), min(box[1], b[1]),
                                     max(box[2], b[2]), max(box[3], b[3]))
    if box is None:
        return (480, 480, 520, 520)
    pad = max(4, int(max(box[2] - box[0], box[3] - box[1]) * 0.06))
    return (max(0, box[0] - pad), max(0, box[1] - pad),
            min(2400, box[2] + pad), min(2400, box[3] + pad))

def grid_image(frames, w, h, cols):
    rows = math.ceil(len(frames) / cols)
    sheet = Image.new('RGBA', (w * cols, h * rows), (0, 0, 0, 0))
    for i, f in enumerate(frames):
        sheet.alpha_composite(f, ((i % cols) * w, (i // cols) * h))
    return sheet, cols, rows

def save_webp(im, path, quality=72):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    im.save(path, 'WEBP', quality=quality, method=4)

def choose_range(tracks, names, total):
    for n in names:
        for tn, st in tracks:
            if tn == n:
                r = anim_range(st, n)
                if r:
                    return r
    return 0, total - 1

def _is_head_track(name, n):
    return (name.startswith('anim_head%d' % n) or name.startswith('ThreePeater_head%d' % n)
            or name.startswith('anim_face%d' % n) or name.startswith('ThreePeater_mouth%d' % n)
            or name.startswith('ThreePeater_head%d' % n))

def _draw_group(canvas, tracks, frame, include):
    ox, oy = 1000, 1000
    for name, states in reversed(tracks):
        if not include(name):
            continue
        st = sample_state(states, frame)
        if not st.get('i') or (st.get('f') is not None and st['f'] < 0):
            continue
        im = get_img(st['i'])
        if im is None or abs(st['sx']) < 1e-4 or abs(st['sy']) < 1e-4:
            continue
        paste_part(canvas, im, st, 1.0, ox, oy)

def draw_threepeater_frame(tracks, i):
    # 原版三发是同一副身体 + 三组独立头层；三颗头分别取各自的头动画相位。
    canvas = Image.new('RGBA', (2400, 2400), (0, 0, 0, 0))
    body = lambda n: (not _is_head_track(n, 1) and not _is_head_track(n, 2)
                      and not _is_head_track(n, 3)
                      and not n.startswith('anim_shooting') and not n.startswith('anim_blink'))
    _draw_group(canvas, tracks, 124, body)
    _draw_group(canvas, tracks, 4 + (i % 38), lambda n: _is_head_track(n, 1))
    _draw_group(canvas, tracks, 86 + (i % 37), lambda n: _is_head_track(n, 2))
    _draw_group(canvas, tracks, 45 + (i % 38), lambda n: _is_head_track(n, 3))
    return canvas

def build_threepeater(src_name):
    path = os.path.join(SRC, src_name + '.reanim')
    tracks = parse(path, carry=True)
    frames = [draw_threepeater_frame(tracks, i) for i in range(30)]
    box = bbox_union(frames)
    w, h = box[2] - box[0], box[3] - box[1]
    ax, ay = 1000 - box[0], 1000 - box[1]
    max_h = 260
    k = min(1.0, max_h / float(h))
    if k < 1.0:
        w, h = max(1, int(round(w * k))), max(1, int(round(h * k)))
        ax, ay = ax * k, ay * k
    imgs = [f.crop(box) for f in frames]
    if k < 1.0:
        imgs = [im.resize((w, h), Image.BICUBIC) for im in imgs]
    clips = {}
    for cname in ('idle', 'shoot'):
        sheet, cols, rows = grid_image(imgs, w, h, min(8, len(imgs)))
        rel = 'reanim/threepeater/%s.webp' % cname
        out = os.path.join(OUT, 'threepeater', cname + '.webp')
        save_webp(sheet, out)
        clips[cname] = dict(file=rel, frames=len(imgs), cols=cols, rows=rows,
                            fps=float(FPS), bytes=os.path.getsize(out))
    print('%-14s %-22s %dx%d clips=idle,shoot (3 head layers)' % ('threepeater', src_name, w, h))
    return dict(w=float(w), h=float(h), ax=float(ax), ay=float(ay),
                fps=FPS, scale=1.0, clips=clips)

def build_reanim_role(role, src_name, clip_names, kind, hide=()):
    path = os.path.join(SRC, src_name + '.reanim')
    if not os.path.exists(path):
        print('MISSING', role, path)
        return None
    if role == 'threepeater':
        return build_threepeater(src_name)
    tracks = parse(path)
    total = max(len(st) for _, st in tracks)
    # 选出每个 clip 的区间
    spec = {}
    max_i = 0
    for _, st in tracks:
        max_i = max(max_i, first_image_frame(st))
    for cname in clip_names:
        key = 'special' if cname == 'special' else cname
        names = (ZOMBIE_CLIPS if kind == 'zombie' else PLANT_CLIPS).get(key, [])
        s, e = choose_range(tracks, names, total)
        if kind == 'plant':
            # 植物头部/叶片可能到 idle 段末尾才出现，做 pre-roll；僵尸部件都在 0 帧齐
            if s <= max_i < e:
                s = max_i
            elif max_i >= e:
                s = max_i
                e = min(total - 1, s + 24)
        if e - s < 2:
            e = min(total - 1, s + 2)
        spec[cname] = (s, e)
    # 基准段：walk/idle，用来定 cell 包围盒
    base = 'walk' if 'walk' in spec else 'idle'
    if base not in spec:
        base = next(iter(spec))
    s, e = spec[base]
    # 包围盒必须覆盖整个基准动画段（含 pre-roll 之前的部分）。
    # 只看 pre-roll 起点会把大嘴花这类“末尾才出现舌头”的角色裁成只剩茎叶。
    span = list(range(0, e + 1))
    if len(span) > 30:
        span = [round(i * e / 29) for i in range(30)]
    base_frames = [draw_frame(tracks, f, hide) for f in span]
    box = bbox_union(base_frames)
    w, h = box[2] - box[0], box[3] - box[1]
    ax, ay = 1000 - box[0], 1000 - box[1]
    # 按运行时目标高度降采样，避免 Boss 这类 900px 图集撑爆体积
    max_h = 380 if role == 'boss' else (330 if role == 'gargantuar' else 260)
    k = min(1.0, max_h / float(h))
    crop_box = box
    if k < 1.0:
        w, h = max(1, int(round(w * k))), max(1, int(round(h * k)))
        ax, ay = ax * k, ay * k
    clips = {}
    for cname, (cs, ce) in spec.items():
        frames = list(range(cs, ce + 1))
        if len(frames) > 30:
            frames = [cs + round(i * (ce - cs) / 29) for i in range(30)]
        imgs = [draw_frame(tracks, f, hide).crop(crop_box) for f in frames]
        if k < 1.0:
            imgs = [im.resize((w, h), Image.BICUBIC) for im in imgs]
        cols = min(len(imgs), 8)
        sheet, cols, rows = grid_image(imgs, w, h, cols)
        rel = 'reanim/%s/%s.webp' % (role, cname)
        out = os.path.join(OUT, role, cname + '.webp')
        save_webp(sheet, out)
        clips[cname] = dict(file=rel, frames=len(imgs), cols=cols, rows=rows,
                            fps=float(FPS), bytes=os.path.getsize(out))
    print('%-14s %-22s %dx%d clips=%s' % (role, src_name, w, h, ','.join(clips)))
    return dict(w=float(w), h=float(h), ax=float(ax), ay=float(ay),
                fps=FPS, scale=1.0, clips=clips)

HEAD_TRACK_KEYS = ('anim_head_idle', 'anim_face', 'idle_mouth', 'idle_headleaf',
                   'idle_shoot_blink', 'anim_blink', 'PeaShooter_eyebrow', 'anim_shooting')


def build_repeater(role='repeater'):
    """原版 Repeater 没有独立 reanim，复用 PeaShooter.reanim。

    该文件把 body/head 分别放在同一时间轴的不同段（body idle 79..103，
    head shooting 54..78，head idle 29..53），运行时会叠一个只播
    anim_head_idle 的头层。这里按原版分层合成：idle 用 79..103 全身段，
    shoot 用 body 79..103 叠 head 54..78。
    """
    path = os.path.join(SRC, 'PeaShooter.reanim')
    if not os.path.exists(path):
        print('MISSING', role, path)
        return None
    tracks = parse(path)
    head = [(n, st) for n, st in tracks if any(k in n for k in HEAD_TRACK_KEYS)]
    body = [(n, st) for n, st in tracks if not any(k in n for k in HEAD_TRACK_KEYS)]

    def comp(fb, fh):
        c = draw_frame(body, fb)
        c.alpha_composite(draw_frame(head, fh))
        return c

    pairs = [('idle', [(79 + i, None) for i in range(25)]),
             ('shoot', [(79 + i, 54 + i) for i in range(25)])]
    frames = {}
    for cname, seq in pairs:
        ims = [draw_frame(tracks, fb) if fh is None else comp(fb, fh)
               for fb, fh in seq]
        # 原版 Repeater 复用 PeaShooter 图，但整株压成更深的绿色（单头，不是双头）。
        tinted = []
        for im in ims:
            px = im.load()
            for yy in range(im.height):
                for xx in range(im.width):
                    r, g, b, a = px[xx, yy]
                    if a:
                        px[xx, yy] = (int(r * 0.55), int(g * 0.72), int(b * 0.50), a)
            tinted.append(im)
        frames[cname] = tinted
    box = bbox_union([im for ims in frames.values() for im in ims])
    w, h = box[2] - box[0], box[3] - box[1]
    ax, ay = 1000 - box[0], 1000 - box[1]
    max_h = 260
    k = min(1.0, max_h / float(h))
    if k < 1.0:
        w, h = max(1, int(round(w * k))), max(1, int(round(h * k)))
        ax, ay = ax * k, ay * k
    clips = {}
    for cname, ims in frames.items():
        ims = [im.crop(box) for im in ims]
        if k < 1.0:
            ims = [im.resize((w, h), Image.BICUBIC) for im in ims]
        cols = min(len(ims), 5)
        sheet, cols, rows = grid_image(ims, w, h, cols)
        rel = 'reanim/%s/%s.webp' % (role, cname)
        out = os.path.join(OUT, role, cname + '.webp')
        os.makedirs(os.path.dirname(out), exist_ok=True)
        save_webp(sheet, out)
        clips[cname] = dict(file=rel, frames=len(ims), cols=cols, rows=rows,
                            fps=float(FPS), bytes=os.path.getsize(out))
    print('%-14s %-22s %dx%d clips=%s' % (role, 'PeaShooter(composite)', w, h, ','.join(clips)))
    return dict(w=float(w), h=float(h), ax=float(ax), ay=float(ay),
                fps=FPS, scale=1.0, clips=clips)


def build_golden(role, old):
    man = json.load(open(os.path.join(ROOT, 'assets', 'plants', 'reanim', 'manifest.json')))
    if old not in man:
        return None
    clips = {}
    for state, info in man[old].items():
        cname = 'special' if state == 'rise' else state
        src_png = os.path.join(ROOT, 'assets', 'plants', 'reanim', '%s_%s.png' % (old, state))
        if not os.path.exists(src_png):
            continue
        im = Image.open(src_png).convert('RGBA')
        rel = 'reanim/%s/%s.webp' % (role, cname)
        out = os.path.join(OUT, role, cname + '.webp')
        save_webp(im, out)
        clips[cname] = dict(file=rel, frames=info['frames'], cols=info['frames'],
                            rows=1, fps=float(FPS), bytes=os.path.getsize(out))
    if not clips:
        return None
    base = clips.get('idle') or clips.get('shoot') or next(iter(clips.values()))
    print('%-14s golden                  %dx%d clips=%s' % (role, base['frames'] * 0, base['frames'] * 0, ','.join(clips)))
    return dict(w=float(base['frames'] * 0 + 0), h=0.0, ax=0.0, ay=0.0, fps=FPS, scale=1.0, clips=clips)

def main():
    os.makedirs(OUT, exist_ok=True)
    chars = {}
    # 14 个可信旧条带
    gold_man = json.load(open(os.path.join(ROOT, 'assets', 'plants', 'reanim', 'manifest.json')))
    for role, old in GOLDEN.items():
        if old not in gold_man:
            continue
        clips = {}
        for state, info in gold_man[old].items():
            cname = 'special' if state == 'rise' else state
            src_png = os.path.join(ROOT, 'assets', 'plants', 'reanim', '%s_%s.png' % (old, state))
            if not os.path.exists(src_png):
                continue
            im = Image.open(src_png).convert('RGBA')
            rel = 'reanim/%s/%s.webp' % (role, cname)
            out = os.path.join(OUT, role, cname + '.webp')
            save_webp(im, out)
            clips[cname] = dict(file=rel, frames=info['frames'], cols=info['frames'],
                                rows=1, fps=float(FPS), bytes=os.path.getsize(out))
        if not clips:
            continue
        # cell 尺寸取 idle
        base = clips.get('idle') or next(iter(clips.values()))
        w = gold_man[old]['idle']['w'] if 'idle' in gold_man[old] else next(iter(gold_man[old].values()))['w']
        h = gold_man[old]['idle']['h'] if 'idle' in gold_man[old] else next(iter(gold_man[old].values()))['h']
        chars[role] = dict(w=float(w), h=float(h), ax=w / 2.0, ay=float(h),
                           fps=FPS, scale=1.0, clips=clips)
        print('%-14s golden                  %dx%d clips=%s' % (role, w, h, ','.join(clips)))

    rep = build_repeater('repeater')
    if rep:
        chars['repeater'] = rep

    for role, src in PLANTS.items():
        if role in chars:
            continue
        # 默认 idle+shoot，若 shoot 不存在就不生成
        want = ['idle', 'shoot']
        print('BAKE', role, flush=True)
        ch = build_reanim_role(role, src, want, 'plant')
        if ch:
            # 删除不存在的空 clip
            ch['clips'] = {k: v for k, v in ch['clips'].items() if v['frames'] > 0}
            chars[role] = ch

    for role, src in ZOMBIES.items():
        if role == 'boss':
            continue
        hide = list(HIDE_BASE)
        if role != 'screendoor':
            hide += HIDE_SCREENDOOR
        if role != 'cone':
            hide += HIDE_CONE
        if role != 'bucket':
            hide += HIDE_BUCKET
        if role != 'duckytube':
            hide += HIDE_DUCK
        if role == 'gargantuar':
            hide += ['imp']
        if role == 'flag':
            hide = [h for h in hide if h != 'Zombie_flaghand']
        want = ['walk', 'eat', 'die', 'special']
        ch = build_reanim_role(role, src, want, 'zombie', hide)
        if ch:
            # 只保留成功的 clip；零帧 clip 去掉
            ch['clips'] = {k: v for k, v in ch['clips'].items() if v['frames'] > 0}
            chars[role] = ch

    # Boss 的 reanim 是分体式机器人（身体/头部/手臂分布在不同动画段），
    # 单独烘焙会出现无头或白块；沿用已验收的原版整身立绘，避免退化。
    boss_png = os.path.join(ROOT, 'assets', 'zombies', 'frames', 'boss_0.webp')
    if os.path.exists(boss_png) and 'boss' not in chars:
        im = Image.open(boss_png).convert('RGBA')
        n = 12
        w, h = im.width, im.height
        sheet = Image.new('RGBA', (w * n, h), (0, 0, 0, 0))
        for i in range(n):
            sheet.alpha_composite(im, (i * w, 0))
        rel = 'reanim/boss/idle.webp'
        out = os.path.join(OUT, 'boss', 'idle.webp')
        os.makedirs(os.path.dirname(out), exist_ok=True)
        save_webp(sheet, out)
        chars['boss'] = dict(w=float(w), h=float(h), ax=w / 2.0, ay=float(h),
                             fps=FPS, scale=1.0, clips={'idle': dict(
                                 file=rel, frames=n, cols=n, rows=1, fps=float(FPS),
                                 bytes=os.path.getsize(out))})
        print('%-14s static boss            %dx%d' % ('boss', w, h))

    # 这几个角色原版素材是分体/水下分镜，单帧合成会缺件；
    # 交给已验收的旧绘制/静态立绘回退，避免上线出现碎片。
    for _drop in ('tanglekelp', 'lilypad'):
        chars.pop(_drop, None)
    man = dict(fps=FPS, chars=chars)
    with open(os.path.join(OUT, 'manifest.json'), 'w', encoding='utf-8') as fh:
        json.dump(man, fh, ensure_ascii=False, indent=2)
    total = sum(c['bytes'] for ch in chars.values() for c in ch['clips'].values())
    print('TOTAL %d chars, %d clips, %.2f MB' % (len(chars), sum(len(c['clips']) for c in chars.values()), total / 1048576))

if __name__ == '__main__':
    main()
