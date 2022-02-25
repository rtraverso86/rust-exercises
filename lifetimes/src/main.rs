#![deny(rust_2018_idioms)]
// Theese examples are taken from Section 10.3 of the Rust Book.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Elision rules (to be applied in order):
//   1. Each input reference gets a unique lifetime.
//   2. If there's just one input reference, the output references gets
//      the same lifetime.
//   3. If the first input reference is self (a method), apply the
//      same lifetime to all output references.
impl<'a> ImportantExcerpt<'a> {
    // Because of the first elision rule, the only reference (self) in here
    // gets its lifetime implicitly. Nothing more to be done.
    fn level(&self) -> i32 {
        3
    }

    // Because of the first elision rule, self gets an 'a and announcement a
    // 'b lifetime, and finally because of the third rule the output lifetime
    // obtains the 'a lifetime.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // The elision rules ends up giving respectively 'a and 'b lifetimes
    // to the parameters, while we need new_part to have the same lifetime
    // as 'a (or greater). We must provide it explicitly this time.
    fn replace_part(&mut self, new_part: &'a str) {
        self.part = new_part;
    }

    fn other_part<'b>(&self, other: &ImportantExcerpt<'b>) -> &'b str {
        other.part
    }


    // Same as: fn longest_part(&self, other: &'a ImportantExcerpt) -> &str {
    fn longest_part(&self, other: &Self) -> &str {
        if self.part.len() >= other.part.len() {
            self.part
        } else {
            other.part
        }
    }
}

fn main() {
    let string1 = String::from("long string is long");

    // Example 1
    //   The lifetime of result is the same as the shortest lifetime between
    //   string1 and string2. It compiles and works because the result is
    //   used within the shortest lifetime, which is the inner block declaring
    //   string2.
    let mut result_first;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
        result_first = first(string1.as_str(), string2.as_str());
    }
    println!("The first string is '{}'", result_first);

    // Example 2
    //   Almost the same as before, but this time we try to use result outside
    //   of its scope as defined with the lifetimes annotations. Uncommenting
    //   either of the println!() below will result in a compile-time error.
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        result_first = first(string2.as_str(), string1.as_str());
    }
    //println!("The longest string is '{}'", result);     // compile error!
    //println!("The first string is '{}'", result_first); // compile error!

    // Example 3
    //   The ImportantExceprt struct uses a lifetime saying its instances
    //   cannot outlive instances of the reference it holds.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let mut i = ImportantExcerpt {
        part: first_sentence,
    };
    i.announce_and_return_part("announcement");
    i.announce_and_return_part("announcement 2");
    i.replace_part("This is valid");
}
