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
    # GitHub Pages 单次大文件传输偶发断连，单包上限压到 ~1.3MB。
    # 植物（约 3.8MB）必须首屏就绪，僵尸（约 8.4MB）后台并行拉，先出画面再补海。
    import glob, json
    man = json.load(open(os.path.join(ROOT, 'assets', 'reanim', 'manifest.json'), encoding='utf-8'))
    kind_of = {}
    for name, c in man['chars'].items():
        k = 'plant' if ('idle' in c['clips'] or 'shoot' in c['clips']) else 'zombie'
        for clip in c['clips'].values():
            kind_of[clip['file']] = k
    groups = {'p': [], 'z': []}
    for p in glob.glob(os.path.join(ROOT, 'assets', 'reanim', '*')):
        files = [p] if os.path.isfile(p) else [
            os.path.join(b, f) for b, _, fs in os.walk(p) for f in fs]
        for fp in files:
            rel = os.path.relpath(fp, os.path.join(ROOT, 'assets')).replace(os.sep, '/')
            groups[kind_of.get(rel, 'z')[0]].append((os.path.getsize(fp), fp))
    LIMIT = 1350 * 1024
    for prefix, items in groups.items():
        bins = []
        for size, path in sorted(items, reverse=True):
            target = next((b for b in bins if sum(x[0] for x in b) + size <= LIMIT), None)
            if target is None:
                bins.append([])
                target = bins[-1]
            target.append((size, path))
        for idx, items_bin in enumerate(bins):
            out = os.path.join(OUT, f'assets-reanim-{prefix}{idx}.tar')
            with tarfile.open(out, 'w', format=tarfile.USTAR_FORMAT) as tar:
                for _, path in items_bin:
                    rel = os.path.relpath(path, os.path.join(ROOT, 'assets')).replace(os.sep, '/')
                    ti = tar.gettarinfo(path, arcname='assets/' + rel)
                    ti.uid = ti.gid = 0
                    ti.uname = ti.gname = ''
                    with open(path, 'rb') as fh:
                        tar.addfile(ti, fh)
            print(f'assets-reanim-{prefix}{idx}.tar', os.path.getsize(out), len(items_bin))
