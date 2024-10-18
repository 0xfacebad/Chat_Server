// let  say Alice lagre Prime number is a = 100000007
// then the non zero vlaue will be  p = 3453


// Secret Key A congr to g^a (mod m)
//    Set A=1A=1.

// Set g=gmod  pg=gmodp (to ensure gg is within the modulus).

// While a>0a>0:

//     If aa is odd, set A=(A⋅g)mod  pA=(A⋅g)modp.
//     Set g=(g⋅g)mod  pg=(g⋅g)modp (square the base).
//     Divide aa by 2 (using integer division).

// When aa reaches 0, AA will be the result.

// lets say Bob large Prime number is b = 10000000007
// then the non zero vlaue will be  p = 3453


// Secret Key  B congr to g^b (mod m)

// Second Step =>
// Send the secret key to with each other or exchange 

//Third Step =>
// Alice computes A` congr to B^a (mod p)

// Bob computes B` congr to A^b (mod p)


// fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
//     let mut result = BigUint::one();
//     let reduced_base = base % modulus; // base mod modulus
//     let mut exp = exp.clone();
//     let mut current_base = reduced_base;

//     while exp > BigUint::zero() {
//         if &exp % 2u32 == BigUint::one() {
//             result = (result * &current_base) % modulus; // If exp is odd, multiply base with result
//         }
//         current_base = (current_base.clone() * &current_base) % modulus; // Square the base
//         exp >>= 1; // Divide exp by 2
//     }

//     result
// }


// use num_bigint::BigUint;
// use num_traits::{One, Zero};
// use std::thread;
// use std::sync::{Arc ,Mutex};
// #[allow(unused_variables)]
// #[allow(dead_code)]
// fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
//     let mut result = BigUint::one();
//     let reduced_base = base % modulus; // base mod modulus
//     let mut exp = exp.clone();
//     let mut current_base = reduced_base;

//     while exp > BigUint::zero() {
//         if &exp % 2u32 == BigUint::one() {
//             result = (result * &current_base) % modulus; // If exp is odd, multiply base with result
//         }
//         current_base = (current_base.clone() * &current_base) % modulus; // Square the base
//         exp >>= 1; // Divide exp by 2
//     }

//     result
// }


// fn main() {
   
   



    // let g = BigUint::from(10000000007u128); // Base
    // let p = BigUint::from(1000000000000009u128); // Modulus

//     // let a = BigUint::from(1000023007u128); // Alice's private key
//     // let b = BigUint::from(10000009u128); // Bob's private key

//     // // Calculate public keys
//     // let alice_pub = mod_exp(&g, &a, &p); // A = g^a mod p
//     // let bob_pub = mod_exp(&g, &b, &p); // B = g^b mod p

//     // // Calculate shared secrets
//     // let alice_shared = mod_exp(&bob_pub, &a, &p); // K_A = B^a mod p
//     // let bob_shared = mod_exp(&alice_pub, &b, &p); // K_B = A^b mod p

//     // // Verify that both shared secrets are equal
//     // assert_eq!(alice_shared, bob_shared);
//     // println!("Shared secrets match!");


//     //  let three = 0b11; 
//     //  let thirty = 0o36; 
//     // let three_hundred = 0x12C; 
//     // println!("base 10: {} {} {}", three, thirty, three_hundred);
//     // println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
//     // println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
//     // println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
// }
