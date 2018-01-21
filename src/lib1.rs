type Choice = bool;

pub struct Player {
    pub strat: Box<Strategy>
}
impl Player {
    fn new<T: Strategy>(s: T) -> Self {
        Player { strat: Box::new(s) }
    }
}

pub trait Strategy {
    // Static method signature; `Self` refers to the implementor type.
    fn new() -> Self;

    /// return the strategy's next choices
    fn choice(&mut self) -> Choice;

    /// let the strategy know the opponents last choice
    /// default impl does nothing.
    fn set_oppo_choice(&mut self, _oppo: Choice) { }

    /// reset the strategy before a new game
    /// default impl does nothing.
    fn reset(&mut self) { }

    /// self identify the type.
    fn name(&self) -> &'static str ;
}

pub struct Never { }
impl Strategy for Never {
    fn name(&self) -> &'static str { "Never" }
    fn new() -> Never { Never {} }
    fn choice(&mut self) -> Choice { false }
}

pub struct Always {}
impl Strategy for Always {
    fn name(&self) -> &'static str { "Always" }
    fn new() -> Always { Always {} }
    fn choice(&mut self) -> Choice { true }
}

pub struct AlternateTrueFalse { prev: Choice }
impl Strategy for AlternateTrueFalse {
    fn name(&self) -> &'static str { "AlternateTrueFalse" }
    fn new() -> AlternateTrueFalse {
        AlternateTrueFalse { prev: false }
    }
    fn choice(&mut self) -> Choice {
        self.prev = !self.prev;
        self.prev
    }
    fn reset(&mut self) {
        self.prev = false;
    }
}

pub struct AlternateFalseTrue { prev: Choice }
impl Strategy for AlternateFalseTrue {
    fn name(&self) -> &'static str { "AlternateFalseTrue" }
    fn new() -> AlternateFalseTrue {
        AlternateFalseTrue { prev: true }
    }
    fn choice(&mut self) -> Choice {
        self.prev = !self.prev;
        self.prev
    }
    fn reset(&mut self) {
        self.prev = true;
    }
}

// TitForTat start with true, all future choices are the same as the opponents last choice.
pub struct TitForTat { prev: Choice }
impl Strategy for TitForTat {
    fn name(&self) -> &'static str { "TitForTat" }
    fn choice(&mut self) -> Choice {
        self.prev
    }
    fn new() -> TitForTat {
        TitForTat { prev: true }
    }
    fn reset(&mut self) {
        self.prev = true;
    }
    fn set_oppo_choice(&mut self, oppo: Choice) {
        self.prev = oppo;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_never() {
        let mut p1 = Never::new();
        assert!(!p1.choice());
    }

    #[test]
    fn test_always() {
        let mut p1 = Always::new();
        assert!(p1.choice());
    }

    #[test]
    fn test_alternate1() {
        let mut p1 = AlternateTrueFalse::new();
        assert!(p1.choice());
        assert!(!p1.choice());
        assert!(p1.choice());
    }

    #[test]
    fn test_alternate2() {
        let mut p1 = AlternateFalseTrue::new();
        assert!(!p1.choice());
        assert!(p1.choice());
        assert!(!p1.choice());
    }

    #[test]
    fn test_titfortat() {
        let mut p1 = TitForTat::new();
        assert!(p1.choice());
        assert!(p1.choice());
        p1.set_oppo_choice(false);
        assert!(!p1.choice());
        assert!(!p1.choice());
        p1.set_oppo_choice(true);
        assert!(p1.choice());
        assert!(p1.choice());
    }

    #[test]
    fn test_player() {
        let mut strat = TitForTat::new();
        let mut p1 = Player::new(strat);
        assert!(!p1.choice());
    }
}
