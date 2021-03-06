pub struct Permutator<T: Clone> {
    length: usize,
    value: Option<T>,
    rec_permutator: Option<Box<Permutator<T>>>,
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
                rec_permutator: None,
                curr_perm: None,
                curr_pos: 0,
            }
        } else {
            let length = values.len();
            let value = values[0].clone();
            Permutator {
                length,
                active: true,
                value: Some(value),
                curr_perm: None,
                curr_pos: 0,
                rec_permutator: if length == 1 { None } else { Some(Box::new(Permutator::new(&values[1..]))) }
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
                match self.curr_perm.as_mut() {
                    Some(perm) => {
                        perm[self.curr_pos - 1] = perm[self.curr_pos].clone();
                        perm[self.curr_pos] = self.value.as_ref().unwrap().clone();
                        self.curr_pos += 1;
                        if self.curr_pos == self.length {
                            let res = perm.clone();
                            self.curr_perm = None;
                            Some(res)
                        } else {
                            Some(perm.clone())
                        }
                    },
                    None => {
                        if let Some(mut rec_perm) = self.rec_permutator.as_mut().expect("Unexpected missing recursive permutation").next() {
                            let mut new_perm = Vec::with_capacity(self.length);
                            new_perm.push(self.value.as_ref().unwrap().clone());
                            new_perm.append(&mut rec_perm);
                            let res = new_perm.clone();
                            self.curr_pos = 1;
                            self.curr_perm = Some(new_perm);
                            Some(res)
                        } else {
                            self.active = false;
                            return None;
                        }
                    }
                }
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
