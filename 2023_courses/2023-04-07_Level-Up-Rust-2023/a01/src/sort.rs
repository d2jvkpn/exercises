fn sort_names<T: AsRef<str>>(names: &mut Vec<T>) {
    /*
    names.sort_by(|x, y| {
        let x1 = x.as_ref().to_lowercase();
        let y1 = y.as_ref().to_lowercase();

        x1.cmp(&y1)
    })
    */

    names.sort_by_cached_key(|x: &T| x.as_ref().to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test --lib -- t_sort_names
    #[test]
    fn t_sort_names() {
        let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
        sort_names(&mut users);
        assert_eq!(users, vec!["alison", "Amy", "Jennifer", "mike99", "Todd"]);
    }
}
