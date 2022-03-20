use rand::{Rng};

const IN_A_ROW: i32 = 20;


fn main() {
    let mut heads = 0;
    let mut tails = 0;
    let mut iters = 0;

    loop {
        let coin = toss_coin();
        if coin {
            heads += 1;
            tails = 0;
        }else {
            tails += 1;
            heads = 0;
        }
        iters += 1;
        print!("\r{iters}");
        // print!("\n\r coin: {coin}");
        if heads >= IN_A_ROW {
            println!("\nHEADS!: {heads}");
            break;
        }
        else if tails >= IN_A_ROW {
            println!("\nTAILS!: {tails}");
            break;
        }
    }
}

fn toss_coin() -> bool {
    let rng = rand::thread_rng().gen_range(0 ..=1);

    match rng {
        1 => {
            //Heads
            true
        },
        2 => {
            //Tails
            false
        },
        _ => {false}
    }
}