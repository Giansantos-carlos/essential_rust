use std::slice::Split;
mod firt_function;
mod Functions;


fn main() {

    let novel = String::from("Call me Isgamel. some years ago...");
    let first_sentece = novel.split(".").next().unwrap();

    let i: ImportantExcerpt<'_> = ImportantExcerpt{
        part: first_sentece,
    };


    println!("Excerpt :{}", i.part);

    let result = firt_function::sumi(8, 10);

    println!("result is : {}", result);

    let dividii =  Functions::my_fuctions::divide(10, 2);

    println!("divide is {}", dividii)

}


struct ImportantExcerpt<'a> {
    part: &'a str,
}