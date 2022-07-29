
struct Cat<'a> {
    name: &'a str
}

struct Dog {
    name: String
}

trait Animal {
    fn name(&self) -> &str;
}


impl Animal for Cat<'_> {
    fn name(&self) -> &str {
        self.name
    }
}

impl Animal for Dog {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}


fn foo<'a>(vec: &'a Vec<&'a dyn Animal>) -> &'a dyn Animal {
    let mut max_anim = *vec.first().unwrap();
    for idx in 0..vec.len() {
        if max_anim.name().len() < vec[idx].name().len() {
            max_anim = vec[idx];
        }
    }
    max_anim
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let dog1 = Dog { name: "dog123".to_string() };
        let dog2 = Dog { name: "dog2".to_string() };
        let cat = Cat { name: "cat" };

        let animals: Vec<&dyn Animal> = vec![&dog1, &dog2, &cat];

        println!("{:?}", foo(&animals).name());
    }

}