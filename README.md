## wordgame-cli
### Type words as fast as you can in your terminal

#### How to setup?

##### Build the project and run the binary

- Install [rust](https://www.rust-lang.org/tools/install) if you haven't yet.

- Clone the repo. Cd into the repo.

- Run `cargo build --release`

- Go into your ~/.zshrc (or equivalent): run `code -r ~/.zshrc`.
And add to the alias list: 

```bash
# .zshrc

alias wordgame="~/<YOUR_PATH_TO_THE_REPO>/typeword-game/target/release/typeword-game"
```

- Restart your terminal

- Type wordgame in your terminal. That's it, enjoy!

