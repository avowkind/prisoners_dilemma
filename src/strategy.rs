
use choice::*;
use history::*;
pub type Id = &'static str;

pub trait Strategy {
    fn id(&self) -> Id;
    /// return the strategy's next choice, based on the given history
    fn choice(&self, history: &History) -> Choice;
}



/* Rust lets us create empty (Unit) structs, which are field-less. These are instantiated
   just using the struct name and can have traits associated.
*/

#[derive(Debug)]
pub struct Never;
impl Strategy for Never {
    fn id(&self) -> Id { "Never" }
    fn choice(&self, _history: &History) -> Choice { false }
}

#[derive(Debug)]
pub struct Always;
impl Strategy for Always {
    fn id(&self) -> Id { "Always" }
    fn choice(&self, _history: &History) -> Choice { true }
}

#[derive(Debug)]
pub struct AlternateTrueFalse;
impl Strategy for AlternateTrueFalse {
    fn id(&self) -> Id { "AlternateTrueFalse" }
    fn choice(&self, history: &History) -> Choice {
        // if the history is empty then return first choices
        // else return the opposite to the previous choice.
        match history.last() {
            Some(cp) => !cp.0,
            None => true,
        }
    }
}

#[derive(Debug)]
pub struct AlternateFalseTrue;
impl Strategy for AlternateFalseTrue {
    fn id(&self) -> Id { "AlternateFalseTrue" }
    fn choice(&self, history: &History) -> Choice {
        // if the history is empty then return first choices
        // else return the opposite to the previous choice.
        match history.last() {
            Some(cp) => !cp.0,
            None => false,
        }
    }
}

// TitForTat start with true, all future choices are the same as the opponents last choice.
#[derive(Debug)]
pub struct TitForTat;
impl Strategy for TitForTat {
    fn id(&self) -> Id { "TitForTat" }
    fn choice(&self, history: &History) -> Choice {
        // if the history is empty then return true
        // else return the same as the opponents previous
        match history.last() {
            Some(cp) => cp.1,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_never() {
        let history = History::new(0);
        let p1 = Never;
        assert!(!p1.choice(&history));
    }

    #[test]
    fn test_always() {
        let history = History::new(0);
        let p1 = Always;
        assert!(p1.choice(&history));
    }

    #[test]
    fn test_alternate1() {
        let mut history = History::new(2);
        let p1 = AlternateTrueFalse;
        assert!(p1.choice(&history));
        history.push(ChoicePair(true, true));
        assert!(!p1.choice(&history));
        history.push(ChoicePair(false, false));
        assert!(p1.choice(&history));
    }

    #[test]
    fn test_alternate2() {
        let mut history = History::new(2);
        let p1 = AlternateFalseTrue;
        assert!(!p1.choice(&history));
        history.push(ChoicePair(true, true));
        assert!(!p1.choice(&history));
        history.push(ChoicePair(false, false));
        assert!(p1.choice(&history));
    }

    #[test]
    fn test_titfortat() {
        let mut history = History::new(3);
        let p1 = TitForTat;
        assert!(p1.choice(&history));
        history.push(ChoicePair(true, true));
        assert!(p1.choice(&history));
        history.push(ChoicePair(true, false));
        assert!(!p1.choice(&history));
        history.push(ChoicePair(true, true));
        assert!(p1.choice(&history));
    }

}
