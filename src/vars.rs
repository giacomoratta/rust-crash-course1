pub fn run() {
    println!("Hello from printer");
    println!("{} is from {}", "Brad", "Mass");
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");
    println!("{name} likes to play {activity}", name="John", activity="Baseball");
    println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "hello"));
}