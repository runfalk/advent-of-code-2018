fn parse_digits(number: &str) -> Vec<u8> {
    number.chars().map(|x| x.to_digit(10).unwrap() as u8).collect()
}

fn part_a(recipe_index: usize) -> String {
    let mut elf_a = 0;
    let mut elf_b = 1;
    let mut recipes: Vec<u8> = vec![3, 7];

    while recipes.len() < recipe_index + 10 {
        let new_recipe = parse_digits(&(recipes[elf_a] + recipes[elf_b]).to_string());
        recipes.extend(new_recipe);

        // Find new recipes
        elf_a = (elf_a + recipes[elf_a] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[elf_b] as usize + 1) % recipes.len();

        //println!("A: {}, B: {}, {:?}", elf_a, elf_b, recipes);
    }
    recipes[recipe_index..recipe_index + 10].iter().map(|x| x.to_string()).collect::<String>()
}

fn part_b(sequence: &str) -> usize {
    let mut elf_a = 0;
    let mut elf_b = 1;
    let mut recipes: Vec<u8> = vec![3, 7];
    let sequence = parse_digits(sequence);

    loop {
        let new_recipe = parse_digits(&(recipes[elf_a] + recipes[elf_b]).to_string());
        let added_two = new_recipe.len() > 1;
        recipes.extend(new_recipe);

        // Find new recipes
        elf_a = (elf_a + recipes[elf_a] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[elf_b] as usize + 1) % recipes.len();

        if recipes.len() < sequence.len() {
            continue;
        }

        let seq_start = recipes.len() - sequence.len();
        if recipes[seq_start..] == sequence[..] {
            return seq_start;
        } else if added_two && seq_start >= 1 && recipes[seq_start - 1..recipes.len() - 1] == sequence[..] {
            return seq_start - 1;
        }
    }
}


fn main() {
    println!("Answer A: {}", part_a(864801));
    println!("Answer B: {}", part_b("864801"));
}

#[test]
fn test_parse_digits() {
    assert_eq!(parse_digits("1234567890"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
}

#[test]
fn test_a() {
    assert_eq!(part_a(5), "0124515891");
    assert_eq!(part_a(18), "9251071085");
    assert_eq!(part_a(2018), "5941429882");
}

#[test]
fn test_b() {
    assert_eq!(part_b("51589"), 9);
    assert_eq!(part_b("01245"), 5);
    assert_eq!(part_b("92510"), 18);
    assert_eq!(part_b("59414"), 2018);
}
