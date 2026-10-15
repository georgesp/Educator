//! Avatar accessories (SUB001504): every accessory fits every character, and the control board
//! `content/accessories/planche.html` shows them all (regenerated with
//! `EDUCATOR_WRITE_PLANCHE=1 cargo test -p educator-content --test accessories`).

use educator_content::EducatorContent;
use educator_core::characters::CharacterCatalog;
use educator_core::rewards::accessories::{
    compose_avatar, AccessorySlot, Unlock, WornAccessory, COMPANION_POINT,
};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn content_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

fn content() -> &'static EducatorContent {
    static CONTENT: OnceLock<EducatorContent> = OnceLock::new();
    CONTENT.get_or_init(|| EducatorContent::load(&content_dir()))
}

/// `(width, height)` of the `viewBox` of an accessory file.
fn size_of(file: &Path) -> (f64, f64) {
    let svg = std::fs::read_to_string(file).unwrap();
    let document = roxmltree::Document::parse(&svg).unwrap();
    let view_box: Vec<f64> = document
        .root_element()
        .attribute("viewBox")
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    (view_box[2], view_box[3])
}

#[test]
fn there_are_at_least_twenty_accessories_in_every_slot() {
    let catalog = &content().accessories;
    assert!(
        catalog.accessories.len() >= 20,
        "{}",
        catalog.accessories.len()
    );
    for slot in AccessorySlot::ALL {
        assert!(
            catalog.accessories.iter().any(|a| a.slot == slot),
            "nothing for {slot:?}"
        );
    }
    // Something to wear from the start.
    assert!(catalog
        .accessories
        .iter()
        .any(|a| a.unlock == Unlock::Level(1)));
}

#[test]
fn every_accessory_fits_every_character() {
    let content = content();
    let index: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(content_dir().join("accessories/accessories.json")).unwrap(),
    )
    .unwrap();
    for accessory in &content.accessories.accessories {
        let file = index["accessories"]
            .as_array()
            .unwrap()
            .iter()
            .find(|a| a["id"] == accessory.id.as_str())
            .unwrap()["file"]
            .as_str()
            .unwrap()
            .to_owned();
        let (width, height) = size_of(&content_dir().join("accessories").join(file));
        for character in content.characters.all() {
            let anchors = content.characters.anchors(&character.id).unwrap();
            let target = match accessory.slot {
                AccessorySlot::Head => anchors.head_top,
                AccessorySlot::Eyes => anchors.eyes,
                AccessorySlot::Neck => anchors.neck,
                AccessorySlot::Companion => COMPANION_POINT,
            };
            let left = target.x - accessory.anchor.x;
            let top = target.y - accessory.anchor.y;
            // The avatar's drawing area: -12 -24 280 280. Around the neck, an accessory may go down
            // to the bottom edge, like the clothes (a cape).
            let bottom_ok = accessory.slot == AccessorySlot::Neck || top + height <= 256.0;
            assert!(
                left >= -12.0 && top >= -24.0 && left + width <= 268.0 && bottom_ok,
                "{} on {} is cut: {left} {top} {width} {height}",
                accessory.id,
                character.id
            );
        }
    }
}

#[test]
fn composed_avatars_are_valid_svg() {
    let content = content();
    for character in content.characters.all() {
        let svg = content.characters.svg(&character.id).unwrap();
        let anchors = content.characters.anchors(&character.id).unwrap();
        for slot in AccessorySlot::ALL {
            let worn: Vec<_> = content
                .accessories
                .accessories
                .iter()
                .filter(|a| a.slot == slot)
                .map(|a| (a, content.accessories.color(&a.colors[0]).unwrap()))
                .collect();
            for item in &worn {
                let avatar = compose_avatar(svg, &anchors, std::slice::from_ref(item));
                roxmltree::Document::parse(&avatar)
                    .unwrap_or_else(|e| panic!("{} on {}: {e}", item.0.id, character.id));
            }
        }
    }
}

/// The control board: each accessory on each character, in its first color, at 128 and 48 px.
#[test]
fn the_control_board_is_up_to_date() {
    let content = content();
    let mut rows = String::new();
    for accessory in &content.accessories.accessories {
        let worn = [WornAccessory::new(&accessory.id, &accessory.colors[0])];
        let mut cells = String::new();
        for character in content.characters.all() {
            let svg = content.characters.svg(&character.id).unwrap();
            let anchors = content.characters.anchors(&character.id).unwrap();
            let items: Vec<_> = worn
                .iter()
                .filter_map(|w| {
                    let a = content.accessories.find(&w.accessory_id)?;
                    Some((a, content.accessories.color(&w.color_id)?))
                })
                .collect();
            let avatar = compose_avatar(svg, &anchors, &items).replacen(
                r#"width="256" height="256""#,
                r#"width="96" height="96""#,
                1,
            );
            cells.push_str(&format!("<td>{avatar}</td>"));
        }
        rows.push_str(&format!(
            "<tr><th>{}<br><small>{} · {}</small></th>{cells}</tr>\n",
            accessory.name,
            accessory.slot.as_str(),
            match &accessory.unlock {
                Unlock::Level(level) => format!("niveau {level}"),
                Unlock::Badge(badge) => format!("badge {badge}"),
            }
        ));
    }
    let header: String = content
        .characters
        .all()
        .iter()
        .map(|c| format!("<th>{}</th>", c.name))
        .collect();
    let html = format!(
        r#"<!doctype html>
<html lang="fr">
<head>
  <meta charset="utf-8">
  <title>Planche de contrôle des accessoires</title>
  <!-- SUB001504 : chaque accessoire sur chaque personnage, dans sa première couleur, tel que le cœur Rust
       compose l'avatar. Fichier généré : EDUCATOR_WRITE_PLANCHE=1 cargo test -p educator-content --test accessories -->
  <style>
    body {{ margin: 0; padding: 24px; font-family: system-ui, sans-serif; background: #f3f3f5; color: #1a1a1f; }}
    table {{ border-collapse: separate; border-spacing: 6px; }}
    th {{ font-size: 13px; text-align: left; }}
    td {{ background: #fff; border-radius: 10px; }}
    svg {{ display: block; }}
  </style>
</head>
<body>
  <h1>Accessoires — planche de contrôle</h1>
  <table>
    <tr><th></th>{header}</tr>
{rows}  </table>
</body>
</html>
"#
    );
    let path = content_dir().join("accessories/planche.html");
    if std::env::var_os("EDUCATOR_WRITE_PLANCHE").is_some() {
        std::fs::write(&path, &html).unwrap();
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    assert!(
        committed == html,
        "content/accessories/planche.html is out of date: EDUCATOR_WRITE_PLANCHE=1 cargo test -p educator-content --test accessories"
    );
}
