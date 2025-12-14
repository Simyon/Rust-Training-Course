#[path = "../tasks/c1_common_concepts.rs"]
mod c1_common_concepts;

use c1_common_concepts::*;

fn main() {
    println!("=== Тестирование функций из c1_common_concepts ===\n");

    // Тест 1: Изменяемость
    println!("1. Тест изменяемости:");
    simple_mutability();
    println!();

    // Тест 2: Типы данных
    println!("2. Тест типов данных:");
    simple_data_types();
    println!();

    // Тест 3: Функция квадрата
    println!("3. Тест функции square:");
    let test_numbers = [0, 1, 5, 10];
    for num in test_numbers {
        println!("square({}) = {}", num, square(num));
    }
    println!();

    // Тест 4: Факториал
    println!("4. Тест функции factorial:");
    let factorial_tests = [0, 1, 3, 5, 7];
    for num in factorial_tests {
        println!("factorial({}) = {}", num, factorial(num));
    }
    println!();

    // Тест 5: Проверка знака
    println!("5. Тест функции sign_checker:");
    let sign_tests = [-10, -1, 0, 1, 42];
    for num in sign_tests {
        println!("{} - {}", num, sign_checker(num));
    }
    println!();

    // Тест 6: Поиск наибольшего числа
    println!("6. Тест функции find_biggest_number:");
    let test_arrays = [
        [1, 2, 3, 4, 5],
        [10, 5, 8, 2, 9],
        [100, 50, 75, 25, 90],
        [7, 7, 7, 7, 7],
    ];
    
    for (i, array) in test_arrays.iter().enumerate() {
        println!("Массив {}: {:?} -> наибольшее: {}", 
                 i + 1, array, find_biggest_number(*array));
    }

    println!("\n=== Все тесты завершены! ===");
}