# 植物大战僵尸 · Rust 重制版

用 Rust 重写《植物大战僵尸》的经典草坪关卡，编译成 WebAssembly，在浏览器里直接用 Canvas 2D 实时绘制——**没有任何贴图资源**，房子、草坪、植物、僵尸、阳光全部由代码画出来。

在线试玩：<https://nana-smy.github.io/pvz-rs/>

## 玩法

- 5 行 × 9 列草坪，撑过 10 波僵尸，别让它们走进房子。
- 点击顶部卡片选择植物，再点草坪格子种下；右键取消选择。
- 阳光：天上会掉，向日葵也会产；**点一下就能收**，16 秒不捡会消失。
- 铲子可以铲掉种错的植物。
- 每行末尾有一台割草机，僵尸靠近时自动冲出去清掉这一行的僵尸，只能用一次。

## 植物 / 僵尸

| 植物 | 阳光 | 作用 |
| --- | --- | --- |
| 向日葵 | 50 | 每 20 秒产 25 阳光 |
| 豌豆射手 | 100 | 单发豌豆，20 伤害 |
| 坚果墙 | 50 | 3000 血，挡在前面 |
| 樱桃炸弹 | 150 | 3×3 范围 1800 伤害 |
| 寒冰射手 | 175 | 命中减速 55% |
| 双发射手 | 200 | 一次两发 |

僵尸有普通、路障、铁桶、旗帜四种，血量和速度各不相同；第 5、10 波是旗帜波，最后一波会刷 15 只。

## 操作

- 鼠标：选卡 / 种植 / 收阳光 / 点按钮
- `空格` 暂停或继续
- `R` 重开
- `Esc` 取消选择 / 暂停
- `M` 静音
- `D` 调试信息（开发用）

## 技术栈

- **Rust** 负责全部游戏逻辑（状态机、波次、碰撞、经济、音效触发）
- **wasm-bindgen + web-sys** 做浏览器绑定
- **Canvas 2D** 纯代码绘制，没有 WebGL 依赖
- **WebAudio** 振荡器实时合成音效
- 逻辑分辨率 1280×720，页面按 16:9 等比缩放，支持窗口尺寸变化

项目结构：

```
src/defs.rs     常量、数值、UI 布局
src/game.rs     游戏状态机与全部玩法逻辑
src/render.rs   Canvas 2D 绘制
src/sfx.rs      WebAudio 合成音效
src/main.rs     wasm 入口、输入事件、requestAnimationFrame 主循环
web/index.html  页面外壳、加载动画、等比缩放
```

## 本地运行

前置：Rust stable、`wasm32-unknown-unknown` target、`wasm-bindgen-cli 0.2.130`。

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130   # 或下载 release 里的预编译包
./build.sh
python3 -m http.server 8765 --directory dist
# 打开 http://localhost:8765/
```

## 部署

仓库已配置 `.github/workflows/pages.yml`：推送到 `main` 后自动编译 wasm、生成 `dist/` 并发布到 GitHub Pages（Source 选 GitHub Actions）。也可以在仓库 Settings → Pages 里手动触发 workflow。

## License

MIT
