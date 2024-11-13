use std::collections::HashSet;

#[derive(Debug)]
pub struct ChordTemplate {
    pub base_name: &'static str,
    pub intervals: HashSet<i32>,
}

impl ChordTemplate {
    pub fn create_candidate(&self, intervals: &[i32], root: i32) -> ChordCandidate {
        let missing_notes: Vec<i32> = self
            .intervals
            .iter()
            .filter(|&&interval| !intervals.contains(&interval))
            .cloned()
            .collect();
        let extra_notes: Vec<i32> = intervals
            .iter()
            .filter(|&&interval| !self.intervals.contains(&interval))
            .cloned()
            .collect();
        ChordCandidate::new(self, root, missing_notes, extra_notes)
    }
}

#[derive(Debug)]
pub struct ChordCandidate<'a> {
    pub chord_template: &'a ChordTemplate,
    pub root: i32,
    pub missing_notes: Vec<i32>,
    pub extra_notes: Vec<i32>,
    pub score: usize,
}

impl<'a> ChordCandidate<'a> {
    pub fn new(
        chord_template: &'a ChordTemplate,
        root: i32,
        missing_notes: Vec<i32>,
        extra_notes: Vec<i32>,
    ) -> Self {
        let score = missing_notes.len() + extra_notes.len();
        Self {
            chord_template,
            root,
            missing_notes,
            extra_notes,
            score,
        }
    }

    pub fn get_name(&self) -> String {
        format!(
            "{} {}",
            ROOT_NAMES[self.root as usize], self.chord_template.base_name
        )
    }
}

lazy_static! {
    pub static ref ROOT_NAMES: Vec<&'static str> =
        vec!["c", "c#", "d", "d#", "e", "f", "f#", "g", "g#", "a", "a#", "b",];
    // adapted from https://github.com/shriramters/chordcat/blob/main/src/chord_db.hpp
    pub static ref CHORD_DB: Vec<ChordTemplate> = vec![
        ChordTemplate {
            base_name: "major",
            intervals: HashSet::from([0, 4, 7]),
        },
        ChordTemplate {
            base_name: "7",
            intervals: HashSet::from([0, 4, 7, 10]),
        },
        ChordTemplate {
            base_name: "9",
            intervals: HashSet::from([0, 4, 7, 10, 2]),
        },
        ChordTemplate {
            base_name: "11",
            intervals: HashSet::from([0, 4, 7, 10, 2, 5]),
        },
        ChordTemplate {
            base_name: "13",
            intervals: HashSet::from([0, 4, 7, 10, 2, 5, 9]),
        },
        ChordTemplate {
            base_name: "minor",
            intervals: HashSet::from([0, 3, 7]),
        },
        ChordTemplate {
            base_name: "m7",
            intervals: HashSet::from([0, 3, 7, 10]),
        },
        ChordTemplate {
            base_name: "m9",
            intervals: HashSet::from([0, 3, 7, 10, 2]),
        },
        ChordTemplate {
            base_name: "m11",
            intervals: HashSet::from([0, 3, 7, 10, 2, 5]),
        },
        ChordTemplate {
            base_name: "m13",
            intervals: HashSet::from([0, 3, 7, 10, 2, 5, 9]),
        },
        ChordTemplate {
            base_name: "maj7",
            intervals: HashSet::from([0, 4, 7, 11]),
        },
        ChordTemplate {
            base_name: "maj9",
            intervals: HashSet::from([0, 4, 7, 11, 2]),
        },
        ChordTemplate {
            base_name: "maj11",
            intervals: HashSet::from([0, 4, 7, 11, 2, 5]),
        },
        ChordTemplate {
            base_name: "maj13",
            intervals: HashSet::from([0, 4, 7, 11, 2, 5, 9]),
        },
        ChordTemplate {
            base_name: "mMaj7",
            intervals: HashSet::from([0, 3, 7, 11]),
        },
        ChordTemplate {
            base_name: "mMaj9",
            intervals: HashSet::from([0, 3, 7, 11, 2]),
        },
        ChordTemplate {
            base_name: "mMaj11",
            intervals: HashSet::from([0, 3, 7, 11, 2, 5]),
        },
        ChordTemplate {
            base_name: "mMaj13",
            intervals: HashSet::from([0, 3, 7, 11, 2, 5, 9]),
        },
        ChordTemplate {
            base_name: "sus2",
            intervals: HashSet::from([0, 2, 7]),
        },
        ChordTemplate {
            base_name: "7sus2",
            intervals: HashSet::from([0, 2, 7, 10]),
        },
        ChordTemplate {
            base_name: "maj7sus2",
            intervals: HashSet::from([0, 2, 7, 11]),
        },
        ChordTemplate {
            base_name: "sus4",
            intervals: HashSet::from([0, 5, 7]),
        },
        ChordTemplate {
            base_name: "7sus4",
            intervals: HashSet::from([0, 5, 7, 10]),
        },
        ChordTemplate {
            base_name: "9sus4",
            intervals: HashSet::from([0, 5, 7, 10, 2]),
        },
        ChordTemplate {
            base_name: "maj7sus4",
            intervals: HashSet::from([0, 5, 7, 11]),
        },
        ChordTemplate {
            base_name: "maj9sus4",
            intervals: HashSet::from([0, 5, 7, 11, 2]),
        },
        ChordTemplate {
            base_name: "dim",
            intervals: HashSet::from([0, 3, 6]),
        },
        ChordTemplate {
            base_name: "dim7",
            intervals: HashSet::from([0, 3, 6, 9]),
        },
        ChordTemplate {
            base_name: "dim9",
            intervals: HashSet::from([0, 3, 6, 9, 2]),
        },
        ChordTemplate {
            base_name: "dim11",
            intervals: HashSet::from([0, 3, 6, 9, 2, 5]),
        },
        ChordTemplate {
            base_name: "ø",
            intervals: HashSet::from([0, 3, 6, 10]),
        },
        ChordTemplate {
            base_name: "aug",
            intervals: HashSet::from([0, 4, 8]),
        },
        ChordTemplate {
            base_name: "aug7",
            intervals: HashSet::from([0, 4, 8, 10]),
        },
        ChordTemplate {
            base_name: "aug9",
            intervals: HashSet::from([0, 4, 8, 10, 2]),
        },
        ChordTemplate {
            base_name: "aug11",
            intervals: HashSet::from([0, 4, 8, 10, 2, 5]),
        },
        ChordTemplate {
            base_name: "aug13",
            intervals: HashSet::from([0, 4, 8, 10, 2, 5, 9]),
        },
        ChordTemplate {
            base_name: "augMaj7",
            intervals: HashSet::from([0, 4, 8, 11]),
        },
        ChordTemplate {
            base_name: "augMaj9",
            intervals: HashSet::from([0, 4, 8, 11, 2]),
        },
        ChordTemplate {
            base_name: "augMaj11",
            intervals: HashSet::from([0, 4, 8, 11, 2, 5]),
        },
        ChordTemplate {
            base_name: "augMaj13",
            intervals: HashSet::from([0, 4, 8, 11, 2, 5, 9]),
        },
        ChordTemplate {
            base_name: "6",
            intervals: HashSet::from([0, 4, 7, 9]),
        },
        ChordTemplate {
            base_name: "m6",
            intervals: HashSet::from([0, 3, 7, 9]),
        },
    ];
}
