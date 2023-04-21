mod dice;
mod permutation;

use dice::Dice;

fn main() {
	// The original go first dice that are permutation fair for 4 players
	let go_first_dice = Dice::new(
		vec![
			vec![1, 8, 11, 14, 19, 22, 27, 30, 35, 38, 41, 48],
			vec![2, 7, 10, 15, 18, 23, 26, 31, 34, 39, 42, 47],
			vec![3, 6, 12, 13, 17, 24, 25, 32, 36, 37, 43, 46],
			vec![4, 5, 9,  16, 20, 21, 28, 29, 33, 40, 44, 45],
		]
	);

	go_first_dice.count_permutations();

	// let dice = Dice::new(
	// 	vec![
	// 		vec![1, 3, 5, 7],
	// 		vec![2, 4, 6, 8],
	// 	]
	// );

	// dice.count_permutations();
}
