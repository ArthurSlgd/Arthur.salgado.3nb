fn main() {
    // 1 - código direto, utilizando funções nativas.
    let mut arr = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];
    println!("nivel 1");
    println!("array original: {:?}", arr);
    arr.sort();
    println!("array ordenado: {:?}", arr);

    // 2 -  uso de struct e impl para organizar o algoritmo QuickSort
    println!("\nnível 2");

    let mut arr2 = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];
    let mut sorter = QuickSorter::new(&mut arr2);
    sorter.sort();
    sorter.print_sorted();
}
struct QuickSorter<'a> {
    arr: &'a mut [i32],
}

impl<'a> QuickSorter<'a> {
    fn new(arr: &'a mut [i32]) -> Self {
        println!("Array original: {:?}", arr);
        QuickSorter { arr }
    }
    fn sort(&mut self) {
        let len = self.arr.len();
        if len > 1 {
            self.quick_sort(0, (len - 1) as isize);
        }
    }
    fn quick_sort(&mut self, low: isize, high: isize) {
        if low < high {
            let p = self.partition(low, high);
            self.quick_sort(low, p - 1);
            self.quick_sort(p + 1, high);
        }
    }
    fn partition(&mut self, low: isize, high: isize) -> isize {
        let pivot = self.arr[high as usize];
        let mut i = low - 1;

        for j in low..high {
            if self.arr[j as usize] <= pivot {
                i += 1;
                self.arr.swap(i as usize, j as usize);
            }
        }
        self.arr.swap((i + 1) as usize, high as usize);
        i + 1
    }
    fn print_sorted(&self) {
        println!("Array ordenado: {:?}", self.arr);
    }
}
