pub struct Details {
    pub name: String,
    pub age: i32,
}

pub trait Detail {
    fn get_details(&self);
}

impl Detail for Details {
    fn get_details(&self) {
        println!("Detail: Name: {:} Age: {:}", self.name, self.age);
    }
}
