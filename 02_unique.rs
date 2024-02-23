fn unique(mut a: Vec<i32>) -> Vec<i32> {
    
    //todo!(); // comment
    a.sort_unstable(); //sorts elements by their amount
    println!({:?}, a)
    a.dedup(); //undos dupes
    a

}

fn main(){
    let answer : Vec<f32> = unique(vec![1.0, 2.0, 5.0, 2.0, 5.0]);
    println!("unique([1, 2, 5, 2, 5]) = {:?}", answer);
}

#[test]
fn empty_list(){
    let input: Vec<f32> = vec![];
    let expected_output: Vec<f32> = Some(vec![]);
    let actual-output: Vec<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn sorted_without_duplicates(){
    let input: Vec<f32> = vec![1.0, 4.0, 5.0];
    let expected_output: Vec<f32> = Some(vec![1.0, 4.0, 5.0]);
    let actual_output: Vec<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn sorted_with_duplicates(){
    let input: Vec<f32> = vec![1.0, 4.0, 4.0, 5.0, 5.0];
    let expected_output: Vec<f32> = Some(vec![1.0, 4.0, 5.0]);
    let actual_output: Vec<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn unsorted_without_duplicates(){
    let input: Vec<f32> = vec![1.0, 5.0, 4.0];
    let expected_output: Vec<f32> = Some(vec![1.0, 5.0, 4.0]);
    let actual_output: Vec<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn unsorted_with_duplicates(){
    let input: Vec<f32> = vec![1.0, 5.0, 4.0, 4.0, 1.0];
    let expected_output: Vec<f32> = Some(vec![1.0, 5.0, 4.0]);
    let actual_output: Vec<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}
