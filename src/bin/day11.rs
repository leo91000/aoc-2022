use std::{cell::RefCell, str::FromStr};

use anyhow::anyhow;
use num_bigint::BigInt;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn execute(&self, left: &BigInt, right: &BigInt) -> BigInt {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operator = match s {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => return Err(anyhow!("Unrecognized operator {s}")),
        };
        Ok(operator)
    }
}

#[derive(Debug)]
enum Value {
    Old,
    Number(BigInt),
}

impl Value {
    fn get(&self, old: &BigInt) -> BigInt {
        if let Value::Number(n) = self {
            return n.clone();
        }
        old.clone()
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            return Ok(Value::Old);
        }

        let number = s
            .parse()
            .map_err(|e| anyhow!("Invalid BigInt number value {s} : {e:?}"))?;
        Ok(Value::Number(number))
    }
}

#[derive(Debug)]
struct Operation {
    left: Value,
    operator: Operator,
    right: Value,
}

impl Operation {
    fn execute(&self, old: &BigInt) -> BigInt {
        self.operator
            .execute(&self.left.get(old), &self.right.get(old))
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        if parts.len() != 6 {
            return Err(anyhow!(
                "Expected 6 space separator in '{s}' but found {}",
                parts.len()
            ));
        }

        if parts[0] != "Operation:" {
            return Err(anyhow!(
                "Expected first part of string to be 'Operation:' but found {}",
                parts[0]
            ));
        }

        if parts[1] != "new" {
            return Err(anyhow!(
                "Expected 'new' assignement but found '{}'",
                parts[1]
            ));
        }

        if parts[2] != "=" {
            return Err(anyhow!(
                "Expected asignement operation '=' but found '{}'",
                parts[2]
            ));
        }

        Ok(Operation {
            left: parts[3].parse()?,
            operator: parts[4].parse()?,
            right: parts[5].parse()?,
        })
    }
}

#[derive(Debug)]
enum TestResult {
    True { monkey_id: BigInt },
    False { monkey_id: BigInt },
}

impl TestResult {
    fn monkey_id(&self, value: bool) -> Option<BigInt> {
        let monkey_id = match (self, value) {
            (Self::True { monkey_id }, true) => monkey_id,
            (Self::False { monkey_id }, false) => monkey_id,
            _ => return None,
        };

        Some(monkey_id.clone())
    }
}

impl FromStr for TestResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        if parts.len() != 6 {
            return Err(anyhow!(
                "Expected 6 space separator in '{s}' but found {}",
                parts.len()
            ));
        }

        if parts[0] != "If" {
            return Err(anyhow!(
                "Expected 'If' as first parts but found {}",
                parts[0]
            ));
        }

        if parts[2] != "throw" || parts[3] != "to" || parts[4] != "monkey" {
            return Err(anyhow!(
                "Expected 'throw to monkey' keywords but found {} {} {}",
                parts[2],
                parts[3],
                parts[4],
            ));
        }

        let monkey_id = parts[5].parse()?;
        let result = match parts[1] {
            "true:" => TestResult::True { monkey_id },
            "false:" => TestResult::False { monkey_id },
            _ => return Err(anyhow!("Expected true: or false: but found '{}'", parts[1])),
        };
        Ok(result)
    }
}

#[derive(Debug)]
enum Test {
    Divisible(BigInt),
}

impl Test {
    fn test(&self, value: BigInt) -> bool {
        match self {
            Test::Divisible(v) => value % v == BigInt::from(0),
        }
    }
}

impl FromStr for Test {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(' ').collect::<Vec<_>>();

        if parts.len() != 4 {
            return Err(anyhow!(
                "Expected 3 space separator in '{s}' but found {}",
                parts.len()
            ));
        }

        if parts[0] != "Test:" {
            return Err(anyhow!(
                "Expected 'Test:' as first parts but found {}",
                parts[0]
            ));
        }

