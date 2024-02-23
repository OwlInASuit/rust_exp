fn unique(mut a: Vec<i32>) -> Vec<i32> {
    
    //todo!(); // comment
    a.sort_unstable(); //sorts elements together [1, 1, 4, 4, 5]
    println!("{:?}", a);
    a.dedup(); //undos dupes
    a
}

fn unique_t<T: Ord + std::fmt::Debug>(mut a: Vec<T>) -> Vec<T> {
    
    //todo!(); // comment
    a.sort_by(|x: &T, y:&T| x.cmp(y)); //sorts elements together [1, 1, 4, 4, 5]
    println!("{:?}", a);
    a.dedup(); //undos dupes
    a

}

fn main(){
    let answer : Vec<i32> = unique_t(vec![1, 2, 5, 2, 5]);
    println!("unique([1, 2, 5, 2, 5]) = {:?}", answer);
}

#[test]
fn empty_list(){
    let input: Vec<i32> = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output: Vec<i32> = unique_t(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn sorted_without_duplicates(){
    let input: Vec<i32> = vec![1, 4, 5];
    let expected_output: Vec<i32> = vec![1, 4, 5];
    let actual_output: Vec<i32> = unique_t(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn sorted_with_duplicates(){
    let input: Vec<i32> = vec![1, 4, 4, 5, 5];
    let expected_output: Vec<i32> = vec![1, 4, 5];
    let actual_output: Vec<i32> = unique_t(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn unsorted_without_duplicates(){
    let input: Vec<i32> = vec![1, 5, 4];
    let expected_output: Vec<i32> = vec![1, 5, 4];
    let actual_output: Vec<i32> = unique_t(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn unsorted_with_duplicates(){
    let input: Vec<i32> = vec![1, 5, 4, 4, 1];
    let expected_output: Vec<i32> = vec![1, 5, 4];
    let actual_output: Vec<i32> = unique_t(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}
