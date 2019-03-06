
/// Inspired by:
/// https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1#avoid-boxtrait

trait ListElement: Sized {
    fn chain<T: ListElement>(self, other: T) -> ListItem<Self, T> {
        ListItem {
            left: self,
            right: other,
        }
    }
}

impl<T: Sized> ListElement for T {}

#[derive(Debug)]
struct ListItem<T, U> {
    left: T,
    right: U,
}

impl<T, U> ListItem<T, U> {
    fn get(&self) -> &U {
        &self.right
    }

    fn prev(&self) -> &T {
        &self.left
    }
}

fn main() {
    // Create the list (in reverse order)
    let li = "hello".chain("world").chain("this").chain("is").chain("Mark").chain("speaking").chain("!");

    // Index the third item from the end
    println!("{:?}", li.prev().prev().prev().get());
}
