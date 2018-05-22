
use std::marker::PhantomData;

#[derive(Debug)]
struct Id<T> {
    val: i32,
    _phantom: PhantomData<T>,
}
impl<T> Id<T> {
    pub fn new(id: i32) -> Id<T> {
        return Id::<T> {
            val: id,
            _phantom: PhantomData {},
        };
    }
}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Id<T>) -> bool {
        return self.val == other.val;
    }
}

#[derive(Debug)]
struct ModelAlpha {
    id: Id<ModelAlpha>,
}

#[derive(Debug)]
struct ModelBeta {
    id: Id<ModelBeta>,
    alpha_id: Id<ModelAlpha>,
}

fn main() {
    let id1 = Id::<ModelAlpha>::new(12);
    let id2 = Id::<ModelAlpha>::new(12);
    let id3 = Id::<ModelAlpha>::new(55);
    let id4 = Id::<ModelBeta>::new(12);
    println!("{:?} == {:?} => {:?}", id1, id2, id1 == id2);
    println!("{:?} == {:?} => {:?}", id1, id2, id1 == id3);
    // This comparison fails to compile, which is the desired behaviour!
    //   println!("{:?} == {:?} => {:?}", id1, id2, id1 == id4);

    let alpha = ModelAlpha {
        id: Id::<ModelAlpha>::new(12),
    };
    let beta = ModelBeta {
        id: Id::<ModelBeta>::new(12),
        alpha_id: Id::<ModelAlpha>::new(12),
    };
    println!("{:?}", alpha.id == beta.alpha_id);
    // This comparison fails to compile, which is the desired behaviour!
    //   println!("{:?}", beta.id == beta.alpha_id);
}


