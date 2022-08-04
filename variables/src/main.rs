const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
   // Part 1
   let mut missiles: i32 = STARTING_MISSILES;
   let ready: i32 = READY_AMOUNT;
   println!("Firing {} of my {} missles...", ready, missiles);

   // Part 2
   missiles = missiles - ready;
   println!("{} missiles left", missiles);

   // Extra
   let _unused_variable: i32 = 40;
   let (mut missiles_e, ready_e): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
   println!("Extra: Firing {} of my {} missles...", ready_e, missiles_e);
   println!("Extra: {} missiles left", missiles_e - ready_e);
}
