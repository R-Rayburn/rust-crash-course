// Silences warnings for unused variables
#![allow(unused_variables)]

fn main() {
   let x = do_stuff(4.0, 12.5);


   // Exercise
   let width = 4;
   let height = 7;
   let depth = 10;

   // fixed scope error
   // {
	let area = area_of(width, height);
   // }
   println!("Area is {}", area);

   println!("Volume is {}", volume(width, height, depth));   
}


fn volume(x: i32, y: i32, z: i32) -> i32 {
   x * y * z
}


fn area_of(x: i32, y: i32) -> i32 {
   // Fixed incorrect return value
   // return 0;

   // Fixed warning of not using idiomatic expression
   // return x * y;

   x * y
}


fn do_stuff(qty: f64, oz: f64) -> f64 {
   println!("{} {}-oz sarsaparilla(s)!", qty, oz);
   qty * oz  // tail expression
   // return qty * oz;
}