fn main() {
    let value1:u8 = 0b1100_1001;

    let result_a = value1 << 1; //0110_0100 => 146
    println!("{}", result_a);

    let result_a = value1 >> 1; //0010_0100 => 100
    println!("{}", result_a);
}
