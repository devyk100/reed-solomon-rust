const PRIME_MOD: u32 = 65_537;
fn main() {
    let points = [(1,12), (2,10), (3,2), (4, 3)];
    lagrange_interpolate(&points);
    let ans = modulo_inverse(199, PRIME_MOD);
    println!("{ans}");
}


fn modulo_inverse(x: u16, mod_num: u32) -> u64{
    modulo_exponentiation(x, mod_num-2, mod_num)
}


fn lagrange_interpolate(points:&[(i32, i32)]){
    let a = points[0].1;
    println!("{a}");

}


fn modulo_exponentiation(x: u16, power: u32, mod_num: u32)-> u64{
    let mod_num:u64 = mod_num.into();
    let mut exp = power;
    let mut result: u64 = 1;
    let mut base: u64 = x.into();
    while exp > 0 {
        if exp % 2 == 1{
            result = (result * base)%mod_num;
        }
        base %= mod_num as u64;
        base = (base * base)%mod_num;
        exp /= 2;
    }
    return result.into();
}

