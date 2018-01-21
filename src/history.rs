/* History
Holds the memory of each set of rounds - a double list of choices.
makes it easy to get the last set of results and add a new result.
we separate the history data from the strategy implementation in order to allow strategies
to play against themselves without colliding data. 
*/
use choice::*;

pub struct History {
    pub left: Vec<Choice>,
    pub right: Vec<Choice>
}

impl History {
    pub fn new(rounds: usize) -> History {
        History {
            left: Vec::with_capacity(rounds),
            right: Vec::with_capacity(rounds)
        }
    }
    pub fn last(&self) -> Option<ChoicePair> {
        let pa = self.left.last();
        let pb = self.right.last();

        match (pa, pb) {
            (Some(a), Some(b)) => {
                Some( ChoicePair::new(*a,*b) )
            },
            _ => None
        }
    }

    // add a new set of results to the history
    // ownership passes to the list
    pub fn push(&mut self, cp: ChoicePair) {
        let ChoicePair(a, b) = cp;
        self.left.push(a);
        self.right.push(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_history() {
        let mut history = History::new(2);
        match history.last() {
            Some(_) => assert!(false),
            None => assert!(true)
        }

        let cp = ChoicePair(true, true);
        history.push(cp);
        match history.last() {
            Some(_cp) => assert!(true),
            None => assert!(false)
        }
        let cp = ChoicePair(true, false);
        history.push(cp);
        match history.last() {
            Some(cp) => {
                assert_eq!(cp.0, true);
                assert_eq!(cp.1, false);
            }
            None => assert!(false)
        }

    }


}
