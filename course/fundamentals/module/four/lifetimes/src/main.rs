#![allow(dead_code,unused_variables)]

fn borrow<'a>(i: &'a i32, j: &'a i32) -> &'a i32 { // ties passed variables to the 'a's lifetime
    i
}

struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    // let n = 12;
    // borrow(&n,&n);

    let mut cats = vec![
        Cat("some".to_string()),
        Cat("unknown".to_string()),
        Cat("cat".to_string()),
    ];

    let mut feeders = Vec::new();

    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat });
    }

    feeders.iter_mut().for_each(|f| {
        f.feed();
    });
}
