mod random_info;
use random_info::*;

#[allow(non_snake_case)] 
#[allow(unused_variables)]
 
fn main() {
    println!("Hello World");
    data_types_follow_along();
}

fn data_types_follow_along() {
    let some_data: bool = true; //or false
    let implied_data = false; //Rust assumes you want a boolean
    //rust assumes you want immutable variables
    let mut mutable_data = true; //this is the flag for mutable data

    //integers
    let an_integer: i8 = 100; //2^8, 256 whole numbers -128 to +127 (0 included)

    println!("min i8 is {}", std::i8::MIN);
    println!("max i8 is {}", std::i8::MAX);

    let overflow_test = an_integer + 120; //220 greater than datatype allows
    println!("{}", overflow_test); //this should PANIC, which is a kind of crash

    let a_pos_integer: u8 = 10; //from 0 to 255 //u stand for unsigned
    //used quite often, for example for color values

    let big_data: i128 = 10;
    //i32 is in the +- 2billion range, i64 is really big 
    //i32 is the default

    let some_isize: isize = 10; //depends on if the computer is 32 or 64 bit
    let some_usize: usize = 10;

    //float
    //only 2 kinds, not a perfect representation
    //can only keep so much precision
    //communicate the compiler you need a .
    let a_float: f32 = 10.; 
    let another_float: f64 = 10.;
    //assumes f64 by default

    //char
    let some_char: char = 'a';
    //is 4 bytes, but holds much more than ascii
    //handles chinese, emoji, etc.
    //not a string, don't confuse them.

    //datetime is not a primitive
    //decimal is not a primitive
    //need a package for both, import crate
}

fn strings() {
    //in rust strings are not easy
    //gain memory tradeoff of not hiding complexity of strings
    //simple tings are harder, but harder things are easier down the road

    //2 kinds of strings
    let example_slice: &str = "Howdy"; //mostly immutable
    let example_string: String = String::from("Partner");

    //both are groups o char
    //stored in memory diferenty
    //String is on the heap and mutable
    //&str is immutable, can be kept on stack, heap, or embedded
    //can easily transltate between them
    //str slice is great for runtime speed

    let string_from_str: String = example_string.to_string();
    let string_from_str2: String = "Some_hardcoded_string".to_string();
    //hardcoded strings are string literals, string slices that are held in the program's binary - thus they are still &str

    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from(example_slice);
    //notice that here we didn't have to explicitly define the variable type
    //when it's obvious to the compiler what the type should be it will fill it in for your

    let str_from_string: &str = &example_string;
    //this is a deref symbol
    //creating a slice from a string it just points to the original memory of the string (borrowing)
    //borrowing is faster than copying.

    //let text = "first" + "second"; //this will fail
    let combine_string_literals = ["first", "second"].concat(); //bumps it to a String
    let combine_with_macro = format!("{} {}", "first", "second"); //also givesa String

    let string_plus_str = example_string + example_slice; //only works with the string first
    // will make sense once you know borrowing
    let mut mut_string = String::new();

    mut_string.push_str(&example_string);
    mut_string.push_str("some hardcoaded literal");
    mut_string.push('m');
    //push is meant for char, push_str is or string slices, char is not a single character string
    //char has a lot more info than a string

    let a = String::from("a");
    let b = String::from("b");
    let bombined = a + &b + &mut_string;

    let string_from_substring: &str = &example_string[..2];
    //this works on String and &str
    let string_from_substring: &str = &example_string[..=2];
    //this includes the char at index 2
    //let str_slice_panic: &str = &example_str[..200];
    //you can compile this but it will crash if there is no letter there

    let char_by_index = &example_string.chars().nth(1); //can't retrieve a char from a slice of a single letter
    //nth brings back an "option" a safe type that requries a handling
    //handling at compile time prevents crashing
    //same on both String and &str

    match char_by_index {
        Some(c) => println!("found a char {}", c),
        None => {} //empty block
    }

    if let Some(c) = example_string.chars().nth(2) {
        println!("found a char {}", c);
    }

}

