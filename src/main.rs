// main.rs - How many CPUs does this system have?
// Written by quadfault
// 7/18/18

fn main() {
    println!("Logical cores:  {}", num_cpus::get());
    println!("Physical cores: {}", num_cpus::get_physical());
}
