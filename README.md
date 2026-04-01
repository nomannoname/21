# wasm-blackjack

一个使用 Rust + WebAssembly（wasm-bindgen）实现的简化版 Blackjack（21点）核心逻辑。

## 功能

- 洗牌并发牌（玩家和庄家各两张）
- 玩家 `hit`（要牌）
- 玩家 `stand`（停牌）后庄家自动补牌（小于17继续要牌）
- A 可按 11/1 自动调整
- 返回游戏状态（进行中、爆牌、胜负、平局）

## 构建

```bash
cargo test
cargo build --target wasm32-unknown-unknown
```

推荐用 `wasm-pack`：

```bash
wasm-pack build --target web
```

## JS 侧示例

```javascript
import init, { BlackjackGame, GameStatus } from "./pkg/wasm_blackjack.js";

await init();
const game = new BlackjackGame();

console.log(game.player_cards(), game.dealer_total_visible());

game.hit();
if (game.status() === GameStatus.PlayerBust) {
  console.log("玩家爆牌");
}

game.stand();
console.log(game.status(), game.dealer_cards(), game.player_total());
```
