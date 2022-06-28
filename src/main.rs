use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
struct TestCase {
    dictionary: Vec<[String; 2]>,
    queries: Vec<[String; 2]>,
}

#[derive(Debug, Deserialize)]
struct Input {
    #[serde(rename = "testCases")]
    test_cases: Vec<TestCase>,
}

fn main() {
    let mut file = File::create("output.txt").unwrap();
    let body = fs::read_to_string("./test.in.json").expect("Something went wrong reading the file");
    let input = serde_json::from_str::<Input>(&body).expect("Input JSON");
    for test_case in input.test_cases {
        let mut clusters: Vec<Rc<HashSet<Rc<String>>>> = Vec::new();
        let mut dictionary: HashMap<Rc<String>, Rc<HashSet<Rc<String>>>> = HashMap::new();
        for [w1, w2] in test_case
            .dictionary
            .into_iter()
            .map(|[w1, w2]| [Rc::new(w1.to_lowercase()), Rc::new(w2.to_lowercase())])
        {
            let mut refresh = HashSet::new();
            refresh.insert(Rc::clone(&w1));
            refresh.insert(Rc::clone(&w2));
            clusters = clusters.into_iter().fold(Vec::new(), |mut tmp, cluster| {
                if cluster.contains(&w1) || cluster.contains(&w2) {
                    refresh = refresh
                        .union(&cluster)
                        .map(|word| Rc::clone(word))
                        .collect::<HashSet<Rc<String>>>();
                } else {
                    tmp.push(cluster)
                }
                tmp
            });
            clusters.push(Rc::new(refresh));
        }
        dictionary = clusters.into_iter().fold(dictionary, |mut tmp, cluster| {
            for word in cluster.iter() {
                tmp.insert(Rc::clone(word), Rc::clone(&cluster));
            }
            tmp
        });
        for [w1, w2] in test_case
            .queries
            .into_iter()
            .map(|[w1, w2]| [Rc::new(w1.to_lowercase()), Rc::new(w2.to_lowercase())])
        {
            if w1.eq(&w2) {
                println!("synonyms");
                file.write_all(b"synonyms\n").unwrap();
                continue;
            }
            match dictionary.get(&w1) {
                None => {
                    println!("different");
                    file.write_all(b"different\n").unwrap();
                }
                Some(syms) => {
                    if syms.contains(&w2) {
                        println!("synonyms");
                        file.write_all(b"synonyms\n").unwrap();
                    } else {
                        println!("different");
                        file.write_all(b"different\n").unwrap();
                    }
                }
            };
        }
    }
}
