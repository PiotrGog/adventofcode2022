use std::collections::VecDeque;

#[derive(Default)]
struct Monkey {
    starting_items: VecDeque<u32>,
    operation: Option<Box<dyn Fn(u32) -> u32>>,
    test: Option<Box<dyn Fn(u32) -> bool>>,
    if_true: Option<usize>,
    if_false: Option<usize>,
}

impl Monkey {
    pub fn builder() -> MonkeyBuilder {
        MonkeyBuilder::default()
    }

    pub fn turn(&mut self) -> Vec<(u32, usize)> {
        std::iter::from_fn(|| self.inspect_first_and_throw_to()).collect()
    }

    pub fn inspect_first_and_throw_to(&mut self) -> Option<(u32, usize)> {
        if let Some(worry_level) = self.starting_items.pop_front() {
            let new_worry_level = self.operation.as_ref().unwrap()(worry_level);
            let new_worry_level = new_worry_level / 3;
            Some((
                new_worry_level,
                if self.test.as_ref().unwrap()(new_worry_level) {
                    self.if_true.unwrap()
                } else {
                    self.if_false.unwrap()
                },
            ))
        } else {
            None
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>) -> Vec<usize> {
    let mut inspect_time_result = vec![0; monkeys.len()];
    for i in 0..monkeys.len() {
        let monkey_turn_result = { monkeys[i].turn() };
        inspect_time_result[i] += monkey_turn_result.len();
        monkey_turn_result
            .into_iter()
            .for_each(|(item, monkey_num)| monkeys[monkey_num].starting_items.push_back(item))
    }
    inspect_time_result
}

#[derive(Default)]
struct MonkeyBuilder {
    monkey: Monkey,
}

impl MonkeyBuilder {
    pub fn starting_items(mut self, starting_items: &mut [u32]) -> Self {
        self.monkey.starting_items = Vec::from(starting_items).into();
        self
    }

    pub fn operation(mut self, operation: impl Fn(u32) -> u32 + 'static) -> Self {
        self.monkey.operation = Some(Box::new(operation));
        self
    }

    pub fn test(mut self, test: impl Fn(u32) -> bool + 'static) -> Self {
        self.monkey.test = Some(Box::new(test));
        self
    }

    pub fn if_true(mut self, if_true: usize) -> Self {
        self.monkey.if_true = Some(if_true);
        self
    }

    pub fn if_false(mut self, if_false: usize) -> Self {
        self.monkey.if_false = Some(if_false);
        self
    }

    pub fn build(self) -> Monkey {
        self.monkey
    }
}

fn prepare_monkeys() -> Vec<Monkey> {
    let monkey0 = Monkey::builder()
        .starting_items(&mut [57])
        .operation(|val| val * 13)
        .test(|val| val % 11 == 0)
        .if_true(3)
        .if_false(2)
        .build();
    let monkey1 = Monkey::builder()
        .starting_items(&mut [58, 93, 88, 81, 72, 73, 65])
        .operation(|val| val + 2)
        .test(|val| val % 7 == 0)
        .if_true(6)
        .if_false(7)
        .build();
    let monkey2 = Monkey::builder()
        .starting_items(&mut [65, 95])
        .operation(|val| val + 6)
        .test(|val| val % 13 == 0)
        .if_true(3)
        .if_false(5)
        .build();
    let monkey3 = Monkey::builder()
        .starting_items(&mut [58, 80, 81, 83])
        .operation(|val| val * val)
        .test(|val| val % 5 == 0)
        .if_true(4)
        .if_false(5)
        .build();
    let monkey4 = Monkey::builder()
        .starting_items(&mut [58, 89, 90, 96, 55])
        .operation(|val| val + 3)
        .test(|val| val % 3 == 0)
        .if_true(1)
        .if_false(7)
        .build();
    let monkey5 = Monkey::builder()
        .starting_items(&mut [66, 73, 87, 58, 62, 67])
        .operation(|val| val * 7)
        .test(|val| val % 17 == 0)
        .if_true(4)
        .if_false(1)
        .build();
    let monkey6 = Monkey::builder()
        .starting_items(&mut [85, 55, 89])
        .operation(|val| val + 4)
        .test(|val| val % 2 == 0)
        .if_true(2)
        .if_false(0)
        .build();
    let monkey7 = Monkey::builder()
        .starting_items(&mut [73, 80, 54, 94, 90, 52, 69, 58])
        .operation(|val| val + 7)
        .test(|val| val % 19 == 0)
        .if_true(6)
        .if_false(0)
        .build();
    vec![
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ]
}

fn find_monkey_business_level(mut monkeys: Vec<Monkey>, rounds: usize) -> usize {
    let mut inspect_time_result = vec![0; monkeys.len()];
    for _ in 0..rounds {
        let inspect_counter = round(&mut monkeys);
        inspect_counter
            .into_iter()
            .enumerate()
            .for_each(|(i, counter)| inspect_time_result[i] += counter)
    }
    inspect_time_result.sort();
    inspect_time_result.reverse();
    inspect_time_result[0] * inspect_time_result[1]
}

fn solve_part_1(monkeys: Vec<Monkey>) -> usize {
    find_monkey_business_level(monkeys, 20)
}

fn part_1(monkeys: Vec<Monkey>) {
    let result = solve_part_1(monkeys);
    println!("Part 1 result: {:?}", result);
}

fn main() {
    part_1(prepare_monkeys());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_monkeys() -> Vec<Monkey> {
        let monkey0 = Monkey::builder()
            .starting_items(&mut [79, 98])
            .operation(|val| val * 19)
            .test(|val| val % 23 == 0)
            .if_true(2)
            .if_false(3)
            .build();
        let monkey1 = Monkey::builder()
            .starting_items(&mut [54, 65, 75, 74])
            .operation(|val| val + 6)
            .test(|val| val % 19 == 0)
            .if_true(2)
            .if_false(0)
            .build();
        let monkey2 = Monkey::builder()
            .starting_items(&mut [79, 60, 97])
            .operation(|val| val.pow(2))
            .test(|val| val % 13 == 0)
            .if_true(1)
            .if_false(3)
            .build();
        let monkey3 = Monkey::builder()
            .starting_items(&mut [74])
            .operation(|val| val + 3)
            .test(|val| val % 17 == 0)
            .if_true(0)
            .if_false(1)
            .build();
        vec![monkey0, monkey1, monkey2, monkey3]
    }

    #[test]
    fn test_part_1() {
        let monkeys = prepare_monkeys();
        let result = solve_part_1(monkeys);
        assert_eq!(result, 10605);
    }
}
