/// Esta função soma os elementos de um array apontado por `ptr`
/// considerando que o array possui `len` elementos.
///
/// # Segurança
///
/// - O ponteiro `ptr` deve ser válido e apontar para um array com pelo menos `len` elementos.
/// - **Atenção:** Esta implementação possui um erro intencional!
pub unsafe fn sum_array(ptr: *const i32, len: usize) -> i32 {
    let mut sum = 0;
    for i in 0..len {
        sum += *ptr.add(i);
    }
    sum
}

/// Função segura que encapsula `sum_array`, evitando warnings no `cargo test`
pub fn safe_sum_array(arr: &[i32]) -> i32 {
    // Chamamos `sum_array` dentro de `unsafe` para que a função pública seja segura
    unsafe { sum_array(arr.as_ptr(), arr.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        let sum = safe_sum_array(&arr);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_unsafe_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        let sum = unsafe { sum_array(arr.as_ptr(), arr.len()) };
        assert_eq!(sum, 15);
    }
}
