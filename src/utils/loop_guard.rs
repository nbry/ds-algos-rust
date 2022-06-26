/// Utility for panicking if loop goes longer than expected.
///
/// Use this utility when a loop is expected to run a certain amount of times
/// and/or to prevent infinite loops.
///
/// i.e. Use as a guard rail when figuring out algos.
pub struct LoopGuard {
    pub max_ticks: i32,
    pub count: i32,
}

impl LoopGuard {
    pub fn new(max: i32) -> LoopGuard {
        LoopGuard {
            max_ticks: max,
            count: 0,
        }
    }

    pub fn check(&mut self) {
        self.count += 1;
        if self.count > self.max_ticks {
            panic!("Max count reached - should not have hit this point");
        }
    }
}

#[test]
#[should_panic]
fn loop_goes_past_max_ticks() {
    // Arrange
    let mut guard = LoopGuard::new(1000);

    // Act
    loop {
        // Infinite loop - should obviously panic
        guard.check();
    }
}
