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

#[macro_export]
macro_rules! location {
    () => {{
        let caller = std::panic::Location::caller();
        format!("{}@{}", caller.file(), caller.line())
    }};
}

#[macro_export]
macro_rules! trace {
    () => {{
        fn f() {}

        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let caller = std::panic::Location::caller();
        let name = type_name_of(f);

        /*
        let list: Vec<&str> = name.split("::").collect();
        // println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" { length - 3 } else { length - 2 };

        format!("{}:{}:{}", caller.file(), caller.line(), list[idx])
        */

        // Find and cut the rest of the path
        let func = match &name[..name.len() - 3].rfind(':') {
            Some(i) => &name[i + 1..name.len() - 3],
            None => &name[..name.len() - 3], // {{closure}}
        };

        format!("{}@{}::{}", caller.file(), caller.line(), func)
    }};
}

// $ cargo test --lib -- --nocapture -- tests_mod
#[cfg(test)]
mod tests_mod {
    // biz code
    fn new_error(loc: String) -> String {
        loc // create a custom error enum in production
    }

    fn t_a() -> Result<i32, String> {
        Err(new_error(trace!()))
    }

    #[test]
    fn t_b() {
        if let Err(e) = t_a() {
            eprintln!("!!! Got an error: {}", e);
        }

        println!("~~ location: {}", location!());
    }
}
