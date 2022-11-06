//! P2: Terminal user interface
//!
//! This problem explores the differences between designing systems with
//! classes and traits. The adjacent file `tui.cpp` provides a C++ implementation
//! of a terminal user inteface (TUI), i.e. a simple set of graphical elements that
//! can be drawn into the terminal. The C++ code uses classes, inheritance, and
//! virtual methods.
//!
//! To see the C++ code in action, you can build and execute the code by running:
//!
//! ```bash
//! ./run.cpp-sh
//! ```
//!
//! Your task is to redesign the C++ TUI API into Rust. Your API should similarly
//! contain data structures that represent Text, Heading, and Container. You should
//! replicate the behavior of `main` in `tui.cpp` into the `container_test` function.
//!
//! Note: Cargo's test harness silences printing by default. You can prevent that
//! behavior by running:
//!
//! ```bash
//! cargo test container -- --nocapture
//! ```

struct Dimensions {
    width: usize,
    height: usize,
}

trait Element {
    fn dimensions(self: &Self) -> Dimensions;
    fn render(self: &Self);
}

struct Text {
    text: String,
}

impl Element for Text {
    fn dimensions(self: &Self) -> Dimensions {
        return Dimensions {
            width: self.text.chars().count(),
            height: 1,
        };
    }

    fn render(self: &Self) {
        print!("{0}", self.text);
    }
}

struct Heading {
    text: String,
}

impl Element for Heading {
    fn dimensions(self: &Self) -> Dimensions {
        return Dimensions {
            width: self.text.chars().count(),
            height: 1,
        };
    }

    fn render(self: &Self) {
        print!("\u{001b}[1m{0}\u{001b}[0m", self.text);
    }
}

struct Container {
    children: Vec<Box<dyn Element>>,
}

impl Element for Container {
    fn dimensions(self: &Self) -> Dimensions {
        let mut max_width: usize = 0;
        let mut sum_height: usize = 0;
        for child in &self.children {
            let dims = child.as_ref().dimensions();
            max_width = max_width.max(dims.width);
            sum_height += dims.height;
        }
        return Dimensions {
            width: max_width + 2,
            height: sum_height,
        };
    }

    fn render(self: &Self) {
        let dims = self.dimensions();

        let render_line = || {
            print!("+");
            for _ in 0..dims.width - 2 {
                print!("-");
            }
            println!("+");
        };
        render_line();

        for child in &self.children {
            let child_dims = child.dimensions();
            print!("|");
            child.render();
            for _ in 0..dims.width - 2 - child_dims.width {
                print!(" ");
            }
            println!("|");
        }
        render_line();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn container_test() {
        // Your unit test goes here!
        let text = Heading {
            text: String::from("Hello world"),
        };
        let text2 = Text {
            text: String::from("This is a long string of text"),
        };
        let children: Vec<Box<dyn Element>> = vec![Box::new(text), Box::new(text2)];
        let container = Container { children: children };
        container.render();

        assert!(true);
    }
}
