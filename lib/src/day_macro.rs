/**
Can be used to write the boilerplate for a test part binary.
Generates a main function that measures the duration it takes to get the answer, and prints it to std-out.
Also generates a 1-n test with this format, separated by `,`:
`(test_name ([raw("test data")] | [("./input-file.txt")]) => expected_result)`
For the following `test_name`s the following default `input-file` is assumed, otherwise you have to specify it:
```md
| test_name | input_file          |
|-----------|---------------------|
| test      | "test-input.txt"    |
| example   | "example-input.txt" |
| answer    | "input.txt"         |
```

# Examples

```ignore
# #[macro_use] extern crate lib;
lib::day!(09, part2, test => 3, answer => 42);
```

```ignore
# #[macro_use] extern crate lib;
lib::day!(
    09,
    part2,
    test raw("3") => 3,
    another_test raw("4 2") => 42
);
```

```
# #[macro_use] extern crate lib;
# fn part2(input: &str) -> usize {
#   input.replace(' ', "").parse().expect("should only contain numbers and space")
# }
lib::day_test!(
    09,
    part2,
    test raw("3") => 3,
    another_test raw("4 2") => 42
);
```
*/
#[macro_export]
macro_rules! day {
    ($day: literal, $part: expr, $($answers:tt)*) => {
        $crate::day_main!($day, $part);

        $crate::day_test!($day, $part, $($answers)*);
    };
}

#[macro_export]
macro_rules! day_main {
    ($day: literal, $part: expr) => {
        pub fn main() {
            let input = $crate::get_input_file!(answer);
            let output = $part(input);
        }
    };
}

#[macro_export]
macro_rules! day_test {
    (
        $day: literal,
        $part: expr,
        $($name: ident $($raw: ident)?$(($test_file: literal))? => $result: literal),+
    ) => {
        #[cfg(test)]
        $crate::paste::item! {
            mod [< day_ $day _ $part _tests >] {
                use super::*;

                $(
                    #[test]
                    fn [< day_ $day _ $name _works >]() {
                        let _timer = lib::PrintTimer::new("");

                        let result = $part($crate::get_input_file!($name $($raw)?$( $test_file)?));
                        assert_eq!(result, $result);
                    }
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! get_input_file {
    (test) => {
        include_str!("test-input.txt")
    };
    (example) => {
        include_str!("example-input.txt")
    };
    (answer) => {
        include_str!("input.txt")
    };
    ($_:ident raw $raw_test_content:literal) => {
        $raw_test_content
    };
    ($_:ident $test_file:literal) => {
        include_str!($test_file)
    };
}
