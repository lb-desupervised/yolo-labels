# Yolo Label Parser

A simple rust library for parsing the yolo label format.

## Usage

In your `Config.toml`:

```toml
[dependencies]
yolo-labels = "0.1.0"
```

In your code:

```rust
use yolo_labels::Labels;

let labels = Labels::from_file("/path/to/file.txt")?;
labels.labels[0].label_index == 2;
```
