use std::{ops::Deref, rc::Rc};

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|x| {
            // Rc is also basically a pointer which is reference counted, thats why we do
            // as_deref()
            self.next = x.next.as_deref();
            &x.elem
        })
    }
}

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            // and_then is like >>= in haskell
            // a monad
            // M a -> (a -> M b) -> M b in haskell signature
            // >>=  == and_then , takes in an options strips it out , applies the closure and
            // wraps it up as option again
            head: self.head.as_ref().and_then(|x| x.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.as_ref().elem)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(x) = curr {
            // so this is garbage collected
            // Rc doesnt allow us to mutate the thing inside it until its the last reference
            // This is probably because if we do this the things that were sharing this
            // will be invalidated I guess
            // Rust is against that
            // That would possibly violate the one mutable reference at one time rule I guess
            // So this isnt allowed
            // Rc::try_unwrap will allow us to modify the Node inside Rc if and only if it is the
            // last reference or it will throw error through Result type
            if let Ok(mut y) = Rc::try_unwrap(x) {
                curr = y.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::List;

    #[test]
    pub fn basic_tests() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(10).append(20).append(30);
        assert_eq!(list.head(), Some(&30));

        let list = list.tail();
        assert_eq!(list.head(), Some(&20));
        let list = list.tail();
        assert_eq!(list.head(), Some(&10));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new();
        let list = list.append(1).append(2);
        let mut x = list.iter();
        assert_eq!(x.next(), Some(&2));
        assert_eq!(x.next(), Some(&1));
        let list2 = List::new();
        let list2 = list2
            .append(String::from_str("Hi").unwrap())
            .append(String::from_str("Hello").unwrap());
        let mut y = list2.iter();
        assert_eq!(y.next().map(|x| x.as_ref()), Some("Hello"));
        assert_eq!(y.next().map(|x| x.as_ref()), Some("Hi"));
    }
}
