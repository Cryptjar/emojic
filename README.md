# emojic 😀 🙂 😇

[![Crates.io](https://img.shields.io/crates/v/emojic.svg)](https://crates.io/crates/emojic)
[![Documentation](https://docs.rs/emojic/badge.svg)](https://docs.rs/emojic)
[![License](https://img.shields.io/github/license/orhanbalci/emojic.svg)](https://github.com/orhanbalci/emojic/blob/master/LICENSE)

<!-- cargo-sync-readme start -->


Emoji constants for your rusty strings. This crate is inspired by the Go library
[emoji](https://github.com/enescakir/emoji) written by
[@enescakir](https://github.com/enescakir).


## 📦 Cargo.toml

```toml
[dependencies]
emojic = "0.3"
```

## 🔧 Example

```rust
use emojic::Gender;
use emojic::Pair;
use emojic::Tone;
use emojic::flat::*;

println!("Hello {}", WAVING_HAND);
println!(
    "I'm {} from {}",
    TECHNOLOGIST.gender(Gender::Male),
    FLAG_TURKEY
);
println!(
    "Different skin tones default {} light {} dark {}",
    THUMBS_UP,
    OK_HAND.tone(Tone::Light),
    CALL_ME_HAND.tone(Tone::Dark)
);
println!(
    "Multiple skin tones: default: {}, same: {} different: {}",
    PERSON_HOLDING_HANDS,
    PERSON_HOLDING_HANDS.tone(Tone::Medium),
    PERSON_HOLDING_HANDS.tone((Tone::Light, Tone::Dark))
);
println!(
    "Different sexes: default: {} male: {}, female: {}",
    GENIE,
    GENIE.gender(Gender::Male),
    GENIE.gender(Gender::Female),
);
println!(
    "Mixing attributes: men & light: {} and women & drak: {}",
    PERSON_TIPPING_HAND.gender(Gender::Male).tone(Tone::Light),
    PERSON_TIPPING_HAND.gender(Gender::Female).tone(Tone::Dark),
);
```

## 🖨️ Output

```text
Hello 👋
I'm 👨‍💻 from 🇹🇷
Different skin tones default 👍 light 👌🏻 dark 🤙🏿
Multiple skin tones: default: 🧑‍🤝‍🧑, same: 🧑🏽‍🤝‍🧑🏽 different: 🧑🏻‍🤝‍🧑🏿
Different sexes: default: 🧞 male: 🧞‍♂️, female: 🧞‍♀️
Mixing attributes: men & light: 💁🏻‍♂️ and women & drak: 💁🏿‍♀️
```

This crate contains emojis constants based on the
[Full Emoji List v13.1](https://unicode.org/Public/emoji/13.1/emoji-test.txt).
Including its categorization:

```rust
assert_eq!(
    emojic::grouped::people_and_body::hands::OPEN_HANDS, //🤲
    emojic::flat::OPEN_HANDS, //🤲
);
```

As well as iterators to list all the emojis in each group and subgroup:

```rust
// Iterates all hand emoji: 👏, 🙏, 🤝, 👐, 🤲, 🙌
emojic::grouped::people_and_body::hands::base_emojis()
```

Finally, it has additional emoji aliases from [github/gemoji](https://github.com/github/gemoji).

```rust
parse_alias(":+1:") // 👍
parse_alias(":100:") // 💯
parse_alias(":woman_astronaut:") // 👩‍🚀
```

## 🔭 Examples

For more examples have a look at the
[examples](https://github.com/orhanbalci/emojic/tree/master/examples) folder.



<!-- cargo-sync-readme end -->

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
