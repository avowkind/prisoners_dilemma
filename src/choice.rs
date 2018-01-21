pub type Choice = bool;

#[derive(Debug)]
#[derive(Clone)]
pub struct ChoicePair(pub Choice, pub Choice);

impl ChoicePair {
    pub fn new(a: bool, b: bool) -> Self {
        ChoicePair(a, b)
    }

    pub fn score( &self ) -> (u32, u32 ){
        match (self.0, self.1) {
            (false, false) => (  0,   0),
            (false, true)  => ( 20,   0),
            (true,  false) => (  0,  20),
            (true,  true)  => (100, 100),
        }
    }

    // return a new pair with the elements swapped.
    pub fn swap( &self) -> ChoicePair {
        ChoicePair (self.1, self.0 )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choicepair() {
        let ff = ChoicePair(false, false);
        // destructure ChoicePair
        let ChoicePair (a, b) = ff;
        assert_eq!(a, false);
        assert_eq!(b, false);

        let tf = ChoicePair(true,false);
        assert_eq!(tf.0, true);
        assert_eq!(tf.1, false);

        let ChoicePair (a, b) = tf.swap();
        assert!(!a);
        assert!(b);

        let tf1 = ChoicePair::new(true,false);
        let tf2 = tf1.clone();
        assert_eq!(tf1.0, tf2.0);
        assert_eq!(tf1.1, tf2.1);


    }
}
