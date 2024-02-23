fn median(mut a: Vec<f32>) -> Option<f32> {
    //todo!(); // comment
    if a.is_empty(){
        return None;
    }
    //a.sort(); // f32 can't be compared without establishing a criteria
    //a.sort_by(compare: |x: &f32, y: &f32| {x.partial_cmp(y).unwrap()}); // ???
    a.sort_by(|x, y| x.partial_cmp(y).expect("Success"));

    //find middle value
    let n_elements: usize = a.len();
    let middle: usize = n_elements / 2; // floor
    let a_is_even = n_elements % 2 == 0;

    // you may declare a thing's value with an if
    let med : f32 = if a_is_even {
        // mean  / average of the middle-two elements
        (a[middle] + a[middle-1]) / 2.0
    } else {
        a[middle]
    };

    Some(med)

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
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn sorted_list(){
    let input: Vec<f32> = vec![1.0, 4.0, 5.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn even_length(){
    let input: Vec<f32> = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output: Option<f32> = Some(4.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

#[test]
fn unsorted_list(){
    let input: Vec<f32> = vec![1.0, 5.0, 2.0];
    let expected_output: Option<f32> = Some(2.0);
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output); // pass/fail
}

