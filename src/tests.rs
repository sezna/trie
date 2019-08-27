extern crate rand;
extern crate test;
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use test::Bencher;

use super::Trie;

#[test]
fn insert_and_search() {
    let mut trie = Trie::new();
    trie.add_word_mut("test");
    trie.add_word_mut("alex");
    trie.add_word_mut("mary");
    assert!(trie.search("test"));
    assert!(trie.search("alex"));
    assert!(trie.search("mary"));
}

#[test]
fn insert_and_predict() {
    let mut trie = Trie::new();
    trie.add_word_mut("test");
    trie.add_word_mut("alex");
    trie.add_word_mut("testosterone");
    trie.add_word_mut("alexander");
    assert!(trie.predict("alex").contains(&String::from("alexander")));
    assert!(trie.predict("test").contains(&String::from("testosterone")));
}

#[test]
fn predict_case_insensitive() {
    let mut trie = Trie::new();
    trie.add_word_mut("test");
    trie.add_word_mut("alex");
    trie.add_word_mut("Testosterone");
    trie.add_word_mut("Alexander");
    let lowercase_compare = |a: char, b: char| {
        let a_iter = a.to_lowercase();
        let b_iter = b.to_lowercase();
        a_iter.zip(b_iter).all(|(a, b)| a == b)
    };
    assert!(trie
        .predict_by_key("alex", lowercase_compare)
        .contains(&String::from("Alexander")));
    assert!(trie
        .predict_by_key("test", lowercase_compare)
        .contains(&String::from("Testosterone")));
}

#[bench]
fn insert_a_lot(b: &mut Bencher) {
    let mut trie = Trie::new();
    b.iter(|| {
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        trie.add_word_mut(&rand_string);
    });
}

#[bench]
fn search_a_lot(b: &mut Bencher) {
    let mut vec_of_strings: Vec<String> = Vec::new();
    let mut trie = Trie::new();

    for _ in 0..1000 {
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        vec_of_strings.push(rand_string.clone());
        trie.add_word_mut(&rand_string)
    }

    b.iter(|| {
        trie.search(vec_of_strings.choose(&mut rand::thread_rng()).unwrap());
    })
}

#[bench]
fn predict_a_lot(b: &mut Bencher) {
    let mut vec_of_strings: Vec<String> = Vec::new();
    let mut trie = Trie::new();

    for _ in 0..1000 {
        let rand_string: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        vec_of_strings.push(rand_string.clone());
        trie.add_word_mut(&rand_string)
    }

    b.iter(|| {
        trie.predict(vec_of_strings.choose(&mut rand::thread_rng()).unwrap());
    })
}
