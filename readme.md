# Snake-rs :snake:

This is a simple implementation of [snake](https://en.wikipedia.org/wiki/Snake_(video_game_genre)) written in ***rust*** (:crab:)
using [ggez](https://github.com/ggez/ggez).  
**This project has three goals:**
1. Learn rust.
2. Learn game-dev.
3. Have fun! :sunglasses:

**It's currently in a playable state, but I want to add/improve the following:**
1. ~~The graphics are currently simple rectangles. Add textures for the snake, food and (maybe?) background.~~
   * ~~If we want to get really fancy, we can maybe add moving grass...?~~
2. Add sound effects.
3. The movement is currently choppy (only 8 updates a second) and should be made smoother.
4. Add animation for the snake (slithering, eating food) and maybe to the food as well.

**After (if :sweat_smile:) I finish the above, I think it would be fun to expand the game further:**
* Add powerups (think *Achtung, die Kurve!*).
* Add competition elements with pve and pvp(local, online).
* GUI (maybe [iced](https://github.com/hecrj/iced), maybe [imgui](https://github.com/Gekkio/imgui-rs)).

## Getting Started

To play the game, simply run:

```shell
git clone https://github.com/ceranco/snake-rs.git
cd snake-rs
cargo run --release
```


## License

This project is licensed under the MIT License.
