const PRIME_MOD: u64 = 65_537;
fn main() {
    let points = [(1,12), (2,10), (5,12), (10, 702)];
    let ans = lagrange_interpolate(&points, 3);
    println!("{ans}");
}


fn modulo_inverse(x: u64, mod_num: u64) -> u64{
    modulo_exponentiation(x, mod_num-2, mod_num)
}


fn lagrange_interpolate(points:&[(u64, u64)], x:u64)->u64{
    let mut result = 0;

    for point in points {
        let mut temp_res = 1;
        temp_res *= point.1;

        for point2 in points {
            if point.0 != point2.0 {
                temp_res *= (x%PRIME_MOD + PRIME_MOD - point2.0%PRIME_MOD)%PRIME_MOD;
                let intermediate = (point.0%PRIME_MOD + PRIME_MOD - point2.0%PRIME_MOD)%PRIME_MOD;
                temp_res %= PRIME_MOD;
                temp_res *= modulo_inverse(intermediate as u64, PRIME_MOD);
            }
        }
        temp_res %= PRIME_MOD;
        result += temp_res;
        result %= PRIME_MOD;
    }
    result
}


fn modulo_exponentiation(x: u64, power: u64, mod_num: u64)-> u64{
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

