use rand::random; // rand 크레이트를 지역범위로 가져온다.

static mut ERROR: isize = 0; // error를 0으로 초기화

struct File; // 크기가 0인 타입

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        // 이 조건식이 진행되면 8번 중 한 번은 참을 반환
        unsafe {
            // 정적 가변 변수에 접근해 이를 수정하려면 unsafe 블록이 필요하다.
            // 안정성 보장하지않으면 개발자가 책임진다.
            ERROR = 1; // 오류가 발생했음을 알리기 위한 값
        }
    }
    0 // 항상 0바이트를 읽은 것으로 간주
}

#[allow(unused_mut)] // 실제로 값을 바꾸지는 않지만 코드 일관성을 위해 버퍼를 가변 상태로 유지한다.
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        // 정적 가변 변수에 잡근하는 것은 안전하지 않다.
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}
