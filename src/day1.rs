#[derive(Debug)]
struct Elf {
    id: usize,
    foods: Vec<u32>,
    calories: u32,
}

pub fn day1() {
    let input = include_str!("input.txt");

    let splits: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

    let mut elfs = Vec::new();

    for (id, split) in splits.iter().enumerate() {
        let foods: Vec<u32> = split
            .split('\n')
            .filter_map(|calories| calories.parse().ok())
            .collect();

        let calories = foods.iter().sum();
        elfs.push(Elf {
            id,
            foods,
            calories,
        })
    }

    elfs.sort_by(|elf1, elf2| elf2.calories.cmp(&elf1.calories));

    let first_3 = &elfs[0..3];
    let first_3_sum: u32 = first_3.iter().map(|e| e.calories).sum();

    println!("Elfs : {:#?}", first_3_sum);
}