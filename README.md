## wordgame-cli
### Type words as fast as you can in your terminal

![wordgame-cli](https://user-images.githubusercontent.com/66871571/189606264-daced8b1-866f-44f7-be2b-00dda12f5ad9.gif)

#### How to setup?

##### Build the project and run the binary

- Install [rust](https://www.rust-lang.org/tools/install) if you haven't yet.

- Clone the repo. Cd into the repo.

- Run `cargo build --release`

- Go into your ~/.zshrc (or equivalent): run `code -r ~/.zshrc`.
And add to the alias list: 

```bash
# .zshrc

alias wordgame="~/<YOUR_PATH_TO_THE_REPO>/wordgame-cli/target/release/typeword-game"
```

- Restart your terminal

- Type wordgame from anywhere in your terminal. That's it, enjoy!

