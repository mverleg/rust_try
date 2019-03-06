
/// Inspired by:
/// https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1#avoid-boxtrait

trait ListElement: Sized {
    fn chain<T: ListElement>(self, other: T) -> ListItem<Self, T> {
        ListItem {
            value: self,
            next: other,
        }
    }
}

impl<T: Sized> ListElement for T {}

//#[derive(Debug)]
//enum ListLink<T> {
//    End,
//    Item(ListItem<T>),
//}

#[derive(Debug)]
struct ListItem<T, U> {
    value: T,
    next: U,
}


//
//enum ListLink<T: Elem> {
//    End(T),
//    Prev(T, T)
//}
//
//// impl<T: Elem> Elem for ListLink<T> {}
//
//impl<T: Elem> ListLink<T> {
//    fn add(self, item: T) -> ListLink<T> {
//        ListLink::Prev(self, item)
//    }
//}

// impl<T: Elem> ElemType<T> {
// //    fn get(&self) -> T {
// //
// //    }

//     fn add(self, value: T) -> ElemType<T> {
//         ElemType::Item(value)
//     }
// }

// impl<T: Elem> Elem for ElemType<T> {}

fn main() {
    let li = ListItem {
        value: "hello",
        next: ListItem {
            value: "world",
            next: ()
        }
    };
//        .add("hello")
//        .add("world");
    println!("{:?}", li);
}
