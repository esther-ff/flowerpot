#[cfg(test)]
mod tests {
    use flowerpot::FlowerPot;

    #[test]
    fn pushing() {
        let mut pot = FlowerPot::<i32, 4>::new();

        assert!(
            !pot.full(),
            "unexpected: `FlowerPot` was not meant to be full at this stage"
        );
        assert!(
            pot.empty(),
            "unexpected: `FlowerPot` was meant to be empty at this stage"
        );

        for num in 1..5 {
            pot.push(num).unwrap()
        }

        assert!(
            pot.get_init_slice() == &[1, 2, 3, 4],
            "unexpected: invalid contents of flower pot"
        )
    }

    #[test]
    fn trying_till_full() {
        const SIZE: usize = 4;
        let mut pot = FlowerPot::<i32, SIZE>::new();

        for num in 1..33_i32 {
            if num as usize >= SIZE + 1 {
                assert!(
                    pot.push(num).is_err(),
                    "`push` at full capacity should fail"
                )
            } else {
                assert!(pot.push(num).is_ok(), "`push` should work while not full")
            };
        }

        assert!(pot.get_init_slice() == &[1, 2, 3, 4], "invalid contents");
    }

    #[test]
    fn popping() {
        const SIZE: usize = 4;
        let mut pot = FlowerPot::<i32, SIZE>::new();

        [1, 2, 3, 4]
            .into_iter()
            .for_each(|number| pot.push(number).unwrap());

        assert!(pot.pop().unwrap() == 4);
        assert!(pot.pop().unwrap() == 3);
        assert!(pot.pop().unwrap() == 2);
        assert!(pot.pop().unwrap() == 1);

        assert!(pot.pop().is_none());

        assert!(pot.empty());
    }
}
