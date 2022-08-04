#[allow(unused_variables)] // 컴파일러 경고를 완화한다.

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)] // 사용하지 않는 함수에 대한 컴파일러 경고를 완화한다.
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // ! 반환 타입은 이 함수가 절대로 어떤 값도 반환하지 않는다고 컴파일러에게 알려준다.
    unimplemented!() // 프로그램이 이 지점에 오게 되면 중단시키는 매크로
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
