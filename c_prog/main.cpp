#include <cmath>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>
#include <vector>

// Предполагаемая функция для интегрирования
double func(double x) {
  return x * x; // Пример: интегрируем x^2
}

// Параллельная версия функции с использованием taskbag
double integrate_parallel(double a, double b, int intervals) {
  double step = (b - a) / intervals;
  double base_sum = 0.5 * (func(a) + func(b)); // Сумма краевых точек

  // Создаем очередь задач и мьютекс для синхронизации
  std::queue<int> task_queue;
  std::mutex mtx;

  // Заполняем очередь индексами от 1 до intervals-1
  std::cout << "Создание задач...\n";
  for (int i = 1; i < intervals; ++i) {
    task_queue.push(i);
  }

  // Количество потоков (можно настроить)
  const int num_threads = 8;
  std::vector<std::thread> threads;
  std::vector<double> thread_sums(num_threads,
                                  0.0); // Локальные суммы для каждого потока

  std::cout << "Запуск потоков...\n";
  // Запускаем потоки
  for (int t = 0; t < num_threads; ++t) {
    threads.emplace_back([t, &task_queue, &mtx, &thread_sums, a, step]() {
      double local_sum = 0.0;
      while (true) {
        int i;
        {
          std::lock_guard<std::mutex> lock(mtx);
          if (task_queue.empty())
            break;
          i = task_queue.front();
          task_queue.pop();
        }
        double x = a + i * step;
        local_sum += func(x);
      }
      thread_sums[t] = local_sum;
    });
  }

  std::cout << "Ожидаем завершения...\n";
  // Ждем завершения всех потоков
  for (auto &thread : threads) {
    thread.join();
  }

  // Суммируем результаты всех потоков
  double total_sum = 0.0;
  for (const auto &sum : thread_sums) {
    total_sum += sum;
  }

  // Финальный результат по формуле трапеций
  return (base_sum + total_sum) * step;
}

int main() {

  for (int i = 3; i <= 8; i++) {
    auto start = std::chrono::high_resolution_clock::now();

    printf("C++ TaskBag. 10^%d\n", i);
    double result = integrate_parallel(0.0, 1.0, std::pow(10, i));

    // Конец замера времени
    auto end = std::chrono::high_resolution_clock::now();
    auto duration =
        std::chrono::duration_cast<std::chrono::nanoseconds>(end - start);

    std::cout << "Время выполнения: " << duration.count() << "нс\n";

    std::cout << "Результат: " << result << "\n";
  }
  return 0;
}
