pub struct Permutator<T: Clone> {
    length: usize,
    value: Option<T>,
    rec_permutation: Option<Box<Permutator<T>>>,
    curr_perm: Option<Vec<T>>,
    curr_pos: usize,
    active: bool
}

impl<T: Clone> Permutator<T> {
    pub fn new(values: &[T]) -> Permutator<T> {
        if values.is_empty() {
            Permutator {
                length: 0,
                active: false,
                value: None,
                rec_permutation: None,
                curr_perm: None,
                curr_pos: 0,
            }
        } else {
            let length = values.len();
            let value = values[0].clone();
            Permutator {
                length: values.len(),
                active: true,
                value: Some(value),
                curr_perm: None,
                curr_pos: 0,
                rec_permutation: if length == 1 { None } else { Some(Box::new(Permutator::new(&values[1..]))) }
            }
        }
    }
}

impl<T: Clone> Iterator for Permutator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.active {
            if self.length == 1 {
                self.active = false;
                return Some(vec![self.value.as_ref().unwrap().clone()]);
            } else {
                let mut perm = match self.curr_perm.as_ref() {
                    Some(perm) => {
                        perm.clone()
                    },
                    None => {
                        self.curr_perm = self.rec_permutation.as_mut().expect("Unexpected missing recursive permutation").next();
                        if let Some(perm) = self.curr_perm.as_ref() {
                            self.curr_pos = 0;
                            perm.clone()
                        } else {
                            self.active = false;
                            return None;
                        }
                    }
                };
                if self.curr_pos < perm.len() {
                    perm.insert(self.curr_pos, self.value.as_ref().unwrap().clone());
                    self.curr_pos += 1
                } else {
                    perm.push( self.value.as_ref().unwrap().clone());
                    self.curr_perm = None;
                }
                Some(perm)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut permutation = Permutator::new(&["test"]);
        assert_eq!(permutation.next(),Some(vec!["test"]));
        assert_eq!(permutation.next(),None);

        let mut permutation = Permutator::new(&[1,2]);
        assert_eq!(permutation.next(),Some(vec![1,2]));
        assert_eq!(permutation.next(),Some(vec![2,1]));
        assert_eq!(permutation.next(),None);

        let mut permutation = Permutator::new(&[1,2,3]);
        assert_eq!(permutation.next(),Some(vec![1,2,3]));
        assert_eq!(permutation.next(),Some(vec![2,1,3]));
        assert_eq!(permutation.next(),Some(vec![2,3,1]));
        assert_eq!(permutation.next(),Some(vec![1,3,2]));
        assert_eq!(permutation.next(),Some(vec![3,1,2]));
        assert_eq!(permutation.next(),Some(vec![3,2,1]));
        assert_eq!(permutation.next(),None);

        let mut permutation = Permutator::new(&["1","2","3"]);
        assert_eq!(permutation.next(),Some(vec!["1","2","3"]));
        assert_eq!(permutation.next(),Some(vec!["2","1","3"]));
        assert_eq!(permutation.next(),Some(vec!["2","3","1"]));
        assert_eq!(permutation.next(),Some(vec!["1","3","2"]));
        assert_eq!(permutation.next(),Some(vec!["3","1","2"]));
        assert_eq!(permutation.next(),Some(vec!["3","2","1"]));
        assert_eq!(permutation.next(),None);

    }
}
