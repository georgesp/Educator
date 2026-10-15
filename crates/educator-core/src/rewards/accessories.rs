//! Avatar accessories (SUB001504): hats, glasses, scarves, capes and small companions drawn in the
//! style of the characters (`content/accessories`), unlocked by a level or a badge. The avatar of a
//! profile is its character and the accessories it wears (one per slot), each in one of its colors;
//! it is drawn by stacking SVG layers on the anchor points of the character.

use std::fmt;

/// Where an accessory is worn; one accessory per slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessorySlot {
    /// On the top of the head (`headTop`).
    Head,
    /// On the eyes (`eyes`).
    Eyes,
    /// Around the neck (`neck`): scarves, bow ties, capes.
    Neck,
    /// Next to the character, at the bottom right.
    Companion,
}

impl AccessorySlot {
    /// Order of the slots on the customization screen.
    pub const ALL: [AccessorySlot; 4] = [
        AccessorySlot::Head,
        AccessorySlot::Eyes,
        AccessorySlot::Neck,
        AccessorySlot::Companion,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            AccessorySlot::Head => "head",
            AccessorySlot::Eyes => "eyes",
            AccessorySlot::Neck => "neck",
            AccessorySlot::Companion => "companion",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == name)
    }

    /// Drawing order of the front layers: the scarf under the glasses, the glasses under the hat,
    /// the companion last.
    fn order(self) -> u8 {
        match self {
            AccessorySlot::Neck => 0,
            AccessorySlot::Eyes => 1,
            AccessorySlot::Head => 2,
            AccessorySlot::Companion => 3,
        }
    }
}

/// Drawn behind the character (a cape) or in front of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessoryLayer {
    Back,
    Front,
}

/// A point in the 256 × 256 drawing of a character, or in the drawing of an accessory.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Where the accessories attach on a character (`content/characters/characters.json`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CharacterAnchors {
    pub head_top: Point,
    pub eyes: Point,
    pub neck: Point,
}

/// Where a companion stands: its anchor (the middle of its feet) goes there.
pub const COMPANION_POINT: Point = Point::new(222.0, 252.0);

/// The drawing area of an avatar, larger than the 256 × 256 of a character so that a tall hat or a
/// companion is never cut: `x y width height` of the `viewBox`. The same for every avatar, worn
/// accessories or not, so that the character never changes size.
pub const AVATAR_VIEW_BOX: &str = "-12 -24 280 280";

/// A color of the accessories: the main one and a lighter one (shine, stripes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessoryColor {
    pub id: String,
    /// French name (« Rouge »).
    pub name: String,
    /// `#RRGGBB`.
    pub main: String,
    pub light: String,
}

/// How an accessory is unlocked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unlock {
    /// From this level (1: from the start).
    Level(u32),
    /// With this badge.
    Badge(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Accessory {
    pub id: String,
    /// French name (« Chapeau de magicien »).
    pub name: String,
    pub slot: AccessorySlot,
    pub layer: AccessoryLayer,
    /// Point of the accessory's drawing placed on the anchor of its slot.
    pub anchor: Point,
    /// Content of the accessory's SVG (without the `<svg>` element), using `var(--acc)` and
    /// `var(--acc-light)` for its colors.
    pub content: String,
    /// Ids of the colors offered, the first being the default one.
    pub colors: Vec<String>,
    pub unlock: Unlock,
}

/// The accessories of the content and their colors.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessoryCatalog {
    pub accessories: Vec<Accessory>,
    pub colors: Vec<AccessoryColor>,
}

impl AccessoryCatalog {
    pub fn find(&self, id: &str) -> Option<&Accessory> {
        self.accessories.iter().find(|a| a.id == id)
    }

    pub fn color(&self, id: &str) -> Option<&AccessoryColor> {
        self.colors.iter().find(|c| c.id == id)
    }

    /// The accessories unlocked at `level` with `badges`.
    pub fn unlocked_by<'a>(
        &'a self,
        level: u32,
        badges: &'a [String],
    ) -> impl Iterator<Item = &'a Accessory> + 'a {
        self.accessories.iter().filter(move |a| match &a.unlock {
            Unlock::Level(needed) => level >= *needed,
            Unlock::Badge(badge) => badges.contains(badge),
        })
    }
}

/// An accessory worn by a profile, in one of its colors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WornAccessory {
    pub accessory_id: String,
    pub color_id: String,
}

