fn main() {

    // 1. String creations
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    println!("Hello, world!");

    // Strings are utf8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("testing! {}", s);

    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    // s1 is removed!
    let s3 = s1 + &s2;
    println!("zz {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    println!("{}", s2);

    // We can also use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // String is a wrapper over a Vec<u8>
    let len = String::from("Hola").len();
    println!("Hola len {}", len);
    let len = String::from("Здравствуйте").len();
    println!("Rus len {}", len);
}
