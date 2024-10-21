use std::sync::mpsc;
use std::thread::{spawn};




fn isPrime(num:u64) -> bool{
    for i in 2..num/2 {
        if num % i == 0{
            return false;
        }
    }
    return true;
}



fn main() {
    
    let mut starting_range = String::from("");
    println!("Please enter the starting number:\n");
    std::io::stdin().read_line(&mut starting_range).expect("input error");

    let starting_range:u64 = starting_range.trim().parse().expect("Expected a number");
    
    let mut ending_range = String::from("");
    println!("Please enter the starting number:\n");
    std::io::stdin().read_line(&mut ending_range).expect("input error");

    let ending_range:u64 = ending_range.trim().parse().expect("Expected a number");

    // 8 core system. so let's divide the task into eight threads.
    let rec = {
            let (tx, rx) = mpsc::channel();
            let total_numbers = ending_range - starting_range;
            let equal_partition = total_numbers/8; 
            for i in starting_range..ending_range + 1 {
                let tx1 = tx.clone();
                spawn(move || {
                    for j in starting_range..starting_range + equal_partition {
                        if isPrime(j) {
                            tx1.send(j).unwrap();
                        }
                    }
                });
                let tx2 = tx.clone();
                spawn(move || {
                    for j in starting_range + equal_partition + 1..starting_range + equal_partition*{
                        if isPrime(j) {
                            tx2.send(j).unwrap();
                        }
                    }
                });
                let tx3 = tx.clone();
                spawn(move || {
                    tx3.send(1).unwrap();
                });
                let tx4 = tx.clone();
                spawn(move || {
                    tx4.send(1).unwrap();
                });
                let tx5 = tx.clone();
                spawn(move || {
                    tx5.send(1).unwrap();
                });
                let tx6 = tx.clone();
                spawn(move || {
                    tx6.send(1).unwrap();
                });
                let tx7 = tx.clone();
                spawn(move || {
                    tx7.send(1).unwrap();
                });
                let tx8 = tx.clone();
                spawn(move || {
                    tx8.send(1).unwrap();
                });
            }
            rx
    };


    // 2 to 500
    // 498
    // 498/8 = 62
    // 2 + 62 => 1st thread
    // 65 + 62 => 127 => 2nd thread
    // 128 + 62 => 190 => 3rd thread
    // 191 + 62 => 253 => 4th thread
    // 254 + 62 => 316 => 5th thread
    // 317 + 62 => 379 => 6th thread
    // 380 + 62 => 442 => 7th thread
    // 443 ... 500 tak => 8th thread
    

}
