use std::io;
fn create_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(rows);
    let mut input = String::new();
    
    for i in 0..rows {
        let mut mrows: Vec<i32> = Vec::with_capacity(cols);
        for j in 0..cols {
            input.clear();
            println!("Enter element ({}, {}):", i + 1, j + 1);
            io::stdin().read_line(&mut input).expect("Reading error");
            let value: i32 = input.trim().parse().expect("Please enter an integer");
            mrows.push(value);
        }
        matrix.push(mrows)
    }
    matrix
}

fn matrix_determinant_2_dim(matrix:Vec<Vec<i32>>) -> i32 {
    let det = matrix[0][0]*matrix[1][1] - matrix[0][1]*matrix[1][0];
    det
}

fn matrix_determinant_3_dim(matrix:Vec<Vec<i32>>) -> i32 {
    let det_pos = (matrix[0][0]*matrix[1][1]*matrix[2][2])+(matrix[0][1]*matrix[1][2]*matrix[2][0])+(matrix[0][2]*matrix[1][0]*matrix[2][1]);
    let det_neg = (matrix[0][2]*matrix[1][1]*matrix[2][0])+(matrix[0][1]*matrix[1][0]*matrix[2][2])+(matrix[0][0]*matrix[1][2]*matrix[2][1]);
    let det = det_pos - det_neg;
    det
}


fn main() {
    let ma = create_matrix(3,3);
    let determinant = matrix_determinant_3_dim(ma);
    println!("{}", determinant);
}
