// Import (via `use`) the `fmt` module to make it available
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented.
// This is a tuple struct named `Structure` that contains an `i32`.
// #[derive(Debug)]
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.

impl fmt::Display for Structure {
    // This trait requires `fmt` with this extract signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 출력할 수 있는 (포맷형)을 구현하는 함수
        // `fmt::Result`라는 것을 반환하여 출력하는 형태인 것으로 추측
        // 출력하기 위해서는 `write!` 라는 것으로 해야하는 것으로 보임
        // `f`는 스트림, 즉 출력할 대상인 것 같다.
        // 그리고 왜 인지는 모르겠으나, `write!` 뒤에 세미콜론을 붙이면 타입 에러가 발생한다.
        write!(f, "{}", self.0)
    }
}

fn main() {
    let structure = Structure(6349);
    println!("print structure {}", structure);
}
