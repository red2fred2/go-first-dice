pub struct Permutation {
	structure: Vec<usize>,
	permutation: Vec<usize>,
	first_run: bool,
}

impl Permutation {
	pub fn new(structure: &Vec<usize>) -> Self {
		let structure = structure.clone();
		let permutation = vec![0; structure.len()];
		let first_run = true;

		Self {structure, permutation, first_run}
	}

	pub fn is_last(&self) -> bool {
		for i in 0..self.structure.len() {
			if self.permutation[i] != self.structure[i] - 1 {
				return false
			}
		}

		true
	}
}

impl Iterator for Permutation {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
		// Check first run
		if self.first_run {
			self.first_run = false;
			return Some(self.permutation.clone())
		}

		// Check last run
		if self.is_last() {
			return None
		}

		let mut i = self.structure.len() - 1;
		while self.permutation[i] == self.structure[i] - 1 {
			self.permutation[i] = 0;
			i -= 1;
		}

		self.permutation[i] += 1;

		Some(self.permutation.clone())
    }
}
