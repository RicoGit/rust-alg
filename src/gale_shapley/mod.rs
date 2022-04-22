//! Solution for stable marriage problem.
//! [Gale-shapley algorithm](https://en.wikipedia.org/wiki/Gale%E2%80%93Shapley_algorithm)

use std::collections::HashMap;

fn find_pairs<'a>(
    men: HashMap<&'a str, Vec<&'a str>>,
    women: HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, &'a str> {
    // key is woman name
    let mut pairs = HashMap::<&'a str, &'a str>::new();

    loop {
        for (man, pref) in &men {
            for woman_candidate in pref {
                match pairs.get(woman_candidate) {
                    None => {
                        pairs.insert(woman_candidate, man);
                        break;
                    }
                    Some(rival) => {
                        let w_pref = women.get(woman_candidate).expect("unknown woman");
                        if new_is_better(w_pref, man, rival) {
                            pairs.insert(woman_candidate, man);
                            break;
                        } else {
                            continue;
                        }
                    }
                }
            }
        }

        if pairs.len() == women.len() {
            break;
        }
    }

    pairs
}

fn new_is_better(preferences: &[&str], current_candidate: &str, candidate2: &str) -> bool {
    for next in preferences {
        if *next == current_candidate {
            return true;
        } else if *next == candidate2 {
            return false;
        }
    }
    panic!(
        "Candidates {} {} are not in a list, {:?}",
        current_candidate, candidate2, preferences
    )
}

#[cfg(test)]
mod test {
    use crate::gale_shapley::find_pairs;
    use std::collections::HashMap;

    #[test]
    fn test1() {
        let men = HashMap::from([
            ("Bob", vec!["Anna", "Inna", "Emma"]),
            ("Rob", vec!["Inna", "Anna", "Emma"]),
            ("Tob", vec!["Inna", "Emma", "Anna"]),
        ]);

        let women = HashMap::from([
            ("Anna", vec!["Bob", "Rob", "Tob"]),
            ("Inna", vec!["Tob", "Rob", "Bob"]),
            ("Emma", vec!["Rob", "Bob", "Tob"]),
        ]);

        let res = find_pairs(men, women);

        assert_eq!(
            res,
            HashMap::from([("Anna", "Bob"), ("Emma", "Rob"), ("Inna", "Tob")])
        )
    }

    #[test]
    fn test2() {
        let men = HashMap::from([
            ("Bob", vec!["Anna", "Inna", "Emma"]),
            ("Rob", vec!["Anna", "Inna", "Emma"]),
            ("Tob", vec!["Anna", "Inna", "Emma"]),
        ]);

        let women = HashMap::from([
            ("Anna", vec!["Tob", "Rob", "Bob"]),
            ("Inna", vec!["Tob", "Rob", "Bob"]),
            ("Emma", vec!["Tob", "Rob", "Bob"]),
        ]);

        let res = find_pairs(men, women);

        assert_eq!(
            res,
            HashMap::from([("Anna", "Tob"), ("Inna", "Rob"), ("Emma", "Bob")])
        )
    }
}
