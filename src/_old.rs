fn main() {
    let x = "test";
    println!("x is: {}", x);

    let x = "test2";
    println!("x is: {}", x);

    let some_tuple = (1, "Test", true);
    let (num1, desc, is_deleted) = some_tuple;

    println!(
        "Hey, {}! Desc is: {}. Is deleted? {}",
        num1,
        desc,
        if is_deleted { "Yes" } else { "No" }
    );

    // -----

    // let string_as_reference = "I am a string.";
    // let mut string_as_mutable_reference = "I am a string.";

    // String is a vector of bytes, cannot mutate
    // &str is a reference type, pointing to the string

    let mut test_string = String::new();
    test_string.push('1');
    test_string.push('2');
    test_string.push('3');
    test_string.push_str("four five six");
    // println!("{}", test_string); // 123four five six

    for word in "This is a sentence".split_whitespace() {
        println!("Word: {}", word)
    }

    let test_string_2 = String::from("This is also working!");
    let mut test_string_2_as_vector: Vec<char> = test_string_2.chars().collect(); // without .collect(), it's of type "Chars"

    test_string_2_as_vector.sort(); // order by
    test_string_2_as_vector.dedup(); // distinct
    println!("{}", String::from_iter(test_string_2_as_vector));

    // -------

    enum Day {
        Mon,
        Sat,
        Sun,
    }

    impl Day {
        fn get_full_name(&self) -> &str {
            if matches!(self, Day::Mon) {
                return "Monday";
            } else {
                return "too tired to care";
            }
        }
    }

    impl Day {
        fn is_week_day(&self) -> bool {
            match self {
                Day::Sat | Day::Sun => false,
                _ => true,
            }
        }
    }

    println!(
        "Is {} a weekday? {}, and is {} a weekday? {}",
        Day::Mon.get_full_name(),
        Day::Mon.is_week_day(),
        Day::Sat.get_full_name(),
        Day::Sat.is_week_day()
    )

    // let test = "Hello".to_owned();
}
