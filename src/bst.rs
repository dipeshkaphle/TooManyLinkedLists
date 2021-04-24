use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

pub struct BST<T> {
    root: Link<T>,
}

pub struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord + Display> Node<T> {
    pub fn new(key: T) -> Node<T> {
        Node {
            elem: key,
            left: None,
            right: None,
        }
    }

    pub fn min(&self) -> &T {
        &self.min_node().elem
    }

    pub fn min_node(&self) -> &Node<T> {
        match self.left.as_ref() {
            Some(x) => x.as_ref().min_node(),
            None => &self,
        }
    }

    pub fn max(&self) -> &T {
        &self.max_node().elem
    }

    pub fn max_node(&self) -> &Node<T> {
        match self.right.as_ref() {
            Some(x) => x.as_ref().max_node(),
            None => &self,
        }
    }

    pub fn search(&self, key: &T) -> bool {
        if key == &self.elem {
            true
        } else if key < &self.elem {
            match self.left.as_ref() {
                Some(x) => x.as_ref().search(key),
                None => false,
            }
        // return self.left.as_ref().map(|x| x.as_ref().search(key)).unwrap();
        } else {
            match self.right.as_ref() {
                Some(x) => x.as_ref().search(key),
                None => false,
            }
            // return self.right.as_ref().map(|x| x.as_ref().search(key)).unwrap();
        }
    }

    pub fn insert(&mut self, key: T) {
        match &self.elem.cmp(&key) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => match &mut self.right {
                Some(x) => {
                    x.as_mut().insert(key);
                }
                None => {
                    self.right.replace(Box::new(Node::new(key)));
                }
            },
            std::cmp::Ordering::Greater => match &mut self.left {
                Some(x) => {
                    x.as_mut().insert(key);
                }
                None => {
                    self.left.replace(Box::new(Node::new(key)));
                }
            },
        }
    }

    pub fn inorder(&self) {
        self.left.as_ref().map(|x| x.inorder());
        print!("{} ", &self.elem);
        self.right.as_ref().map(|x| x.inorder());
    }
    pub fn pre_order(&self) {
        print!("{} ", &self.elem);
        self.left.as_ref().map(|x| x.pre_order());
        self.right.as_ref().map(|x| x.pre_order());
    }

    pub fn post_order(&self) {
        self.left.as_ref().map(|x| x.post_order());
        self.right.as_ref().map(|x| x.post_order());
        print!("{} ", &self.elem);
    }

    pub fn to_vec_as_ref<'a>(&'a self) -> Vec<&'a T> {
        let mut x = match self.left.as_ref() {
            Some(y) => y.to_vec_as_ref(),
            None => vec![],
        };
        x.push(&self.elem);
        let y = match self.right.as_ref() {
            Some(z) => z.to_vec_as_ref(),
            None => vec![],
        };
        [x, y].concat()
        // for i in y {
        //     x.push(i);
        // }
        // x
    }
}

impl<T: Ord + Display> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn search(&self, key: &T) -> bool {
        if let Some(x) = self.root.as_ref() {
            return x.as_ref().search(key);
        } else {
            false
        }
    }

    pub fn insert(&mut self, key: T) {
        match self.root.as_mut() {
            Some(x) => x.insert(key),
            None => {
                self.root.replace(Box::new(Node {
                    elem: key,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    pub fn inorder(&self) {
        match self.root.as_ref() {
            Some(x) => x.inorder(),
            None => {}
        }
        println!("\n");
    }

    pub fn pre_order(&self) {
        self.root.as_ref().map(|x| x.pre_order());
        // match self.root.as_ref() {
        //     Some(x) => x.pre_order(),
        //     None => {}
        // }
        println!("\n");
    }
    pub fn post_order(&self) {
        self.root.as_ref().map(|x| x.post_order());
        // match self.root.as_ref() {
        //     Some(x) => x.pre_order(),
        //     None => {}
        // }
        println!("\n");
    }

    pub fn to_vec_as_ref(&self) -> Option<Vec<&T>> {
        self.root.as_ref().map(|x| x.to_vec_as_ref())
    }

    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|x| x.min())
    }

    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|x| x.max())
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    pub fn basics() {
        let mut tree = BST::new();
        tree.insert(10);
        assert_eq!(tree.search(&10), true);
        tree.insert(12);
        assert_eq!(tree.search(&12), true);
        tree.insert(8);
        assert_eq!(tree.search(&8), true);
        assert_eq!(tree.search(&11), false);
        tree.insert(20);
        assert_eq!(tree.search(&20), true);
        assert_eq!(tree.min(), Some(&8));
        assert_eq!(tree.max(), Some(&20));
        assert_eq!(tree.max(), Some(&20));
    }
}
