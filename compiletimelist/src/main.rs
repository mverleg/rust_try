
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

#[derive(Debug)]
struct ListItem<T, U> {
    value: T,
    next: U,
}

impl<T, U> ListItem<T, U> {
    fn get(&self) -> &U {
        &self.next
    }

    fn next(&self) -> &T {
        &self.value
    }
}

fn main() {
    let li = "hello".chain("world").chain("this").chain("is").chain("Mark").chain("speaking").chain("!");
    println!("{:?}", li);
    println!("{:?}", li.get());
    println!("{:?}", li.next().get());
    println!("{:?}", li.next().next().get());
    println!("{:?}", li.next().next().next().get());
}
