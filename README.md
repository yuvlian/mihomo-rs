## how to add to dependencies
```
mihomo_rs = { git = "https://github.com/yuvlian/mihomo-rs" }
```

## example usage
```
use mihomo_rs::mihomo;
use mihomo_rs::Language;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uid = 802775147;
    let language = Language::EN;

    let sr_data = mihomo(uid, language).await?;

    let player_name = sr_data.player.name;
    let character_names: Vec<String> = sr_data.characters
        .iter()
        .map(|c| c.name.clone())
        .collect();
    let achievement_cnt = sr_data.player.space_info.achievement_count;

    println!("Player name: {}", player_name);
    println!("Displayed characters: {}", character_names.join(", "));
    println!("Achievement obtained: {}", achievement_cnt);

    Ok(())
}
```
