// insipred by https://blog.s20n.dev/posts/how-chordcat-works/

use itertools::Itertools;

#[macro_use]
extern crate lazy_static;

mod db;
mod test;

fn chord_to_intervals(chord: &[i32], root: i32) -> Vec<i32> {
    chord.iter().map(|&x| (x - root) % 12).collect()
}

pub fn get_chord_name<I>(chord: I) -> String
where
    I: IntoIterator<Item = i32>,
{
    let chord: Vec<i32> = chord.into_iter().collect();

    (0..12)
        .flat_map(|root| {
            let intervals = chord_to_intervals(&chord, root);
            db::CHORD_DB
                .iter()
                .map(move |template| template.create_candidate(&intervals, root))
        })
        .sorted_by_key(|c| c.score)
        .take(1)
        .next()
        .unwrap()
        .get_name()
}
