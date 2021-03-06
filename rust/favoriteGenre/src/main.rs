use edn_rs::hmap;
use itertools::{GroupBy, Itertools};
use std::collections::{btree_map, BTreeMap, HashMap};

// Favorite Genres

// Given a map Map<String, List<String>> userMap, where the key is a username and the value is a list of user's songs. Also given a map Map<String, List<String>> genreMap, where the key is a genre and the value is a list of songs belonging to this genre. The task is to return a map Map<String, List<String>>, where the key is a username and the value is a list of the user's favorite genres. Favorite genre is a genre with the most song.

// Example :
// Input:

// userMap = {
//    "David": ["song1", "song2", "song3", "song4", "song8"],
//    "Emma":  ["song5", "song6", "song7"]
// },
// genreMap = {
//    "Rock":    ["song1", "song3"],
//    "Dubstep": ["song7"],
//    "Techno":  ["song2", "song4"],
//    "Pop":     ["song5", "song6"],
//    "Jazz":    ["song8", "song9"]
// }
// Output:
// {
//    "David": ["Rock", "Techno"],
//    "Emma":  ["Pop"]
// }
// Explanation:
// David has 2 Rock, 2 Techno and 1 Jazz song. So he has 2 favorite genres.
// Emma has 2 Pop and 1 Dubstep song. Pop is Emma's favorite genre.

fn main() {
    // println!("Hello, world!");
    let hashmap = hmap! {
        "Rock".to_string() => vec!["song1".to_string(), "song3".to_string()],
        "Dubstep".to_string() => vec!["song7".to_string()],
        "Techno".to_string() => vec!["song2".to_string(), "song4".to_string()],
        "Pop".to_string() => vec!["song5".to_string(), "song6".to_string()],
        "Jazz".to_string() => vec!["song8".to_string(), "song9".to_string()]
    };
    // println!("{:#?}", invert_map(hashmap));
    let inverted_map = invert_map(hashmap);
    let user_map = hmap! {
    "David".to_string() => vec!["song2".to_string(), "song4".to_string(), "song8".to_string(), "song1".to_string(), "song3".to_string()],
    "Emma".to_string() => vec!["song5".to_string(), "song6".to_string(), "song7".to_string()]
    };

    let favorite_genre = favorite_genres_map(user_map, inverted_map)
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .fold(HashMap::<String, usize>::new(), |mut m, x| {
                        *m.entry(x).or_default() += 1;
                        m
                    })
                    // .into_iter()
                    // .sorted_by(|a, b| a.1.cmp(&b.1))
                    // .collect::<BTreeMap<String, usize>>(),
            )
        })
        .collect::<HashMap<String, HashMap<String, usize>>>();

    let user_greatest_key = favorite_genre.clone().into_iter().fold(
        HashMap::<String, usize>::new(),
        |mut acc, (k, v)| {
            let max_key = v.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            acc.insert(k, max_key.1.to_owned());
            acc
        },
    );

    let new_map =
        favorite_genre
            .iter()
            .fold(HashMap::<String, Vec<String>>::new(), |mut acc, (k, v)| {
                let favorites = v
                    .iter()
                    .filter(|(_, v1)| v1 == &user_greatest_key.get(&k.to_string()).unwrap())
                    .map(|(k1, _)| k1.to_string())
                    .collect::<Vec<String>>();
                acc.insert(k.to_string(), favorites);
                acc
            });

    println!("{:#?}", new_map);
}

fn favorite_genres_map(
    user_map: HashMap<String, Vec<String>>,
    inverted_genre_map: HashMap<String, String>,
) -> HashMap<String, Vec<String>> {
    // iterate user map and substitute song per genre
    user_map
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.iter()
                    .filter_map(|s| inverted_genre_map.get(s))
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<HashMap<String, Vec<String>>>()
}

fn invert_map(genre_map: HashMap<String, Vec<String>>) -> HashMap<String, String> {
    genre_map
        .iter()
        .map(|(k, v)| -> Vec<(String, String)> {
            v.iter()
                .map(|x| (x.to_string(), k.to_string()))
                .collect::<Vec<(String, String)>>()
        })
        .flatten()
        .collect::<HashMap<String, String>>()
}

// function Input: Map<String, List<String>> userMap
