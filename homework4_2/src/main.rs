fn main() {
    let number_list = vec![07, 18, 90, 09, 85];
    
    let result = sum (&number_list);
    match result{
        Some(x) => println!("The sum of the list is {}", x),
        None => println!("Overflow"),
    }
}



fn sum (list: &[u32]) -> Option<u32> {
    //check if input is u32 list, if not, return none
    //if yes, add the number and return a sum
    let temp:u32 = list.iter().sum();
    // I dont know how to check overflow here
    if temp > 4294967295 {
        None
    } else {
        Some(list.iter().sum())
    }
    //let mut sum; // = &list[0];

    //for item in list {
    //    sum = sum + item;
    //}
    
    
}