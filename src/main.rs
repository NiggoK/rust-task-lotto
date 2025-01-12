use std::env;

use rand::{prelude::IteratorRandom, thread_rng};
// use rand::{thread_rng, Rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        // todo!("Implement")

        // did not work with the tests, but gives good results:
        // let mut rng = thread_rng();
        // let mut numbers: Vec<usize> = vec![];
        // let mut random_number: usize;

        // while numbers.len() <= take {
        //     random_number = rng.gen_range(1..=from);
        //     if !numbers.contains(&random_number) {
        //         numbers.push(random_number);
        //     }
        // }

        let mut rng = thread_rng();
        let numbers: Vec<usize> = (1..=from).choose_multiple(&mut rng, take).clone();

        return Self {
            take,
            from,
            numbers,
        };
    }

    fn get_numbers(&self) -> Vec<usize> {
        // todo!("Implement")
        self.numbers.clone()
        // with self instead of &self:
        // return self.numbers;
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    // Tip: Use the format macro
    let msg = format!(
        "{} of {}: {:?}",
        lotto.take,
        lotto.from,
        lotto.get_numbers()
    );
    return msg.to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // todo!("Implement CLI")
    let args_len = args.len();
    if args_len >= 3 && (args_len % 2) == 1 {
        for i in (1..args_len).step_by(2) {
            let arg1: usize = args[i].parse().expect("Could not parse number of takes");
            let arg2: usize = args[i + 1]
                .parse()
                .expect("Could not parse number of froms");
            let lotto = Lotto::new(arg1, arg2);

            print!("{} \n", format_lotto_results(&lotto));
        }
    } else {
        print!("Give even amount of numbers");
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
