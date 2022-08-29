// Defining struct for Linear Regression model
struct LinearRegression{
    data: Vec<u32>,
    label: Vec<u32>,
}

impl LinearRegression{
    fn get_mean(&self) -> u32{
        let sum: u32 = self.data.iter().sum();
        let length: usize = self.data.len();
        sum / length;
    }
}

fn main(){
    let data: Vec<u32> = vec![1, 2, 3, 4, 5];
    let label: Vec<u32> = vec![3, 4, 5, 6, 7];
    let regress = LinearRegression{
        data,
        label
    };

    let sum: u32 = regress.get_mean();


    println!("Linear regression {}", sum);


}