mod dice;
mod permutation;

use dice::Dice;

fn main() {
	let dice = Dice::new(
		vec![
			vec![1, 3, 5, 7],
			vec![2, 4, 6, 8],
		]
	);

	dice.count_permutations();
}
