pub mod animal;

#[cfg(test)]
mod tests {
    // use animal::Dog;
    // use animal::Cat;
    use crate::animal::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn use_cat() {
        // cat::hello()
        assert_eq!(true,  cat::is_cat());
    }

    #[test]
    fn user_dog(){
        // assert_eq!(true,animal::Dog::is_dog());
        assert_eq!(true,dog::is_dog());
    }


}
