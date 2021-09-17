use std::path::Path;

/// YOLO label.
#[derive(Debug)]
pub struct Label {
	pub label_index: i8,
	pub x_centre: f32,
	pub y_centre: f32,
	pub width: f32,
	pub height: f32,
	pub probability: Option<f32>,
	pub object_id: Option<i8>,
}

pub trait Unnormaliser {
	fn unnormalise(&self, dimensions: (u32, u32)) -> Self;
}

impl From<&str> for Label {
	fn from(s: &str) -> Self {
		let split: Vec<&str> = s.split(" ").collect();

		let probability = if split.len() > 5 {
			Some(split[5].parse().unwrap())
		} else {
			None
		};

		let object_id = if split.len() > 6 {
			Some(split[6].parse().unwrap())
		} else {
			None
		};

		Self {
			label_index: split[0].parse().unwrap(),
			x_centre: split[1].parse().unwrap(),
			y_centre: split[2].parse().unwrap(),
			width: split[3].parse().unwrap(),
			height: split[4].parse().unwrap(),
			probability,
			object_id,
		}
	}
}

impl Unnormaliser for Label {
	fn unnormalise(&self, dimensions: (u32, u32)) -> Self {
		let width = dimensions.0 as f32;
		let height = dimensions.1 as f32;
		Self {
			label_index: self.label_index,
			x_centre: self.x_centre * width,
			y_centre: self.y_centre * height,
			width: self.width * width,
			height: self.height * height,
			probability: self.probability,
			object_id: self.object_id,
		}
	}
}

pub struct Labels {
	// We have to use a nested item because we can't implement From<String>
	// directly on Vec<Label>
	labels: Vec<Label>,
}

impl From<&str> for Labels {
	fn from(s: &str) -> Self {
		Labels {
			labels: s.split("\n").map(Label::from).collect::<Vec<Label>>(),
		}
	}
}

impl Labels {
	pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
		let string = std::fs::read_to_string(path)?;
		Ok(Labels::from(string.as_str()))
	}
}

// Enables `labels.iter()`
impl std::ops::Deref for Labels {
	type Target = [Label];

	fn deref(&self) -> &Self::Target {
		&self.labels[..]
	}
}

impl Unnormaliser for Labels {
	fn unnormalise(&self, dimensions: (u32, u32)) -> Self {
		Labels {
			labels: self
				.labels
				.iter()
				.map(|x| x.unnormalise(dimensions))
				.collect(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::io::Write;

	fn within(a: f32, b: f32, c: f32) {
		assert!(a < b);
		assert!(b < c);
	}

	#[test]
	fn label_from_string() {
		let label_string = "-1 0.603856 0.368098 0.048642 0.075372";
		let label = Label::from(label_string);
		assert_eq!(label.label_index, -1);
		within(0.603756, label.x_centre, 0.603956);
		within(0.367098, label.y_centre, 0.369098);
		within(0.048542, label.width, 0.048742);
		within(0.075362, label.height, 0.075382);
	}

	#[test]
	fn labels_from_string() {
		let labels_string = "-1 0.603856 0.368098 0.048642 0.075372\n\
		                     -1 0.603856 0.368098 0.048642 0.075372";
		let labels = Labels::from(labels_string);
		assert_eq!(labels.labels.len(), 2);
	}

	#[test]
	fn labels_from_file() {
		let labels_string = "-1 0.603856 0.368098 0.048642 0.075372\n\
		                     -1 0.603856 0.368098 0.048642 0.075372";

		let mut file = tempfile::NamedTempFile::new().unwrap();
		write!(file, "{}", labels_string).unwrap();

		let labels = Labels::from_file(file.path()).unwrap();
		assert_eq!(labels.labels.len(), 2);

		let new_labels = labels.unnormalise((2000, 1300));
		assert_eq!(new_labels.labels.len(), 2);
	}

	#[test]
	fn try_iterating_labels() {
		let labels_string = "-1 0.603856 0.368098 0.048642 0.075372\n\
		                     -1 0.603856 0.368098 0.048642 0.075372";
		let labels = Labels::from(labels_string);
		for label in labels.iter() {
			assert_eq!(label.label_index, -1);
		}
	}
}
