fn main() {
    scalar_types();
    compound_types()
}

fn scalar_types() {
    let int = 31;
    // let int: i32 = 31;
    let float = 3.1;
    // let float: f64 = 3.1;
    let boolean = true;
    // let boolean: bool = true;
    let character = '5';
    // let character: char = '5';

    println!("Scalar types");
    println!("Integer {int}");
    println!("Floating point number {float}");
    println!("Boolean {boolean}");
    println!("Character {character}");
}

fn compound_types() {
    // have fixed size

    // can have multiple types
    let tuple = (50, 4.0, 'a');
    // let tuple: (i32, f64, char) = (50, 4.0, 'a');

    // access tuple values
    let fifty = tuple.0;
    let four_point_zero = tuple.1;
    let a = tuple.2;
    // let (fifty, four_point_zero, a) = tuple;

    println!("Tuple values: {fifty}, {four_point_zero}, {a}");

    // should only have one type
    let array = [1, 2, 3, 4, 5];
    // let array: [i32; 5] = [1, 2, 3, 4, 5];

    // access array values
    let one = array[0];
    let two = array[1];
    let three =array[2];
    let four = array[3];
    let five = array[4];
    // let [one, two, three, four, five] = array;

    println!("Array values: {one}, {two}, {three}, {four}, {five}");
}