impl WornAccessory {
    pub fn new(accessory_id: impl Into<String>, color_id: impl Into<String>) -> Self {
        Self {
            accessory_id: accessory_id.into(),
            color_id: color_id.into(),
        }
    }
}

/// Why an outfit is refused (English: a mistake of the front, never shown to the child).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutfitProblem(pub String);

impl fmt::Display for OutfitProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Checks an outfit: known accessories, one per slot, offered colors, all unlocked.
pub fn check_outfit(
    catalog: &AccessoryCatalog,
    worn: &[WornAccessory],
    unlocked: &[String],
) -> Result<(), OutfitProblem> {
    let mut slots = Vec::new();
    for item in worn {
        let accessory = catalog
            .find(&item.accessory_id)
            .ok_or_else(|| OutfitProblem(format!("Unknown accessory {}.", item.accessory_id)))?;
        if slots.contains(&accessory.slot) {
            return Err(OutfitProblem(format!(
                "Two accessories in the slot {}.",
                accessory.slot.as_str()
            )));
        }
        slots.push(accessory.slot);
        if !accessory.colors.contains(&item.color_id) {
            return Err(OutfitProblem(format!(
                "Accessory {} has no color {}.",
                accessory.id, item.color_id
            )));
        }
        if !unlocked.contains(&accessory.id) {
            return Err(OutfitProblem(format!(
                "Accessory {} is not unlocked.",
                accessory.id
            )));
        }
    }
    Ok(())
}

/// The accessories of an outfit still valid with the content (an accessory or a color removed
/// from the content is left out, never an error in front of the child).
pub fn wearable<'a>(
    catalog: &'a AccessoryCatalog,
    worn: &[WornAccessory],
) -> Vec<(&'a Accessory, &'a AccessoryColor)> {
    let mut items: Vec<(&Accessory, &AccessoryColor)> = Vec::new();
    for item in worn {
        let Some(accessory) = catalog.find(&item.accessory_id) else {
            continue;
        };
        if items.iter().any(|(a, _)| a.slot == accessory.slot) {
            continue;
        }
        let color = if accessory.colors.contains(&item.color_id) {
            catalog.color(&item.color_id)
        } else {
            accessory.colors.first().and_then(|c| catalog.color(c))
        };
        if let Some(color) = color {
            items.push((accessory, color));
        }
    }
    items
}

/// The inner content of an SVG document (between `<svg …>` and `</svg>`), or the text itself.
pub fn svg_content(svg: &str) -> &str {
    let Some(start) = svg.find("<svg") else {
        return svg;
    };
    let Some(open_end) = svg[start..].find('>').map(|i| start + i + 1) else {
        return svg;
    };
    let close = svg.rfind("</svg>").unwrap_or(svg.len());
    if close < open_end {
        return "";
    }
    &svg[open_end..close]
}

/// The avatar as one SVG: the accessories behind, the character, then the accessories in front,
/// each moved so that its anchor lies on the anchor of its slot and colored through `--acc` and
/// `--acc-light`. Vector drawing: the same at 48 px and at 256 px.
pub fn compose_avatar(
    character_svg: &str,
    anchors: &CharacterAnchors,
    worn: &[(&Accessory, &AccessoryColor)],
) -> String {
    let mut back: Vec<&(&Accessory, &AccessoryColor)> = worn
        .iter()
        .filter(|(a, _)| a.layer == AccessoryLayer::Back)
        .collect();
    let mut front: Vec<&(&Accessory, &AccessoryColor)> = worn
        .iter()
        .filter(|(a, _)| a.layer == AccessoryLayer::Front)
        .collect();
    back.sort_by_key(|(a, _)| a.slot.order());
    front.sort_by_key(|(a, _)| a.slot.order());

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{AVATAR_VIEW_BOX}" width="256" height="256">"#
    );
    for (accessory, color) in back {
        svg.push_str(&layer(accessory, color, anchors));
    }
    svg.push_str(r#"<g class="avatar-character">"#);
    svg.push_str(svg_content(character_svg));
    svg.push_str("</g>");
    for (accessory, color) in front {
        svg.push_str(&layer(accessory, color, anchors));
    }
    svg.push_str("</svg>");
    svg
}

