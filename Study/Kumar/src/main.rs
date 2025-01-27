const _X:i32 = 23;                                                 //defining global variables

fn main() {
    let c: char = 'a';
    let numbers: [i32;5] = [1,2,3,4,5];                             // initializing array
    let human =("Alice", 30, false);
    println!("number array: {:?}", numbers);                         // printing full array
    println!("3rd number: {}", numbers[2]);                          //specific element of array
    println!("character: {}", c);                                    //charachter 
    println!("Human tuple: {:?}", human);                            //tuple with basic DTs
    let mut stone_cold: String = String::from ("Hell,");             // muted string
    stone_cold.push_str("Yeah!");                                    // function to add/edit at start of string
    println!("Stone cold says: {}", stone_cold);
    let slice: &str = &stone_cold[5..10];                             // to slice/extract multiple continuous elemens of string
    println!("Slice value: {}", slice);
    let mix_tuple = ("Kratos", 23, [1,2,3,4]);                        //mixed tuple
    println!("Tuple: {:?}", mix_tuple);
    println!("All elements of Stone cold: {:?}", stone_cold.chars());   // fn to print all charachters of a string
    tell_height(186);                                                     // calling a function
    human_id("Vyom",25,182.2);
    println!("Global variable is {}", _X);
    let x: i32 = {                                                         //defining variable as a result of operation of other variables
        let price: i32 = 4;
        let qty: i32 = 5;
        price*qty
    };
    println!("Result is {}", x);
    let y: i32 = add(4,6);                                                 // calling a fn which returns values by variable
    println!("Result for fn add is {}", y);    
    println! ("Result for fn add is {}", add(5,8));                        // calling value returning fn directly  
    let addition: i32 = {add(4,5)};                                         // defining variable using value returning fn
println!("Value of variable addtion is {}", addition);

println! ("Your bmi is {:.2}", bmi(64.4,182.2));                           // putting limit {:.2} to print only 2 vlues after decimal

let s1 = String::from ("RUST");                                             
let len = calculate_length(&s1);                                            // passing reference instead of original value
println!("Length of '{}'is  {}", s1, len);
let inc_1:&mut i32 = &mut _X;
*inc_1 +=1;                                                                  //  inc. value 
println!("Original global var. {} is mutated to {}", _X, inc_1);  
let shadow: i32 = 5;
let shadow = shadow +2;                                                      //shadowing                                            //
println!("Value of shadow {} in main external", shadow );
{let shadow = shadow * 2;
println!("Value of shadow {} in main internal", shadow)};
let space = "     ";
let space = space.len();
println!("Value of space is  {}", space);
}

fn tell_height(height: u32){                                               // creating fn manually
    println!("The height is {} cm", height);
}
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);

}
fn add(a: i32, b: i32) -> i32{                                              // functions returning values
    a+b
}
fn bmi(weight_kg: f32, height_m: f32) -> f32{
    weight_kg/height_m*height_m
}
fn calculate_length(s: &String) -> usize{                                  // returns undefined size integer usize, len() calculates lenght of string, &String cause s to be reference
    s.len()
}