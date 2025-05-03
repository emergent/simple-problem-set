fn generate_primes(n: u64) -> Vec<u64> {
    let mut primes = vec![true; (n + 1) as usize];
    primes[0] = false; // 0 is not prime
    primes[1] = false; // 1 is not prime

    let m = n as usize;

    let mut i = 2;
    while i * i <= m {
        if primes[i] {
            let mut j = i * 2;
            while j <= m {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let result = primes
        .into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(i, _)| i as u64)
        .collect();

    result
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }
    let n: u64 = args[1].parse().expect("Please provide a valid number");
    let primes = generate_primes(n);
    println!("Primes up to {}: {:?}", n, primes);
    // Print the number of primes found
}
