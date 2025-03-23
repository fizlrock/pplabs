use std::{sync::Arc, thread, time::Instant};

use crossbeam::queue::SegQueue;

fn integral<F>(f: F, a: f64, b: f64, n: i32) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b)); // Начальные точки

    for i in 1..n {
        let x = a + (i as f64) * h;
        sum += f(x);
    }
    sum * h
}

fn integral_taskbag<F>(f: F, a: f64, b: f64, n: i32) -> f64
where
    F: Fn(f64) -> f64 + Send + Sync + 'static,
{
    println!("Запуск");
    // Создаем мешок задач через SegQueue
    let taskbag = Arc::new(SegQueue::new());

    // Шаг интегрирования
    let h = (b - a) / (n as f64);
    // Базовая сумма для краёв интервала
    let base_sum = 0.5 * (f(a) + f(b));

    println!("Создание задач..");
    // Заполняем мешок задачами (индексы от 1 до n-1)
    for i in 1..n {
        taskbag.push(i);
    }

    let num_threads = 4;
    let mut handles = vec![];

    // Оборачиваем f в Arc для безопасного доступа из потоков
    let f = Arc::new(f);

    println!("Запуск потоков");
    // Запускаем потоки
    for _ in 0..num_threads {
        let taskbag = Arc::clone(&taskbag);
        let f = Arc::clone(&f);
        let handle = thread::spawn(move || {
            let mut local_sum = 0.0; // Локальная сумма для потока
            while let Some(i) = taskbag.pop() {
                let x = a + (i as f64) * h;
                local_sum += f(x);
            }
            local_sum
        });
        handles.push(handle);
    }
    println!("Ожидаем завершения...");

    // Суммируем результаты всех потоков
    let thread_sums: f64 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum::<f64>();

    // Итоговый результат
    (base_sum + thread_sums) * h
}

fn integral_improved<F>(f: F, a: f64, b: f64, n: i32) -> f64
where
    F: Fn(f64) -> f64 + Send + Sync + 'static,
{
    let h = (b - a) / (n as f64);
    let f_arc = Arc::new(f);
    let base_sum = 0.5 * (f_arc(a) + f_arc(b));

    let total_interior = (n - 1) as usize;
    let num_threads = 8usize;
    let base = total_interior / num_threads;
    let remainder = total_interior % num_threads;

    let mut ranges = vec![];
    let mut current = 1i32;
    for t in 0..num_threads {
        let size_t = base + if t < remainder { 1 } else { 0 };
        let size_t_i32 = size_t as i32;
        let start_t = current;
        let end_t = start_t + size_t_i32 - 1;
        ranges.push((start_t, end_t));
        current = end_t + 1;
    }

    let mut handles = vec![];
    for (start, end) in ranges {
        let f_clone = Arc::clone(&f_arc);
        let handle = thread::spawn(move || {
            let local_sum: f64 = (start..=end).map(|i| f_clone(a + (i as f64) * h)).sum();
            local_sum
        });
        handles.push(handle);
    }

    let thread_sums: f64 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    (base_sum + thread_sums) * h
}

fn test(n: i32) {
    let a = 0.0; // Нижний предел интегрирования
    let b = 100000.0; // Верхний предел интегрирования

    {
        println!("Taskbag");
        let function = |x: f64| ((((x + 100.0) * x + 100.0) * x + 100.123).sin()); // Функция для интегрирования
        let start = Instant::now();
        let result = integral_taskbag(function, a, b, n);
        let duration = start.elapsed();
        println!("Время выполнения: {:.2?}", duration);
        println!("Интеграл от x по отрезку [{}, {}] ≈ {:.6}", a, b, result);
    }
    {
        println!("Simple");
        let function = |x: f64| ((((x + 100.0) * x + 100.0) * x + 100.123).sin()); // Функция для интегрирования
        let start = Instant::now();
        let result = integral(function, a, b, n);
        let duration = start.elapsed();
        println!("Время выполнения: {:.2?}", duration);
        println!("Интеграл от x по отрезку [{}, {}] ≈ {:.6}", a, b, result);
    }
    {
        println!("Imporved");
        let function = |x: f64| ((((x + 100.0) * x + 100.0) * x + 100.123).sin()); // Функция для интегрирования
        let start = Instant::now();
        let result = integral_improved(function, a, b, n);
        let duration = start.elapsed();
        println!("Время выполнения: {:.2?}", duration);
        println!("Интеграл от x по отрезку [{}, {}] ≈ {:.6}", a, b, result);
    }
}

fn main() {
    let n = 10_i32.pow(2); // Количество разбиений
    println!("10*2");
    test(n);
    println!("");
    println!("");

    let n = 10_i32.pow(3); // Количество разбиений
    println!("10*3");
    test(n);
    println!("");
    println!("");

    let n = 10_i32.pow(4); // Количество разбиений
    println!("10*4");
    test(n);
    println!("");
    println!("");

    let n = 10_i32.pow(5); // Количество разбиений
    println!("10*5");
    test(n);
    println!("");
    println!("");

    let n = 10_i32.pow(6); // Количество разбиений
    println!("10*6");
    test(n);
    println!("");
    println!("");

    let n = 10_i32.pow(7); // Количество разбиений
    println!("10*7");
    test(n);
    println!("");
    println!("");

    let n = 10_i32.pow(8); // Количество разбиений
    println!("10*8");
    test(n);
    println!("");
    println!("");
}
