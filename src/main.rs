mod dice;
mod permutation;

use dice::Dice;

fn main() {
	// The original go first dice that are permutation fair for 4 players
	// let dice = Dice::new(
	// 	vec![
	// 		vec![1, 8, 11, 14, 19, 22, 27, 30, 35, 38, 41, 48],
	// 		vec![2, 7, 10, 15, 18, 23, 26, 31, 34, 39, 42, 47],
	// 		vec![3, 6, 12, 13, 17, 24, 25, 32, 36, 37, 43, 46],
	// 		vec![4, 5, 9,  16, 20, 21, 28, 29, 33, 40, 44, 45],
	// 	]
	// );

	// 2 player matched go first dice
	// let dice = Dice::new(
	// 	vec![
	// 		vec![1, 4],
	// 		vec![2, 3],
	// 	]
	// );

	// 2 player mismatched go first dice
	// let dice = Dice::new(
	// 	vec![
	// 		vec![1, 3],
	// 		vec![2],
	// 	]
	// );

	// 3 player matched go first dice
	let dice = Dice::new(
		vec![
			vec![1, 5, 10, 11, 13, 17],
			vec![3, 4, 7,  12, 15, 16],
			vec![2, 6, 8,   9, 14, 18],
		]
	);

	dice.count_permutations();
}
