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
    build('assets-core.tar', os.path.join(ROOT, 'assets-webp'), ['plants', 'bg', 'projectiles'])
    build('assets-zombies.tar', os.path.join(ROOT, 'assets-webp'), ['zombies'])
    build('assets-audio.tar', os.path.join(ROOT, 'assets'), ['audio'])
