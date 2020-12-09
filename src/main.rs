use hex::FromHex;
// use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fs;

const TABLE_HEIGHT: usize = 256;
const ROUNDS: u8 = 2 * 8;

fn prt_b(num: u32) {
    println!("{:x}", num)
}

fn main() {
    // let mut rng = StdRng::seed_from_u64(64);
    let mut raw_key = fs::read_to_string("./key").expect("Something went wrong reading the file");
    println!("{:?}", raw_key);
    let mut key: [u64; 8] = [0; 8];
    let raw_decoded = hex::decode(raw_key).unwrap();
    for i in 0..8 {
        let mut temp_buff: u64 = 0;
        for offset in 0..8 {
            temp_buff <<= 8;
            temp_buff |= raw_decoded[i * 8 + offset] as u64;
        }
        key[i] = temp_buff;
    }
    for elem in &key {
        print!("{:x},", elem);
    }
    print!("\n");

    // let mut rng = rand::thread_rng();
    let mut table = [[0_u8; 4]; TABLE_HEIGHT];

    for index in 0..TABLE_HEIGHT {
        for cell in &mut table[index] {
            *cell = index as u8;
            // print!("{:x}-", index)
        }
        // print!("\n")
    }
    let mut table_clamped: [u32; TABLE_HEIGHT] = [0; TABLE_HEIGHT];
    for row_index in 0..TABLE_HEIGHT {
        table_clamped[row_index] = (((table[row_index][0] as u32) << 8 * 3)
            | ((table[row_index][1] as u32) << 8 * 2)
            | ((table[row_index][2] as u32) << 8 * 1)
            | ((table[row_index][3] as u32) << 8 * 0))
            .into()
    }
    // let original_block = KEY[0];
    // // prt_b(original_block);
    // let mut L = (original_block >> 32) as u32;
    let mask = 0xffffffff_u64;
    // // prt_b(mask);
    // let mut R = (original_block & mask) as u32;
    // cypher_block(&mut L, &mut R, &mixed_clamped);
    // println!("FINAL L");
    // prt_b(L);
    // println!("FINAL R");
    // prt_b(R);

    let mut encoded_seq: [u32; 8 * 4] = [0; 8 * 4];

    for key_part in 0..key.len() {
        let mut left_part = (key[key_part] >> 32) as u32;
        let mut right_part = (key[key_part] & mask) as u32;
        cypher_block(
            &mut left_part,
            &mut right_part,
            &table_clamped,
            &[0, 0, 0, 0],
        );
        // println!("FINAL L");
        // prt_b(L);
        // println!("FINAL R");
        // prt_b(R);
        encoded_seq[key_part * 2] = left_part;
        encoded_seq[key_part * 2 + 1] = right_part;
    }

    println!("{:?}", encoded_seq);
    println!("K0: {:x}", encoded_seq[0]);
    println!("K1: {:x}", encoded_seq[1]);
    println!("K2: {:x}", encoded_seq[2]);
    println!("K3: {:x}", encoded_seq[3]);
    let sub_keys: [u32; 4] = [
        encoded_seq[0],
        encoded_seq[1],
        encoded_seq[2],
        encoded_seq[3],
    ];

    let mut mixed = table.clone();
    let mut temp: u8;
    for row_index in 0..TABLE_HEIGHT {
        for column_index in 0..4 {
            // let next = rng.gen_range(row_index, TABLE_HEIGHT);
            let next = ((encoded_seq[row_index] >> (8 * (3 - column_index))) & 0xff) as usize;
            println!("{:x}", next);
            temp = mixed[row_index][column_index];
            mixed[row_index][column_index] = mixed[next][column_index];
            mixed[next][column_index] = temp;
            // print!("{:x}-", mixed[row_index][column_index]);
        }
        // print!("\n")
    }

    // for n in &mixed[0] {
    //     println!("{:x}", *n)
    // }

    // let mut mixed_clamped: [u32; TABLE_HEIGHT] = [0; TABLE_HEIGHT];
    // for row_index in 0..TABLE_HEIGHT {
    //     mixed_clamped[row_index] = (((mixed[row_index][0] as u32) << 8 * 3)
    //         | ((mixed[row_index][1] as u32) << 8 * 2)
    //         | ((mixed[row_index][2] as u32) << 8 * 1)
    //         | ((mixed[row_index][3] as u32) << 8 * 0))
    //         .into()
    // }
    // prt_b(mixed_clamped[0]);
}

fn cypher_block(l: &mut u32, r: &mut u32, s: &[u32; TABLE_HEIGHT], k: &[u32; 4]) {
    let mut temp: u32;
    *l ^= k[0];
    *r ^= k[1];

    for round in 0..ROUNDS {
        temp = *r ^ s[(*l & 0xff) as usize];
        let mut shift = 16;
        if round == 2 || round == 3 {
            shift = 8;
        } else if round == 6 || round == 7 {
            shift = 24
        }
        *r = *l << shift | *l >> (32 - shift);
        *l = temp;
    }

    *l ^= k[2];
    *r ^= k[3];
}
