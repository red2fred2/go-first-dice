mod dice;
mod permutation;

use dice::Dice;

fn main() {
	let dice = Dice::new(
		vec![
			vec![1, 2, 3, 4],
			vec![1, 2, 3, 4],
		]
	);

	dice.count_permutations();
}
