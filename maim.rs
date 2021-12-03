// 泛型介绍
// 没有使用泛型的情况下
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list:&[char]) -> char {
    let mut largest  = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//fn  main() {
  //  let number_list = vec![1,2,3,4,3,234,534,1,55];
    //let max_number = largest_i32(&number_list);
    //println!("max_number:{}", max_number);
    //let char_list = vec!['a','a','c','y','z'];
    //let max_char = largest_char(&char_list);
    //println!("max_char:{}", max_char)
//}

// 使用泛型
// 在函数和方法中使用泛型

fn largest<T:PartialOrd + Copy> (list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}


fn  main() {
    let number_list = vec![1,2,3,4,3,234,534,1,55];
    let max_number = largest(&number_list);
    println!("max_number:{}", max_number);
    let char_list = vec!['a','a','c','y','z'];
    let max_char = largest(&char_list);
    println!("max_char:{}", max_char)
}
