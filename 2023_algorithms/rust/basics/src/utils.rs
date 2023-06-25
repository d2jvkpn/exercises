use std::any::type_name;

fn type_name_of<T: ?Sized>(_: &T) -> String {
    format!("{}", type_name::<T>())
}
