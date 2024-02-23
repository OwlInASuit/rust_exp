fn median(a: Vec<f32>) -> Option<f32> {
    todo!();
}

fn main(){
    let answer : Option<f32> = median(vec![1.0, 2.0, 5.0]);
    println!("median([1, 2, 5]) = {:?}", answer);
}

#[test]
fn empty_list(){
    let input: Vec<f32> = vec![];
    let expected_output: Option<f32> = None;
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list(){
    let input: Vec<f32> = vec![1.0, 4.0, 5.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length(){
    let input: Vec<f32> = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list(){
    let input: Vec<f32> = vec![1.0, 5.0, 2.0];
    let expected_output: Option<f32> = Some(2.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}
