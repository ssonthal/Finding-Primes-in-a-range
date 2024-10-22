use std::sync::mpsc;
use std::thread::spawn;




fn is_prime(num:u64) -> bool {
    for i in 2..num/2 + 1 {
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
    // let rec = {
            let (tx, rx) = mpsc::channel();
            let total_numbers = ending_range - starting_range;
            let equal_partition = total_numbers/8; 

            let tx1 = tx.clone();
            spawn(move || {
                for j in starting_range..starting_range + equal_partition {
                    if is_prime(j) {
                        tx1.send(j).unwrap();
                    }
                }
            });
            let tx2 = tx.clone();
            spawn(move || {
                let x = starting_range + equal_partition + 1;
                for j in x..x + equal_partition {
                    if is_prime(j) {
                        tx2.send(j).unwrap();
                    }
                }
            });
            let tx3 = tx.clone();
            spawn(move || {
                let x = starting_range + 2*equal_partition + 1;
                for j in x..x + equal_partition {
                    if is_prime(j) {
                        tx3.send(j).unwrap();
                    }
                }
            });
            let tx4 = tx.clone();
            spawn(move || {
                let x = starting_range + 3*equal_partition + 1;
                for j in x..x + equal_partition {
                    if is_prime(j) {
                        tx4.send(j).unwrap();
                    }
                }
            });
            let tx5 = tx.clone();
            spawn(move || {
                let x = starting_range + 4*equal_partition + 1;
                for j in x..x + equal_partition {
                    if is_prime(j) {
                        tx5.send(j).unwrap();
                    }
                }
            });
            let tx6 = tx.clone();
            spawn(move || {
                let x = starting_range + 5*equal_partition + 1;
                for j in x..x + equal_partition {
                    if is_prime(j) {
                        tx6.send(j).unwrap();
                    }
                }
            });
            let tx7 = tx.clone();
            spawn(move || {
                let x = starting_range + 6*equal_partition + 1;
                for j in x..x + equal_partition {
                    if is_prime(j) {
                        tx7.send(j).unwrap();
                    }
                }
            });
            let tx8 = tx.clone();
            spawn(move || {
                let x = starting_range + 7*equal_partition + 1;
                for j in x..ending_range {
                    if is_prime(j) {
                        tx8.send(j).unwrap();
                    }
                }
            });
        // };
        // all senders should go out of scope before the receiver. they can't go out of scope together. So it's important to drop tx from the main function ends. 
        drop(tx);
        let mut ans = Vec::new();
        for res in rx {
            ans.push(res);
        }
        ans.sort();
        println!("{:?}", ans);
    
    }


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
