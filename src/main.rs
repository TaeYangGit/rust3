#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // 읽은 바이트 수를 반환
    let mut tmp = f.data.clone(); // data 복사본 Vec<T>가 줄어드므로 만든다.
    let read_length = tmp.len();
    save_to.reserve(read_length); // 데이터 저장 공간이 충분한지 확인
    save_to.append(&mut tmp); // f의 내용을 담기 위해 save_to 버퍼에 충분한 데이터를 할당한다.
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer); // Vec<u8>을 String으로 변환한다.

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text) // 바이트를 실제 단어로 표시한다.
}
