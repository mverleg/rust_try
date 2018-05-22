

use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Error};

trait Table {
    fn table_name() -> String;
}

struct Id<T : Table> {
    val: i32,
    _phantom: PhantomData<T>,
}
impl<T : Table> Id<T> {
    pub fn new(id: i32) -> Id<T> {
        return Id::<T> {
            val: id,
            _phantom: PhantomData {},
        };
    }
}
impl<T : Table> PartialEq for Id<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        return self.val == other.val;
    }
}
impl<T: Table> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{0:}Id:{1:}", T::table_name(), self.val)
    }
}

#[derive(Debug)]
struct User {
    id: Id<User>,
}
impl Table for User {
    fn table_name() -> String {
        return "User".to_owned();
    }
}

#[derive(Debug)]
struct Post {
    id: Id<Post>,
    user_id: Id<User>,
}
impl Table for Post {
    fn table_name() -> String {
        return "Post".to_owned();
    }
}

fn main() {
    let id1 = Id::<User>::new(12);
    let id2 = Id::<User>::new(12);
    let id3 = Id::<User>::new(55);
    // let id4 = Id::<Post>::new(12);
    println!("{:?} == {:?} => {:?}", id1, id2, id1 == id2); // true
    println!("{:?} == {:?} => {:?}", id1, id2, id1 == id3); // false
    // This comparison fails to compile, which is the desired behaviour!
    //   println!("{:?} == {:?} => {:?}", id1, id2, id1 == id4);

    let user = User {
        id: Id::<User>::new(12),
    };
    let post = Post {
        id: Id::<Post>::new(12),
        user_id: Id::<User>::new(12),
    };
    println!("{:?}", user.id == post.user_id);
    // This comparison fails to compile, which is the desired behaviour!
    //   println!("{:?}", post.id == post.user_id);
    
    let user2 = User {
        id: Id::<User>::new(12),
    };
    println!("{:?}", user.id == user2.id);
    // This fails, as it should
    //   println!("{:?}", user.id < user2.id);
    
    // println!("{}", id1);
    println!("{:?}", id1);
}