fn procedures_and_functions() {
    //functions return a value
    //procedures do not
    fn some_function(param_a: f32, param_b: i128, _param_c: i8) ->  f32 {
        //-> shows the return variable
        // params that start with _ are ignored

        // to return just put a value without a semicolon
        // here are several ways to explicitly declare the type you are returning
        // 10f32
        // 10 as f32 //as is also used to cast variables
        // 10.
        // 10.0
        // 10_f32
        //these are all the same
        //can also be a varaible
        
        if param_a < 100. {
            let return_val = 10.1 * param_a + param_b as f32; //you have to be careful, an i128 can hold much biger data than a 32
        
            return_val 
        } else {
            -1.
        }
    }
}

fn some_procedure (param_a: i128, param_b: i8) {
    println!("I'm a procedure with a {} and b {}", param_a, param_b);
    //your main function is always a procedure
}

//passing strings to procedures
fn conditionals() {
    let some_bool = true;
    let some_int = 50;

    if some_bool == true {
        println!("Hit If branch");
    }

    if some_bool || some_int < 100 { //or

    }

    if !some_bool && some_int < 100 { //and

    } else if some_int < 100 {
        println!("Hit Else branch");
    } else {

    }

    let var_from_inline = if some_int == 9 { 300 } else if some_int == 10 { 100 } else { 400 };
    let var_from_inline2 = if some_int == 9 {
        300 
    } else if some_int == 10 {
        println!("can put other lines here");
        100 
    } else { 
        400 
    };

    match some_int { //match checks that all cases are covered?
        0 => println!("hit 0 branch"),
        1..=100 => {
            println!("Between 1 and 100 branch");
            println!("more stuf");
        },
        _ => println!("Else branch"),
    }

    let var_from_match = match some_bool {true => 10, false => 20};
    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 => 100,
        _ => 200,
    };
}

fn tuples() {
    let some_tuple = (2, 3.4, "a", "b".to_string(), 'c', (1.1, 2.2));
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("My full tuple is {:?}", some_tuple);

    //nested tuple, compiler is a bit weird need a random space
    let some_val = (some_tuple.5).1;

    fn get_some_rgb() -> (u8, u8, u8) {
        //some logic...
        (200, 10, 2)
    }

    let some_color = get_some_rgb();
    println!("Green is {}", some_color.1);

    let(my_red, my_green, my_blue) = some_color;
    println!(r {} g {}, b {}, my_red, my_green, my_blue);

    //unit tuple
    let empty_tuple = ();
    //used in match statements for doing nothing at the end of a branch
    //procedures are actually functions that return an empty tuple
}

fn structs() {
    // structs are complex data types like objects but different. Object oriented - ish
    // no inheritance
    //yes mthods
    // traits - similar to polymorphism
    struct KylesData {
        some_bool: bool,
        some_float: f64,
        some_int: i32,
        // rust embraces composition over inheritence
        //instead of extending you include
        random: RandomInfo,
    }

    let mut kyles_var = KylesData {
        some_bool: true,
        some_int: 80,
        some_float: 10.3,
        random: RandomInfo {
            some_int: 8,
            some_float: 8.,
            call_count: 0,
        },
    };

    kyles_var.some_int = 100;

    let dougs_var_2 = KylesData {
        some_int: 200, //manually set things before spreading any inherited values
        ..kyles_var
    };

    let mut random_info_var = RandomInfo {
        some_int: 4,
        some_float: 3.,
        call_count: 0,
    };

    //in rust you create methods outside of objects

    let mut kyles_var = KylesData {
        some_bool: true,
        some_int: 80,
        some_float: 10.3,
        random: RandomInfo::new(3), //2 colons, class method
    };

    let is_this_smaller = random_info_var.is_smaller(9); //dot operator if using self, instance method, use . 
}