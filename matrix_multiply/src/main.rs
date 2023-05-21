fn main() {

    let first_matrix = vec![vec![1,2,3],vec![4,5,6]];

    let second_matrix = vec![vec![1,2,3,4], vec![5,6,7,8], vec![9,10,11,12]];
    
    println!("Matrix multiplication of A & B is {:?}", matrix_multiply(first_matrix, second_matrix));
}

fn matrix_multiply(first: Vec<Vec<i32>>, second: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    for row1 in first.iter() {
        let mut row_f: Vec<i32> = vec![0;second[0].len()];
        for i in 0..first[0].len() {
            row_f = row_f
                .iter()
                .zip(second[i].iter().map(|x| x * row1[i]))
                .map(|(a, b)| a + b)
                .collect::<Vec<i32>>();
        }
        ans.push(row_f);
    }
    ans
}
