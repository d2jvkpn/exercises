// declarative macro

// macro_rules! name {
//   rule0;
//   rule1;
//   // ...
//   ruleN;
// }
//
// (matcher) => { expansion aka transcriber }

#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello, world!");
    };
}

// declarative macros
#[macro_export]
macro_rules! map {
    // $ [identifier] : [fragment-specifier]
    ($key:ty, $val:ty) => {{
        let data: HashMap<$key, $val> = HashMap::with_capacity(3);

        data
    }};

    // $ (...) sep rep(? + *)
    ($($key:expr => $val:expr), *) => {{
        let mut data = HashMap::with_capacity(3);

        $(data.insert($key, $val);)*

        data
    }};
}
