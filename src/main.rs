use std::io;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line"); // считываем N

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input value not integer"),
    }; // проверяем N на валидность

    let numbers: Vec<i32> = (1..=n).collect();

    let num_threads = num_cpus::get(); // получаем кол-во ядер процессора

    let chunk_size = (n as usize + num_threads - 1) / num_threads; // вычисляем размер блока

    let sum = Arc::new(AtomicU64::new(0)); // создаем атомарный объект типа u64, инициализируем его нулем, и создаем умный указатель Arc для безопасного доступа к данным в многопоточной среде

    let mut handles = vec![];

    // разбиваем массив на блоки
    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let sum = Arc::clone(&sum); // для каждого потока создаем отдельный указатель переменную хранящую сумму чисел
        let handle = thread::spawn(move || {
            let chunk_sum: u64 = chunk.iter().map(|&num| (num * num) as u64).sum(); // подсчитываем сумму квадратов каждого числа в чанке(блоке) и суммируем
            sum.fetch_add(chunk_sum, Ordering::SeqCst); // прибавляем полученное значение к общей сумме используя метод для атомарного сложения во избежание возникновения гонки данных
        });
        handles.push(handle);
    }

    // Завершаем работу всех потоков
    for handle in handles {
        handle.join().expect("Error");
    }
    println!("Сумма квадратов: {}", sum.load(Ordering::SeqCst)); // выводим результат
}
