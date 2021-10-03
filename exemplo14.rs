fn main(){
    let a:u16 = 0x6db7;
    let b:u16;
    let mask:u16 = 0xff00;

    b = a | mask;
    
    println!("(a) » 0x{:x} ", a);
    println!("(mask) » 0x{:x} ", mask);
    println!("b = a & mask ");
    println!("b = 0x{:x} ", b);
    
}