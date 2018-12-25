use aoc::get_arg;

fn parse_digits(number: &str) -> Vec<u8> {
    number
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

fn extend_recipes(recipes: &mut Vec<u8>, new_recipes: u8) -> bool {
    if new_recipes > 9 {
        // Since we are summing to one digit numbers we can never go above
        // 18.
        recipes.push(1);
        recipes.push(new_recipes % 10);
        true
    } else {
        recipes.push(new_recipes);
        false
    }
}

fn part_a(recipe_index: usize) -> String {
    let mut elf_a = 0;
    let mut elf_b = 1;
    let mut recipes: Vec<u8> = Vec::with_capacity(recipe_index + 10);
    recipes.push(3);
    recipes.push(7);

    while recipes.len() < recipe_index + 10 {
        let new_recipes = recipes[elf_a] + recipes[elf_b];
        extend_recipes(&mut recipes, new_recipes);

        // Find new recipes
        elf_a = (elf_a + recipes[elf_a] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[elf_b] as usize + 1) % recipes.len();
    }
    recipes[recipe_index..recipe_index + 10]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
}

fn part_b(sequence: &str) -> usize {
    let mut elf_a = 0;
    let mut elf_b = 1;
    let mut recipes: Vec<u8> = vec![3, 7];
    let sequence = parse_digits(sequence);

    loop {
        let new_recipes = recipes[elf_a] + recipes[elf_b];
        let added_two = extend_recipes(&mut recipes, new_recipes);

        // Find new recipes
        elf_a = (elf_a + recipes[elf_a] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[elf_b] as usize + 1) % recipes.len();

        if recipes.len() < sequence.len() {
            continue;
        }

        let seq_start = recipes.len() - sequence.len();
        if recipes[seq_start..] == sequence[..] {
            return seq_start;
        } else if added_two
            && seq_start >= 1
            && recipes[seq_start - 1..recipes.len() - 1] == sequence[..]
        {
            return seq_start - 1;
        }
    }
}

fn main() {
    let arg: String = get_arg().unwrap();

    println!("Answer A: {}", part_a(arg.parse::<usize>().unwrap()));
    println!("Answer B: {}", part_b(&arg));
}

#[test]
fn test_parse_digits() {
    assert_eq!(
        parse_digits("1234567890"),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
    );
}

#[test]
fn test_a() {
    assert_eq!(part_a(5), "0124515891");
    assert_eq!(part_a(18), "9251071085");
    assert_eq!(part_a(2018), "5941429882");
}

#[test]
fn test_b() {
    assert_eq!(part_b("01245"), 5);
    assert_eq!(part_b("51589"), 9);
    assert_eq!(part_b("92510"), 18);
    assert_eq!(part_b("59414"), 2018);
}
