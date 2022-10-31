mod tea;

fn main() {
    let c = tea::encrypt([10, 10], [1, 2, 3, 4]);
    println!("encrypted: {:?}", c);

    let d = tea::decrypt(c, [1, 2, 3, 4]);
    println!("decrypted: {:?}", d);
}
