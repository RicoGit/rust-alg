use std::marker::PhantomData;

pub struct HList<Head, Tail> {
    pub head: PhantomData<Head>,
    pub tail: Tail,
}

pub struct HNil;

pub trait HListOps {
    fn add<E>(self) -> HList<E, Self> where Self: Sized;
}

impl HListOps for HNil {
    fn add<E>(self) -> HList<E, Self> {
        HList {
            head: PhantomData::<E>::default(),
            tail: HNil,
        }
    }
}

impl<Head, Tail> HListOps for HList<Head, Tail> {
    fn add<E>(self) -> HList<E, Self> {
        HList {
            head: PhantomData::<E>::default(),
            tail: self,
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::hlist::{HList, HListOps, HNil};

    #[test]
    fn test() {

        let _res: HList<Vec<()>, HList<String, HList<i32, HNil>>> =
            HNil.add::<i32>()
            .add::<String>()
            .add::<Vec<()>>();
    }
}
