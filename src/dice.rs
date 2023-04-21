use std::collections::HashMap;

use super::permutation::Permutation;

pub struct Dice<T> {
	dice: Vec<Vec<T>>
}

impl<T> Dice<T> where T: Copy + std::fmt::Debug + Ord {
	pub fn new(dice: Vec<Vec<T>>) -> Self {
		Self{dice}
	}

	pub fn count_permutations(&self)
	 {
		let structure = self.get_structure();
		let permutations = Permutation::new(&structure);

		let mut counts = HashMap::new();

		for permutation in permutations {
			let rolls = self.get_rolls(&permutation);
			let ordering = Self::get_ordering(&rolls);

			let mut count = 1;

			if let Some(n) = counts.get(&ordering) {
				count += *n;
			}

			counts.insert(ordering, count);
		}

		println!("{counts:?}");
	}

	fn get_ordering(rolls: &Vec<T>) -> Vec<usize> {
		let mut enumerated: Vec<_> = rolls.iter().enumerate().collect();
		enumerated.sort_by_key(|(_, e)| *e);

		let mut ordered = vec![0; rolls.len()];

		for i in 0..enumerated.len() {
			let (o, _) = enumerated[i];
			ordered[o] = i + 1;
		}

		ordered
	}

	fn get_rolls(&self, permutation: &Vec<usize>) -> Vec<T> {
		let mut rolls = Vec::new();

		for die in 0..self.dice.len() {
			let perm = permutation[die];
			let value = self.dice[die][perm];

			rolls.push(value)
		}

		rolls
	}

	fn get_structure(&self) -> Vec<usize> {
		let mut structure = Vec::new();

		for die in &self.dice {
			structure.push(die.len());
		}

		structure
	}

}
