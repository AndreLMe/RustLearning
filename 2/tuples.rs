use std::fmt;

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "\n( {} {} )\n( {} {} )", 
        self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (f32, f32)) -> (f32, f32)
{
    let (first, second) = pair;
    (second, first)
}

fn tranpose(matrix: Matrix) -> Matrix
{
    let (second, third) = reverse((matrix.1, matrix.2));

    Matrix(matrix.0, second, third, matrix.3)
}

fn main()
{
    let matrix = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("Matrix: {}", matrix);
    println!("Transpose: {}", tranpose(matrix));
}