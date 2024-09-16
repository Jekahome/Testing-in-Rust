
# Testing

* [Tools cargo-nextest](https://github.com/Jekahome/Testing-in-Rust#tools-cargo-nextest)
* [Проблемы тестирования](https://github.com/Jekahome/Testing-in-Rust#проблемы-тестирования)
* [Негласные правила](https://github.com/Jekahome/Testing-in-Rust#негласные-правила)
* [Рекомендация по архитектуре тестов](https://github.com/Jekahome/Testing-in-Rust#рекомендация-по-архитектуре-тестов)
* [Unit tests](https://github.com/Jekahome/Testing-in-Rust#unit-tests)
* [Integration tests](https://github.com/Jekahome/Testing-in-Rust#integration-tests)
* [End-to-end (сквозное тестирование)](https://github.com/Jekahome/Testing-in-Rust#end-to-end-сквозное-тестирование)
* [Mocking (crate mockall)](https://github.com/Jekahome/Testing-in-Rust#mocking-crate-mockall)
* [Fuzz testing](https://github.com/Jekahome/Testing-in-Rust#fuzz-testing)
* [Property based testing](https://github.com/Jekahome/Testing-in-Rust#property-based-testing)
* [Generating fake data](https://github.com/Jekahome/Testing-in-Rust#generating-fake-data)
* [Как тестировать stdout вывод](https://github.com/Jekahome/Testing-in-Rust#как-тестировать-stdout-вывод)
* [Как тестировать ожидая аргументы командной строки](https://github.com/Jekahome/Testing-in-Rust#как-тестировать-ожидая-аргументы-командной-строки)
* [Как тестировать работу с файлами](https://github.com/Jekahome/Testing-in-Rust#как-тестировать-работу-с-файлами)
* [Как тестировать async function](https://github.com/Jekahome/Testing-in-Rust#как-тестировать-async-function)
* [Как успеть очистить env среду разработки после panic в тесте] (https://github.com/Jekahome/Testing-in-Rust#как-успеть-очистить-env-среду-разработки-после-panic-в-тесте)

## Tools cargo-nextest 

Предлагает более чистый интерфейс результатов теста, а также работает быстрее.

Важно: `cargo-nextest` это полная замена `cargo test`

```bash
$ cargo install cargo-nextest --locked
$ cargo nextest run
```

![cargo-nextest output](/cargo-nextest-output.png "This is a sample image.")

## Проблемы тестирования:

 - Часто имеет дело с пользовательским вводом, читает файлы и записывает выходные данные. 

 - Нет доступа к коду для его тестирования (к примеру код спаггети, где весь ф-ционал вместе).

p.s. надо сразу учитывать как протестировать этот функционал

 - После изменения API вам придется переписывать тестируемый код, возможно инкапсулируя интерфейс API check функцией вы избавитесь от переписывания всех тестов этого API
(Решение состоит в том, чтобы написать тесты для функций таким образом, чтобы они были независимы от кода.)

- Многослойные архитектуры должны включать интегрированные тесты для каждого уровня

 ```
  L1 <- Tests
  L1 <- L2 <- Tests
 ```

[matklad how-to-test](https://matklad.github.io/2021/05/31/how-to-test.html)

[matklad delete cargo integration tests](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html)

## Негласные правила:

 - имена тестов должны ясно показывать что тестировали (BDD)

 - проверять одну функциональность за раз

 - не тестируйте то что знаете, тестируйте функциональность отбросив контекст своего знания о библиотеке (как "черный ящик")

 - Тестируйте функциональность, а не реализацию (Это похоже на просмотр футбольного матча: вас больше волнует гол, чем каждый пас, ведущий к нему)

 - В общем, вы всегда должны стремиться тестировать поведение , а не функции, классы или модули. Это помогает исключить детали реализации из вашего теста и уменьшает зависимости.

![Pyramid](/Pyramid.png "This is a sample image.")

## [Рекомендация по архитектуре тестов](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Delete-Cargo-Integration-Tests)

p.s. терминология unit/integration основана исключительно на расположении функций #[test], а не на том, что эти функции на самом деле делают.

- рекомендация, больших проектах только один интеграционный тестовый ящик с несколькими модулями,
  чтобы недопустить увеличения времени компиляции тестов;

``` 
не делайте этого: ❌
tests/
  foo.rs
  bar.rs
```

```
Вместо этого сделайте это: ✅ 
tests/
  integration/
    mod.rs
    foo.rs
    bar.rs
```

- для библиотеки с публичным API, одиночные интеграционные тесты;

```
Для не больших crates:  ✅ 
tests/
  it.rs
```

```
Или для больших crates: ✅ 
tests/
  it/
    main.rs
    foo.rs
    bar.rs
```    

- для внутренней библиотеки, избегать интеграционных тестов. Вместо этого использовать модульные тесты;

```
src/
  lib.rs
  tests.rs
  tests/
     mod.rs
     integration_tests/
        foo.rs
        mod.rs
        bar.rs
     
где: 
lib.rs
  #[cfg(test)]
  mod tests;

mod.rs:
  #[cfg(test)]
  mod integration_tests;

integration_tests/mod.rs:
  #[cfg(test)]
  mod foo;
  #[cfg(test)]
  mod bar;

Запуск:
cargo run tests integration_tests
```

Даже если вы придерживаетесь модульных тестов, библиотека перекомпилируется дважды: один раз с `--test`, и один раз без `--test`,
отключите настройку test если вы используете модульные тесты
```
[lib]
test = false
```

## Unit tests

В Unit тесте можно протестировать приватные функции (они в одной области видимости)

Так как модульные тесты идут в тех же файлах что и основной код, вы будете использовать `#[cfg(test)]` чтобы указать, что они не должны быть включены в скомпилированный результат.

Модульное тестирование проверяет отдельные функции или «единицы» кода.

Модульные тесты выполняют две основные функции:

Убедитесь, что отдельные части программы ведут себя должным образом, и не допустить, чтобы будущие изменения изменили существующее поведение.

## Integration tests

Интеграционное тестирование. (lib должен быть)
(в отдельной папке tests, тестирует публичные методы)

Каждый файл в папке tests/ скомпилирован как отдельный ящик. 

Интеграционные тесты являются внешними по отношению к вашему ящику и
 используют только его открытый интерфейс так же, как любой другой код.

Запуск определенной функции из всех файлов интеграционных тестов
cargo test --test integration_test

Если наш проект является бинарным крейтом, который содержит только src/main.rs и не содержит src/lib.rs, то в таком случае, мы не можем создать интеграционные тесты в папке tests и подключить функции определённые в файле src/main.rs в область видимости с помощью выражения use. Только библиотечные крейты могут предоставлять функции, которые можно использовать в других крейтах; бинарные крейты предназначены только для самостоятельного запуска.

Это одна из причин того, что Rust проекты для выполняемой программы имеют просто файл src/main.rs,, который вызывает логику, которая находится в файле src/lib.rs. Используя такую структуру, интеграционные тесты могут протестировать библиотечный крейт с помощью use, чтобы подключить важную функциональность и сделать её доступной.

Каждый файл интеграционного теста компилируется как отдельный контейнер, что может отрицательно повлиять на время компиляции тестов. Группировка похожих тестов в одном файле может помочь уменьшить это влияние.

## End-to-end сквозное тестирование

[Puppetry](https://puppetry.app/)

Это интеграционные тесты которые более конкретно оценивают всю систему, иначе называемым сквозными тестами.

Вы не просто проверяете правильность своей логики (что можно проверить с помощью модульных тестов), но и то, что ваше программное обеспечение, оборудование, сеть и разрешения работают вместе.

Модульные тесты и локализованное тестирование сценариев использования могут проверить вашу бизнес-логику, но сквозной тест также проверяет инфраструктуру, в которой работает и с которой взаимодействует ваше программное обеспечение.

Для такого тестирования создается тестовое окружение (среда), идентичное окружению, в котором работают реальные пользователи. Тестируются все действия, которые пользователи могут выполнять в приложении.

Тестируется весь user flow (путь пользователя). Например, при разработке онлайн-магазина тестировщик «идет по пути пользователя» от входа посетителя на сайт и регистрации до завершения покупки.

Сквозные тесты самые медленные, потому что время уходит на билд, деплой, запуск приложения

## Mocking crate mockall

[crate mockall crates.io](https://crates.io/crates/mockall)

[crate mockall docs](https://docs.rs/mockall/0.11.4/mockall/)

Дает возможность создать под вашим контролем ложную функциональность, имитирующую поведение реальной функциональности. 
Затем эта функциональность группируется в макет объекта (или просто макет ).

Mocking тестирование — это подход к модульному тестированию, который позволяет вам делать утверждения о том, как тестируемый код взаимодействует с другими модулями системы. 
При макетном тестировании зависимости заменяются объектами, имитирующими поведение реальных. 
Цель Mocking — изолировать и сосредоточиться на тестируемом коде, а не на поведении или состоянии внешних зависимостей.

[Mockall](https://crates.io/crates/mockall) предоставляет инструменты для создания макетов практически любого признака или структуры.


## Fuzz testing

Фа́ззинг (англ. fuzzing) — техника тестирования программного обеспечения, часто автоматическая или полуавтоматическая, заключающаяся в передаче приложению на вход неправильных, неожиданных или случайных данных.

Цель фаззинга — завершить работу программы с различными входными данными, которые разработчики и тестировщики, возможно, не учли при написании тестов 

Нечеткое тестирование не поможет заставить программы работать должным образом, но поможет обнаружить источники сбоев программ

На данный момент в экосистеме Rust есть несколько инструментов для фаззинга . Наиболее известны:

Cargo -fuzz - это оболочка командной строки для использования libFuzzer.
afl.rs позволяет запускать AFL (американский fuzzy lop) на коде, написанном на Rust.
honggfuzz - это фаззер, ориентированный на безопасность, с мощными возможностями анализа, который поддерживает эволюционный фаззинг с обратной связью, основанный на покрытии кода (программном и аппаратном).

[youtube](https://www.youtube.com/watch?v=GXxH01b003E)

[complete-guide-to-testing-code-in-rust](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)

[wikipedia](https://ru.wikipedia.org/wiki/%D0%A4%D0%B0%D0%B7%D0%B7%D0%B8%D0%BD%D0%B3)
 
[Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html)

[crate cargo-fuzz](https://docs.rs/cargo-fuzz)

[crate honggfuzz](https://docs.rs/honggfuzz)
 
[crate afl](https://crates.io/crates/afl)

[how-to-organize-rust-tests](https://blog.logrocket.com/how-to-organize-rust-tests/)

[Effective Rust: Fuzz Testing](https://www.lurklurk.org/effective-rust/testing.html#fuzz-testing)

![fuzzy resources](/fuzz-target/fuzzy-resources.png "This is a sample image.")

## Property based testing

Test property validate

Тестирование свойств дает возможность исследовать случайную выборку из предопределенного пространства тестирования путем установки «свойств» тестовых данных.

Тесты свойств могут помочь найти тестовые входные данные, которые не проходят ваши тесты, но они не могут проверить все входные данные из-за ограниченного пространства для исследования. Если тест не пройден, крейт тестирования свойств Rust проходит процесс, называемый сжатием

Он позволяет проверить, что определенные свойства вашего кода сохраняются для произвольных входных данных, и, если обнаружен сбой, автоматически находит минимальный тестовый пример для воспроизведения проблемы. В отличие от QuickCheck, создание и сжатие определяется для каждого значения, а не для каждого типа, что делает его более гибким и упрощает композицию.

QuickCheck и Proptest во многом схожи: оба генерируют случайные входные данные для функции для проверки определенных свойств и автоматически сокращают входные данные до минимального количества отказов.

Одно большое отличие состоит в том, что QuickCheck генерирует и сжимает значения только на основе типа, тогда как Proptest использует явные Strategy объекты.

Если вы извлекли части своей программы и обнаружили, что пишете множество примеров в виде модульных тестов, пытаясь найти все крайние случаи, вам следует изучить proptest.

Представьте себе мишень для дартса . Традиционные модульные тесты подобны стрельбе в яблочко тщательно выбранной стрелой. Напротив , тестирование на основе свойств больше похоже на выпуск рая пчел на мишень и проверку, все ли они достигли допустимых целей .

[Property-based-testing-in-rust-with-arbitrary](https://www.greyblake.com/blog/property-based-testing-in-rust-with-arbitrary/)

[Property testing wikipedia](https://en.wikipedia.org/wiki/Property_testing)

[Property testing youtube](https://www.youtube.com/watch?v=uJ9qvWBcKA4)

[crate proptest crates.io](https://crates.io/crates/proptest)

[crate proptest docs](https://docs.rs/proptest/latest/proptest/)

[Books proptest](https://altsysrq.github.io/proptest-book/intro.html)

[crate quickcheck](https://crates.io/crates/quickcheck)


## Generating fake data

[crate fake crates.io](https://crates.io/crates/fake)

crate fake генерирует поддельные данные, такие как имена людей, веб-адреса, электронные письма, цвета, адреса, UUID и многое другое.



## Как тестировать stdout вывод

```rust
// Этот вариант не сможем протестировать, нет доступа к выводу
fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
// Этот вариант мы можем протестировать
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
```

## Как тестировать ожидая аргументы командной строки

```rust
[dev-dependencies]
assert_cmd = "2.0.11"
predicates = "3.0.3"

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("YOUR CRATE NAME LIBRARY")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
```

## Как тестировать работу с файлами

```rust
[dev-dependencies]
assert_fs = "1.0.13"

use assert_fs::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}
```

## [Как тестировать async function](https://blog.x5ff.xyz/blog/async-tests-tokio-rust/)

crate ditto_time

```rust
[dependencies]
actix-rt = "*"

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
  use super::*;
  
  #[test]
  fn test_str_len() {
    assert_eq!(str_len("x5ff"), 4);
  }

  #[actix_rt::test]
  async fn test_str_len_async() {
    assert_eq!(str_len_async("x5ff").await, 4);
  }
}
--------------------------------------------------
[dev-dependencies]
tokio-test = "*"

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
  use super::*;
  
  #[test]
  fn test_str_len() {
    assert_eq!(str_len("x5ff"), 4);
  }

  // ... the other async test

  macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
  }

  #[test]
  fn test_str_len_async_2() {
    assert_eq!(aw!(str_len_async("x5ff")), 4);
  }
}
```

[Actix Web TestRequest](https://actix.rs/docs/testing/)

## Как успеть очистить env среду разработки после panic в тесте

```rust
#[tokio::test]
async fn setup_create_account() {
    // setup env ...
  
    // run test
    let join_handle = tokio::spawn(async {
        test_unified().await?;
        Ok::<(), io::Error>(())
    });

    let err = match join_handle.await {
        Ok(Err(e)) => Some(Box::new(e)),
        Err(e) => Some(Box::new(e.into())),
        Ok(Ok(_)) => None,
    };

    // clear env ...

    if let Some(err) = err {
        println!("Test failed, rethrowing panic...");
        panic::resume_unwind(err);
    }
}
```