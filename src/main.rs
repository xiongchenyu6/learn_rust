use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

fn is_op(input: &char) -> bool {
    let op = ['+', '*'];
    let mut flag: bool = false;
    for elem in op.iter() {
        if elem == input {
            flag = true;
            break;
        }
    }
    flag
}
#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) -> () {}
}

// fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
// where
//     K: Clone + Eq + Hash,
//     V: Default,
// {
//     match map.get_mut(&key) {
//         Some(value) => value,
//         None => {
//             map.insert(key.clone(), V::default());
//             map.get_mut(&key).unwrap()
//         }
//     }
// }

//struct BugBox<T> {
//    d: *const T,
//    // _marker: std::marker::PhantomData<T>,
//}
//
//impl<T> BugBox<T> {
//    fn new(val: T) -> BugBox<T> {
//        let p = unsafe { alloc(Layout::new::<T>()) as *mut _ };
//        unsafe {
//            ptr::write(p, val);
//        }
//        BugBox {
//            d: p,
//            // _marker: Default::default(),
//        }
//    }
//}
//
//impl<T> Drop for BugBox<T> {
//    fn drop(&mut self) {
//        let d = unsafe { ptr::read(self.d) };
//        std::mem::drop(d);
//        unsafe {
//            dealloc(self.d as *mut _, Layout::new::<T>());
//        }
//    }
//}
//
//struct Safe1<'a>(&'a str, &'static str);
//
//unsafe impl<'a> Drop for Safe1<'a> {
//    fn drop(&mut self) {
//        println!("Safe1(_, {}) knows when *not* to inspect.", self.1);
//    }
//}
//
//struct SafeS<'a> {
//    b: Option<BugBox<Safe1<'a>>>,
//    s: String,
//}

fn main() {
//    let mut ss = SafeS {
//        b: None,
//        s: "".to_string(),
//    };
   // ss.b = Some(BugBox::new(Safe1(&ss.s, "")));

    // Pancakes::hello_macro();

    // polish to reverse polish
    // "+ 1 * 2 3" to "1 2 3 * +" left to right scan "2 3 * 1 +"
    // "* + 1 2 3" to "1 2 + 3 *"
    let input = ['*', '5', '+', '1', '*', '2', '3'];

    let mut i = input.len();
    let mut output = ['0'; 7];
    let mut j = 0;
    let a: usize = 0;

    while i > a {
        i -= 1;
        println!("{}", i);
        if is_op(&input[i]) {
            if j == 0 {
                let arg1 = input[i + 1];
                let arg2 = input[i + 2];

                output[j] = arg1;
                j += 1;

                output[j] = arg2;
                j += 1;

                output[j] = input[i];
                j += 1;
            } else {
                let arg1 = input[i + 1];

                output[j] = arg1;
                j += 1;

                output[j] = input[i];
                j += 1;
            }
        }
    }
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    println!("{:?}", loan);
    foo.share();
}
