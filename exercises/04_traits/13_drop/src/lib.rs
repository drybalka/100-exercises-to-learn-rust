// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

use core::panic;

struct DropBomb {
    is_defused: bool,
}

impl DropBomb {
    pub fn new() -> DropBomb {
        DropBomb { is_defused: false }
    }

    pub fn defuse(&mut self) {
        self.is_defused = true
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.is_defused {
            panic!("Boom!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
