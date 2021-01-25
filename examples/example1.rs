use derive_stringset::StringSet;
use stringset::StringSet;
use serde_json_core;
use serde::{Deserialize};


#[derive(StringSet, Debug, Deserialize)]
struct Top {
    a: u32,
    b: u8,
    c: [u8;3],
    d: Inner,
}

#[derive(StringSet, Debug, Deserialize)]
struct Inner {
    e: u32,
}

fn main() {
    let mut t = Top {
        a: 0,
        b: 0,
        c: [0; 3],
        d: Inner {
            e: 0,
        }
    };

    let field = "a".split('/').peekable();

    dbg!(&t);
    t.string_set(field, "5").unwrap();
    dbg!(&t);


    let field = "c".split('/').peekable();
    t.string_set(field, "[1,2,3]").unwrap();
    dbg!(&t);

    let field = "d/e".split('/').peekable();
    t.string_set(field, "7").unwrap();
    dbg!(&t);
}