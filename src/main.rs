struct Example {
    boolean_var: bool,
    unsigned_var: u32,
    heap_type: String,
}

fn main() {
    println!("HELLO, WORLD!");
    let example = Example {
        boolean_var: true,
        unsigned_var: 12,
        heap_type: String::from("test"),
    };

    read_example(&example);

    println!("{}", example.boolean_var);

    let example2 = Example{
        boolean_var: false,
        ..example
    };
    println!("{}", example2.unsigned_var);
    println!("{}", example.unsigned_var);
    println!("{}", example2.heap_type);
}

fn read_example(example: &Example) {
    println!("Bool: {}", example.boolean_var);
    println!("Number: {}", example.unsigned_var);
}
