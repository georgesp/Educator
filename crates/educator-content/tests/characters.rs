//! SUB000101: the character set is complete, valid and drawn with the shared palette.

use educator_content::{validate_svg, Characters};
use educator_core::characters::{CharacterCatalog, CharacterKind};
use regex::Regex;
use std::path::{Path, PathBuf};

fn characters_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/characters")
}

fn catalog() -> Characters {
    let loaded = Characters::load(&characters_dir());
    assert!(loaded.problems.is_empty(), "{:?}", loaded.problems);
    loaded.value
}

#[test]
fn at_least_eleven_characters_five_animals_and_three_boys_three_girls() {
    let catalog = catalog();
    let mut animals: Vec<&str> = catalog
        .all()
        .iter()
        .filter(|c| c.kind == CharacterKind::Animal)
        .map(|c| c.id.as_str())
        .collect();
    animals.sort_unstable();

    assert!(catalog.all().len() >= 11);
    assert_eq!(vec!["cochon", "lapin", "lion", "ours", "renard"], animals);
    assert!(
        catalog
            .all()
            .iter()
            .filter(|c| c.id.starts_with("garcon-"))
            .count()
            >= 3
    );
    assert!(
        catalog
            .all()
            .iter()
            .filter(|c| c.id.starts_with("fille-"))
            .count()
            >= 3
    );
}

#[test]
fn every_character_has_its_valid_svg_and_anchors_inside_the_view_box() {
    for definition in catalog().definitions() {
        validate_svg(&characters_dir().join(&definition.file))
            .unwrap_or_else(|p| panic!("{}: {p}", definition.file));
        for point in [
            definition.anchors.head_top,
            definition.anchors.eyes,
            definition.anchors.neck,
        ] {
            assert!(
                (0.0..=256.0).contains(&point.x) && (0.0..=256.0).contains(&point.y),
                "{}",
                definition.file
            );
        }
    }
}

#[test]
fn every_svg_file_is_listed() {
    let mut listed: Vec<String> = catalog()
        .definitions()
        .iter()
        .map(|d| d.file.clone())
        .collect();
    let mut files: Vec<String> = std::fs::read_dir(characters_dir())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".svg"))
        .collect();
    listed.sort();
    files.sort();

    assert_eq!(files, listed);
}

#[test]
fn colors_come_from_the_palette() {
    let palette: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(characters_dir().join("palette.json")).unwrap(),
    )
    .unwrap();
    let colors = palette["colors"].as_object().unwrap();
    let variable = Regex::new(r"var\(--(?<name>[a-z-]+), (?<value>#[0-9A-F]{6})\)").unwrap();
    let literal = Regex::new(r"#[0-9A-Fa-f]{3,6}\b").unwrap();
    let catalog = catalog();

    for definition in catalog.definitions() {
        let svg = catalog.svg(&definition.info.id).unwrap();
        let uses: Vec<_> = variable.captures_iter(svg).collect();
        assert!(!uses.is_empty(), "{}", definition.file);
        for capture in uses {
            let name = &capture["name"];
            let expected = colors
                .get(name)
                .unwrap_or_else(|| panic!("{}: unknown palette color {name}", definition.file));
            assert_eq!(
                expected.as_str().unwrap(),
                &capture["value"],
                "{}",
                definition.file
            );
        }

        assert!(
            !literal.is_match(&variable.replace_all(svg, "")),
            "{}: literal color outside the palette",
            definition.file
        );
    }
}

#[test]
fn invalid_svg_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("test.svg");

    std::fs::write(&file, r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><g id="head"/><script>alert(1)</script></svg>"#).unwrap();
    assert!(validate_svg(&file).is_err());

    std::fs::write(&file, r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><g id="body"/><g id="head"/><g id="face"/><g id="clothes"/><g id="ears"/><a href="https://example.com"/></svg>"#).unwrap();
    assert!(validate_svg(&file).unwrap_err().contains("Forbidden"));
}
