// Usando a função da biblioteca que está em lib.rs
use meu_projeto::safe_sum_array;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let sum = safe_sum_array(&arr);
    println!("A soma do array é: {}", sum);
}
