#!/usr/bin/env python3
"""把 WebP 素材与音频打成 tar 单包；条目路径统一为 assets/...，与页面 key 一致。"""
import os, sys, tarfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, 'dist')


def add_tree(tar, base, arc_prefix, dirs):
    for d in dirs:
        root = os.path.join(base, d)
        if not os.path.isdir(root):
            raise SystemExit('missing dir: ' + root)
        for b, _, files in os.walk(root):
            for f in files:
                p = os.path.join(b, f)
                rel = os.path.relpath(p, base).replace(os.sep, '/')
                ti = tar.gettarinfo(p, arcname=arc_prefix + rel)
                ti.uid = ti.gid = 0
                ti.uname = ti.gname = ''
                with open(p, 'rb') as fh:
                    tar.addfile(ti, fh)


def build(name, base, dirs):
    path = os.path.join(OUT, name)
    with tarfile.open(path, 'w', format=tarfile.USTAR_FORMAT) as tar:
        add_tree(tar, base, 'assets/', dirs)
    print(name, os.path.getsize(path))


if __name__ == '__main__':
    build_ui = ['bg/day.webp', 'plants/sunflower.webp', 'plants/peashooter.webp', 'plants/wallnut.webp', 'plants/cherrybomb.webp', 'plants/snowpea.webp', 'projectiles/pea.webp']
    boot_files = ['pvz_rs.js', 'pvz_rs_bg.wasm']
    path = os.path.join(OUT, 'assets-ui.tar')
    with tarfile.open(path, 'w', format=tarfile.USTAR_FORMAT) as tar:
        for rel in build_ui:
            p = os.path.join(ROOT, 'assets-webp', rel)
            ti = tar.gettarinfo(p, arcname='assets/' + rel)
            ti.uid = ti.gid = 0
            ti.uname = ti.gname = ''
            with open(p, 'rb') as fh:
                tar.addfile(ti, fh)
        for rel in boot_files:
            p = os.path.join(OUT, rel)
            ti = tar.gettarinfo(p, arcname='boot/' + rel)
            ti.uid = ti.gid = 0
            ti.uname = ti.gname = ''
            with open(p, 'rb') as fh:
                tar.addfile(ti, fh)
    print('assets-ui.tar', os.path.getsize(path))
    build('assets-core.tar', os.path.join(ROOT, 'assets-webp'), ['plants', 'bg', 'projectiles'])
    build('assets-zombies.tar', os.path.join(ROOT, 'assets-webp'), ['zombies'])
    build('assets-audio.tar', os.path.join(ROOT, 'assets'), ['audio'])
    # GitHub Pages 对单次大文件传输偶发断连；按体积均分两包，页面并行拉取后合并。
    import glob
    reanim_files = []
    for p in glob.glob(os.path.join(ROOT, 'assets', 'reanim', '*')):
        if os.path.isfile(p):
            reanim_files.append((os.path.getsize(p), p))
        elif os.path.isdir(p):
            for b, _, fs in os.walk(p):
                for f in fs:
                    fp = os.path.join(b, f)
                    reanim_files.append((os.path.getsize(fp), fp))
    # 6MB 分包在 GitHub Pages 上仍会被 CDN 截断，拆成 8 个约 1.5MB 的小包更稳。
    bins = [[] for _ in range(8)]
    for size, p in sorted(reanim_files, reverse=True):
        i = min(range(len(bins)), key=lambda j: sum(x[0] for x in bins[j]))
        bins[i].append((size, p))
    for idx, items in enumerate(bins):
        out = os.path.join(OUT, f'assets-reanim-{idx}.tar')
        with tarfile.open(out, 'w', format=tarfile.USTAR_FORMAT) as tar:
            for _, p in items:
                rel = os.path.relpath(p, os.path.join(ROOT, 'assets')).replace(os.sep, '/')
                ti = tar.gettarinfo(p, arcname='assets/' + rel)
                ti.uid = ti.gid = 0
                ti.uname = ti.gname = ''
                with open(p, 'rb') as fh:
                    tar.addfile(ti, fh)
        print(f'assets-reanim-{idx}.tar', os.path.getsize(out), len(items))
