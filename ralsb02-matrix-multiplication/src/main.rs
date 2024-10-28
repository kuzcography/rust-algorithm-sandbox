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

fn matrix_multiplication(matrix_a: &Vec<Vec<i32>>, matrix_b: &Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let rows = matrix_a[0].len();
    let cols = matrix_b.len();
    let mut matrix = vec![vec![0; cols]; rows];
    let mamb = matrix_a.len();
    for i in 0..rows {
        for j in 0..cols {
            let mut arr: i32 = 0;
            for k in 0..mamb {
                arr += matrix_a[i][k] * matrix_b[k][j];
            }
            matrix[i][j] = arr;
        }
    }
    matrix
}

fn main(){
    let mut d = String::new();
    println!("Enter the number of rows in the matrix : ");
    io::stdin().read_line(&mut d).expect("Reading error");
    let nrows: i32 = d.trim().parse().expect("Please enter an integer");
    d.clear();
    println!("Enter the number of columns in the matrix : ");
    io::stdin().read_line(&mut d).expect("Reading error");
    let ncols: i32 = d.trim().parse().expect("Please enter an integer");
    let matrix_a = create_matrix(2, 2);
    let matrix_b = create_matrix(2, 2);
    let add = matrix_multiplication(&matrix_a,&matrix_b);
    for i in 0..2 {
        for j in 0..2{
            println!("{}", add[i][j]);
        }
    }
}