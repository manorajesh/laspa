/// Used by laspa to print to the console.
#[no_mangle]
pub extern "C" fn print_f64(value: f64) {
    println!("{}", value);
}