#[derive(Debug, PartialEq, Eq)] // Eq is used for tests, don't remove
pub struct Pair<T, U> {
    left: T,
    right: U,
}

impl<T, U> Pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self {
            left: first,
            right: second,
        }
    }

    pub fn swap(&self) -> Pair<U,T>
        where
            T:Clone,
            U:Clone
    {
        let new_left = self.right.clone();
        let new_right = self.left.clone();
        return Pair::new(new_left,new_right)
    }

    pub fn map_left<F, V>(&self, f: F) -> Pair<V, U>
        where
            T: Clone,
            U: Clone,
            F: FnOnce(&T) -> V,
    {
        Pair {
            left: f(&self.left),
            right: self.right.clone(),
        }
    }

    // Implementing map_right
    pub fn map_right<F, W>(&self, f: F) -> Pair<T, W>
        where
            T: Clone,
            U: Clone,
            F: FnOnce(&U) -> W,
    {
        Pair {
            left: self.left.clone(),
            right: f(&self.right),
        }
    }
}

impl<T: Default, U: Default> Default for Pair<T, U> {
    fn default() -> Self {
        Pair {
            left: T::default(),
            right: U::default(),
        }
    }
}

impl<T: Clone> Pair<T, T> {
    pub fn to_vec(&self) -> Vec<T> {
        vec![self.left.clone(), self.right.clone()]
    }

    pub fn to_array(&self) -> [T; 2] {
        [self.left.clone(), self.right.clone()]
    }
}


fn main() {
    let pair = Pair::new("🦜", 1);
    println!("{:?}", pair);
    println!("{:?}", pair.swap());
    println!("{:?}", pair.map_left(|_| 1));
    println!(
        "{:?}",
        pair.map_right(|x| {
            let bytes = [239, 158, 165, 155];
            let bytes = bytes.into_iter().map(|b| b + x).collect::<Vec<_>>();
            String::from_utf8(bytes)
        })
    );
}
