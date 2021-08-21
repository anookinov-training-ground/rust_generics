fn find_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    let result = find_largest(&number_list);
    println!("The largest number is {}", result);

    println!("{:#?}", number_list);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    let result = find_largest(&number_list);
    println!("The largest number is {}", result);
}
