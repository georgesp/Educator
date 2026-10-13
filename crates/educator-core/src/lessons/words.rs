//! The spelling animations of the lessons (SUB001410, SUB001413): `letterTiles` (letters or
//! graphemes to put in order to make a word) and `soundSort` (words to sort by the way a sound is
//! written). The orders and positions they show are computed here.

/// `letterTiles`: the tiles of a word (letters, or graphemes such as « eau ») shown mixed up, with
/// tiles that do not belong to it; the child clicks them in order to build the word. Shown only, the
/// tiles go to their place one after the other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetterTiles {
    pub word: String,
    /// The tiles of the word, in order (their texts make the word).
    pub tiles: Vec<String>,
    /// Tiles that do not belong to the word (« o », « au » for « bateau »).
    pub distractors: Vec<String>,
    pub interactive: bool,
}

impl LetterTiles {
    /// Every tile shown, in the mixed-up order of the screen: indexes in [`Self::tiles`] followed by
    /// the distractors (`tiles.len() + i`). The same word always gives the same order, never the
    /// right one when there are at least two tiles.
    pub fn shuffled(&self) -> Vec<usize> {
        let count = self.tiles.len() + self.distractors.len();
        let mut order: Vec<usize> = (0..count).collect();
        // A small deterministic shuffle (Fisher-Yates with a hash of the word).
        let mut state: u64 = self.word.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
        });
        for i in (1..count).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let j = (state % (i as u64 + 1)) as usize;
            order.swap(i, j);
        }

        if count >= 2 && order.iter().copied().eq(0..count) {
            order.swap(0, 1);
        }

        order
    }

    /// Text of a tile by its index in [`Self::shuffled`] numbering.
    pub fn tile(&self, index: usize) -> Option<&str> {
        self.tiles
            .get(index)
            .or_else(|| self.distractors.get(index - self.tiles.len().min(index)))
            .map(String::as_str)
    }
}

/// A word of a `soundSort`, and where the grapheme of its group stands in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedWord {
    pub word: String,
    /// Index of the group (in [`SoundSort::groups`]).
    pub group: usize,
    /// First letter of the group's grapheme in the word, and its length (to color it).
    pub start: usize,
    pub length: usize,
}

/// `soundSort`: words to put in the box of the grapheme that writes their sound (« o », « au »,
/// « eau »). Interactive: the child clicks a word, then its box; shown only, the words go to their box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundSort {
    /// The graphemes, one box each.
    pub groups: Vec<String>,
    pub words: Vec<SortedWord>,
    pub interactive: bool,
}

/// Position (in letters) of `grapheme` in `word`, the last occurrence when it appears several times
/// (« chocolat » : the last « o »); `None` when it is not in the word.
pub fn grapheme_position(word: &str, grapheme: &str) -> Option<usize> {
    let byte = word.rfind(grapheme)?;
    Some(word[..byte].chars().count())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiles(word: &str, tiles: &[&str], distractors: &[&str]) -> LetterTiles {
        LetterTiles {
            word: word.to_owned(),
            tiles: tiles.iter().map(|t| (*t).to_owned()).collect(),
            distractors: distractors.iter().map(|t| (*t).to_owned()).collect(),
            interactive: true,
        }
    }

    #[test]
    fn the_shuffled_order_is_a_stable_permutation_never_the_right_one() {
        let t = tiles("bateau", &["b", "a", "t", "eau"], &["o", "au"]);
        let order = t.shuffled();
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!((0..6).collect::<Vec<_>>(), sorted);
        assert_eq!(order, t.shuffled());
        assert_ne!((0..6).collect::<Vec<_>>(), order);
        assert_eq!(Some("eau"), t.tile(3));
        assert_eq!(Some("au"), t.tile(5));
        assert_eq!(None, t.tile(6));

        for word in ["le", "un", "et", "ou"] {
            let two = tiles(word, &[&word[..1], &word[1..]], &[]);
            assert_eq!(vec![1, 0], two.shuffled(), "{word}");
        }
    }

    #[test]
    fn the_grapheme_is_found_in_the_word() {
        assert_eq!(Some(3), grapheme_position("bateau", "eau"));
        assert_eq!(Some(1), grapheme_position("été", "t"));
        assert_eq!(Some(6), grapheme_position("chocolat", "a"));
        assert_eq!(Some(4), grapheme_position("chocolat", "o"));
        assert_eq!(None, grapheme_position("vélo", "au"));
    }
}