fn layer(accessory: &Accessory, color: &AccessoryColor, anchors: &CharacterAnchors) -> String {
    let target = match accessory.slot {
        AccessorySlot::Head => anchors.head_top,
        AccessorySlot::Eyes => anchors.eyes,
        AccessorySlot::Neck => anchors.neck,
        AccessorySlot::Companion => COMPANION_POINT,
    };
    format!(
        r#"<g class="avatar-accessory" data-accessory="{}" transform="translate({} {})" style="--acc:{};--acc-light:{}">{}</g>"#,
        accessory.id,
        number(target.x - accessory.anchor.x),
        number(target.y - accessory.anchor.y),
        color.main,
        color.light,
        accessory.content
    )
}

/// A coordinate without useless decimals (`12`, `-3.5`).
fn number(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if rounded.fract() == 0.0 {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color(id: &str) -> AccessoryColor {
        AccessoryColor {
            id: id.to_owned(),
            name: id.to_owned(),
            main: "#D7262B".to_owned(),
            light: "#F07A7D".to_owned(),
        }
    }

    fn accessory(id: &str, slot: AccessorySlot, layer: AccessoryLayer) -> Accessory {
        Accessory {
            id: id.to_owned(),
            name: id.to_owned(),
            slot,
            layer,
            anchor: Point::new(50.0, 40.0),
            content: format!(r#"<path id-free="{id}" d="M0 0h10"/>"#),
            colors: vec!["red".to_owned()],
            unlock: Unlock::Level(1),
        }
    }

    const ANCHORS: CharacterAnchors = CharacterAnchors {
        head_top: Point::new(128.0, 46.0),
        eyes: Point::new(128.0, 118.0),
        neck: Point::new(128.0, 180.0),
    };

    #[test]
    fn layers_are_placed_on_the_anchors_in_order() {
        let red = color("red");
        let cap = accessory("cap", AccessorySlot::Head, AccessoryLayer::Front);
        let glasses = accessory("glasses", AccessorySlot::Eyes, AccessoryLayer::Front);
        let cape = accessory("cape", AccessorySlot::Neck, AccessoryLayer::Back);
        let svg = compose_avatar(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><g id="body"/></svg>"#,
            &ANCHORS,
            &[(&cap, &red), (&glasses, &red), (&cape, &red)],
        );

        assert!(
            svg.starts_with(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="-12 -24 280 280""#)
        );
        assert!(svg.contains(r#"data-accessory="cap" transform="translate(78 6)""#));
        assert!(svg.contains(r#"data-accessory="glasses" transform="translate(78 78)""#));
        assert!(svg.contains("--acc:#D7262B;--acc-light:#F07A7D"));
        let position = |text: &str| svg.find(text).unwrap();
        assert!(position("cape") < position(r#"<g id="body"/>"#));
        assert!(position(r#"<g id="body"/>"#) < position("glasses"));
        assert!(position("glasses") < position(r#"data-accessory="cap""#));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn an_outfit_is_checked() {
        let catalog = AccessoryCatalog {
            accessories: vec![
                accessory("cap", AccessorySlot::Head, AccessoryLayer::Front),
                accessory("crown", AccessorySlot::Head, AccessoryLayer::Front),
            ],
            colors: vec![color("red")],
        };
        let unlocked = vec!["cap".to_owned()];
        assert!(check_outfit(&catalog, &[WornAccessory::new("cap", "red")], &unlocked).is_ok());
        assert!(check_outfit(&catalog, &[WornAccessory::new("crown", "red")], &unlocked).is_err());
        assert!(check_outfit(&catalog, &[WornAccessory::new("cap", "blue")], &unlocked).is_err());
        assert!(check_outfit(&catalog, &[WornAccessory::new("hat", "red")], &unlocked).is_err());
        let both = ["cap".to_owned(), "crown".to_owned()];
        assert!(check_outfit(
            &catalog,
            &[
                WornAccessory::new("cap", "red"),
                WornAccessory::new("crown", "red")
            ],
            &both
        )
        .is_err());
        // An accessory removed from the content is simply not drawn.
        assert_eq!(
            1,
            wearable(
                &catalog,
                &[
                    WornAccessory::new("gone", "red"),
                    WornAccessory::new("cap", "blue")
                ]
            )
            .len()
        );
    }

    #[test]
    fn the_content_of_an_svg_is_found() {
        assert_eq!(
            "<g/>",
            svg_content(r#"<svg xmlns="x" viewBox="0 0 1 1"><g/></svg>"#)
        );
        assert_eq!("<g/>", svg_content("<g/>"));
    }
}
