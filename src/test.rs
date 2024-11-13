#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn exact_c() {
        assert_eq!(get_chord_name(vec![60, 64, 67]), "c major");
        assert_eq!(get_chord_name(vec![60, 63, 67]), "c minor");
        assert_eq!(get_chord_name(vec![60, 64, 67, 69]), "c 6");
        assert_eq!(get_chord_name(vec![60, 64, 67, 71]), "c maj7");
    }

    #[test]
    fn exact_other() {
        assert_eq!(get_chord_name(vec![61, 65, 68]), "c# major");
    }

    #[test]
    fn c_many_notes() {
        assert_eq!(get_chord_name(vec![60, 64, 67, 72]), "c major");
        assert_eq!(
            get_chord_name(vec![60, 64, 67, 60 + 12, 64 + 12, 67 + 12]),
            "c major"
        );
    }

    #[test]
    fn c_missing_notes() {
        assert_eq!(get_chord_name(vec![60, 64, 67, 72]), "c major");
        assert_eq!(get_chord_name(vec![60, 64]), "c major");
        assert_eq!(get_chord_name(vec![60, 63]), "c minor");
    }

    #[test]
    fn c_variations() {
        assert_eq!(get_chord_name(vec![64, 67, 72]), "c major");
        assert_eq!(get_chord_name(vec![64, 67, 72, 72 + 10]), "c 7");
    }
}
