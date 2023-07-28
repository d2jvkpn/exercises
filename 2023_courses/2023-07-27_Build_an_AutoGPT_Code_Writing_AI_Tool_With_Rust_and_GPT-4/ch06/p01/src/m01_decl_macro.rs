macro_rules! mad_expr {
    ($x: expr) => {
        format!("You send an expression: {}", $x)
    };
}

macro_rules! mad_type {
    ($x: ty) => {
        match stringify!($x) {
            "i32" => "You sent an i32 type.".to_string(),
            _ => "You sent something else.".to_string(),
        }
    };
}

macro_rules! mad_vec {
    ($($x: expr), +) => {
        {
        	let mut temp = Vec::new();

        	$(
        	    temp.push($x);
        	)+

        	temp
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn t_mad_skill() {
        let value = mad_expr!(1 + 2);
        dbg!(value);

        let value = mad_type!(i32);
        dbg!(value);

        let value = vec![1, 2, 3];
        dbg!(value);

        let value = mad_vec!(1, 2, 3);
        dbg!(&value);
    }
}
