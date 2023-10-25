use std::{env, str::FromStr};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].parse::<MyEnum>() {
        Ok(v) => println!("==> Parsed value: {:?}", v),
        Err(e) => eprintln!("!!! Error parsing value: {}", e),
    }
}

#[derive(Debug)]
enum MyEnum {
    Value1,
    Value2,
    Value3,
}

impl FromStr for MyEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Value1" => Ok(MyEnum::Value1),
            "Value2" => Ok(MyEnum::Value2),
            "Value3" => Ok(MyEnum::Value3),
            _ => Err("invalid value".to_string()),
        }
    }
}
