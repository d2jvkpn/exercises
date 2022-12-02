#[macro_export]
macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let caller = std::panic::Location::caller();
        let name = type_name_of(f);
        let list: Vec<&str> = name.split("::").collect();
        // println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" { length - 3 } else { length - 2 };

        format!("{}:{}:{}", caller.file(), caller.line(), list[idx])
    }};
}

pub mod configuration;
pub mod routes;
pub mod startup;

#[cfg(test)]
mod tests {
    use crate::routes::healthz;

    #[actix_rt::test]
    async fn health_check_succeeds() {
        let response = healthz().await;
        assert!(response.status().is_success())
    }
}
