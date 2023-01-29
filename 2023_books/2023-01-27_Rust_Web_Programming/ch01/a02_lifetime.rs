fn main() {
    //
    let one: i8 = 1;
    let outcome: &i8;
    {
        let two: i8 = 2;
        outcome = filter01(&one, &two);
    }
    println!("{}", outcome);

    //
    println!("{}", filter02(&4, &2));
}

fn filter01<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {
    if first_number < second_number {
        &0
    } else {
        first_number
    }
}

fn filter02<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {
    let ans = if first_number < second_number { second_number } else { first_number };

    ans
}
