// just a one element tuple, wrapper around List
// this is whats called a tuple struct
pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // have to do take() because a mutable reference isnt copyable, unline a normal reference
        // this takes out the value form self.next and puts it in x and then make self.next =
        // x.next and returns a mutable ref to x.elem

        self.next.take().map(|x| {
            self.next = x.next.as_deref_mut();
            &mut x.elem
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|x| {
            // as_deref wiil give access to reference to the type of option I guess , self.next is
            // Option<&'a Node<T>> , map will change &'a Node<T> to
            // something else, it gets changes to &x.elem and also
            // next ptr is updated to x.next
            self.next = x.next.as_deref();
            //
            // Another way of doing the same thing
            // self.next = x.next.as_ref().map(|y| &**y);
            //
            // another way is with turbofish, rust can insert as many
            // *'s as it wants to make the types match
            // self.next = x.next.as_ref().map::<&Node<T>, _>(|y| &y);
            //
            &x.elem
        })
    }
}

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // no need for lifetimes here, as it has only one reference as input and one reference as
    // output, so the compiler itself adds the lifetime here,
    // it's  implicit
    // if it had more than one reference as input, it would require lifetime annotations, if the
    // compiler got confused that is
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            //
            // next: self.head.as_deref(),
            // the below match does the same exact thing as this
            next: match &(self.head) {
                // or self.head.as_ref()
                Some(x) => Some(&**x),
                None => None,
            },
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            // cur_link = node.next and the current value will go
            // out of scope
            cur_link = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(10);
        assert_eq!(list.peek(), Some(&10));
        list.push(11);
        assert_eq!(list.peek(), Some(&11));
        assert_eq!(list.peek_mut(), Some(&mut 11));
        let x = list.peek_mut();
        drop(x);
        list.peek_mut().map(|x| *x = 10);
        assert_eq!(list.peek(), Some(&10));
        list.push(100);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut x = list.into_iter();
        assert_eq!(x.next(), Some(2));
        assert_eq!(x.next(), Some(1));
        let mut list2 = List::new();
        list2.push(String::from_str("Hi").unwrap());
        list2.push(String::from_str("Hello").unwrap());
        let mut y = list2.into_iter();
        assert_eq!(y.next().as_deref(), Some("Hello"));
        assert_eq!(y.next().as_deref(), Some("Hi"));
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut x = list.iter();
        assert_eq!(x.next(), Some(&2));
        assert_eq!(x.next(), Some(&1));
        let mut list2 = List::new();
        list2.push(String::from_str("Hi").unwrap());
        list2.push(String::from_str("Hello").unwrap());
        let mut y = list2.iter();
        assert_eq!(y.next().map(|x| x.as_ref()), Some("Hello"));
        assert_eq!(y.next().map(|x| x.as_ref()), Some("Hi"));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let mut x = list.iter_mut();
        assert_eq!(x.next(), Some(&mut 2));
        assert_eq!(x.next(), Some(&mut 1));
        let mut list2 = List::new();
        list2.push(String::from_str("Hi").unwrap());
        list2.push(String::from_str("Hello").unwrap());
        let mut y = list2.iter_mut();
        assert_eq!(y.next(), Some(&mut String::from_str("Hello").unwrap()));
        assert_eq!(y.next(), Some(&mut String::from_str("Hi").unwrap()));
    }
}
