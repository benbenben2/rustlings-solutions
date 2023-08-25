// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`s into while let and if let
        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        // while let mut integer = cursor {
        //     Some(Some(integer)) = optional_integers.pop();// else {None};
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // } 

        while let Some(integer) = optional_integers.pop().flatten() {
            assert_eq!(integer, range);
            range -= 1;
        }

        // assert_eq!(cursor, 0); why removed?????
    }
}
