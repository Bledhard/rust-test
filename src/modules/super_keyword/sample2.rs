pub fn example() {
    ::modules::super_keyword::sample1::example();
    ::modules::super_keyword::sample1::nested::example();
}

pub fn super_example() {
    super::sample1::example();
    super::sample1::nested::example();
}