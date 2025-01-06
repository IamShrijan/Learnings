fn main() {
    //let v: Vec<i32> = Vec::new();
    //let mut v = Vec::new();

    //v.push(5);
    //v.push(6);
    //v.push(7);
    //v.push(8);
    //let third: &i32 = &v[2];
    //println!("The third element is {third}");

    //let third: Option<&i32> = v.get(2);
    //match third {
    //    Some(third) => println!("The third element is {third}"),
    //    None => println!("There is no third element."),
    //}
    //let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);
    //match does_not_exist {
    //    Some(does_not_exist) => println!("The third element is {does_not_exist}"),
    //    None => println!("There is no does_not_exist element."),
    // }
    let mut v = vec![1, 2, 3, 4, 5];
    //let mut first = &v[0]; this will give error as you cannot have two references to the same error. 
    let mut first = v[0];
    v.push(6);
    println!("The first element is: {first}");
}