        let amount = parts[3].parse()?;
        let result = match (parts[1], parts[2]) {
            ("divisible", "by") => Test::Divisible(amount),
            _ => {
                return Err(anyhow!(
                    "Expected string to start with divisible by but found {} {}",
                    parts[1],
                    parts[2]
                ))
            }
        };
        Ok(result)
    }
}

#[derive(Debug)]
struct MonkeyTest {
    test: Test,
    result: (TestResult, TestResult),
}

impl MonkeyTest {
    fn apply_test(&self, value: &BigInt) -> BigInt {
        let test_result = self.test.test(value.clone());
        self.result.0.monkey_id(test_result).unwrap_or_else(|| {
            self.result
                .1
                .monkey_id(test_result)
                .expect("duplicate conditions")
        })
    }
}

impl FromStr for MonkeyTest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        if lines.len() != 3 {
            return Err(anyhow!(
                "Expected 3 lines in '{s}' but found {}",
                lines.len()
            ));
        }

        Ok(MonkeyTest {
            test: lines[0].trim().parse()?,
            result: (lines[1].trim().parse()?, lines[2].trim().parse()?),
        })
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<BigInt>,
    operation: Operation,
    test: MonkeyTest,
    nb_inspected: BigInt,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        if lines.len() != 6 {
            return Err(anyhow!(
                "Expected 6 lines in '{s}' but found {}",
                lines.len()
            ));
        }

        let (_, items) = lines[1].split_once(": ").ok_or(anyhow!(
            "Expected ': ' separator in second line but found '{}'",
            lines[1]
        ))?;
        let items = items
            .split(", ")
            .map(|i| i.parse())
            .collect::<Result<Vec<BigInt>, _>>()?;

        Ok(Monkey {
            items,
            operation: lines[2].trim().parse()?,
            test: lines[3..6].join("\n").parse()?,
            nb_inspected: BigInt::from(0),
        })
    }
}

impl Monkey {
    fn execute_procedure(&mut self, with_relief: bool) -> anyhow::Result<(BigInt, BigInt)> {
        let mut item = self
            .items
            .pop()
            .ok_or(anyhow!("Cannot apply procedure to monkey, no items left"))?;

        self.nb_inspected += 1;

        item = self.operation.execute(&item);
        if with_relief {
            item /= 3;
        }
        let new_monkey = self.test.apply_test(&item);

        Ok((new_monkey, item))
    }
}

#[derive(Debug)]
struct Monkeys(Vec<RefCell<Monkey>>);

impl FromStr for Monkeys {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys: Vec<RefCell<Monkey>> = s
            .split("\n\n")
            .map(|m| m.parse().map(RefCell::new))
            .collect::<Result<_, _>>()?;

        Ok(Monkeys(monkeys))
    }
}

impl Monkeys {
    fn execute_round(&self, with_relief: bool) {
        for monkey in &self.0 {
            while !monkey.borrow().items.is_empty() {
                let (monkey_id, item) = monkey.borrow_mut().execute_procedure(with_relief).unwrap();

                let index: usize = monkey_id.try_into().unwrap();
                self.0[index].borrow_mut().items.push(item);
            }
        }
    }

    fn calculate_monkey_business(&self, round: u32, with_relief: bool) -> BigInt {
        for _ in 0..round {
            self.execute_round(with_relief);
        }

        let mut monkeys_inspections: Vec<BigInt> = self
            .0
            .iter()
            .map(|v| v.borrow().nb_inspected.clone())
            .collect();
        monkeys_inspections.sort_by(|a, b| b.cmp(a));

        monkeys_inspections[0].clone() * monkeys_inspections[1].clone()
    }
}

fn part1(monkeys: &Monkeys) -> BigInt {
    monkeys.calculate_monkey_business(20, true)
}

fn part2(monkeys: &Monkeys) -> BigInt {
    monkeys.calculate_monkey_business(10000, false)
}

fn main() {
    let input = include_str!("day11_example.txt");

    let monkeys: Monkeys = input.parse().expect("Failed to parse monkeys");

    println!("part 1 : {}", part1(&monkeys));
    println!("part 2 : {}", part2(&monkeys));
}
