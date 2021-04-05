use rand::prelude::*;

fn main() {

    let mut max_rnd = 0;
    let mut min_rnd: u8 = 255;
    let mut sum: u32 = 0;
    let ripetizioni = 1000;
    for _number in 1..ripetizioni {
        let x: u8 = random();
        sum += x as u32;
        if max_rnd < x {
            max_rnd = x
        }
        if min_rnd > x {
            min_rnd = x;
        }
    }
    
    println!("max numero tra 0 e 255: {}", max_rnd);
    println!("min numero tra 0 e 255: {}", min_rnd);
    println!("somma numero tra 0 e 255: {}", sum);
    println!("media tra 0 e 255: {}", sum/ripetizioni);
}

