use rand::{thread_rng, Rng};
macro_rules! calc {
    ($key:ident) => {
        match $key % 4 {
            0 => 4,
            n => n,
        }
    };
}
fn 轮<'a>() -> (&'a str, &'a str) {
    // 大衍之數五十
    let 大衍之数 = 50;

    // 其用四十有九
    let 用 = 大衍之数 - 1; // 1 为 不易

    // 分而為二以象兩
    let mut 易 = thread_rng();
    let 左 = 易.gen_range(1..用); // 在天成象
    let 右 = 用 - 左; // 地随天变

    // 掛一以象三
    let 手 = 1; // 1 为 人
    let 右 = 右 - 手; // 人从地来，现有天地人三才

    let 手 = 手 + calc!(左); // 揲之以四 以象四時，歸奇於扐 以象閏
    let 手 = 手 + calc!(右); // 五歲再閏，故再扐而後掛

    // 三遍成爻（第二遍）
    let 余 = 用 - 手;
    let 左 = 易.gen_range(1..余);
    let 右 = 余 - 左 - 1;
    let 手 = 手 + 1 + calc!(左) + calc!(右);
    //（第三遍）
    let 余 = 用 - 手;
    let 左 = 易.gen_range(1..余);
    let 右 = 余 - 左 - 1;
    match (用 - (手 + 1 + calc!(左) + calc!(右))) / 4 {
        6 /* 老阴 */ => ("阴", "阳"),
        7 /* 少阳 */ => ("阳", "阳"),
        8 /* 少阴 */ => ("阴", "阴"),
        9 /* 老阳 */ => ("阳", "阴"),
        _ => panic!("internal error")
    }
}
fn main() {
    let 卦: Vec<_> = (1..=6).map(|_| 轮()).collect();
    println!("本:   变:");
    卦.into_iter().rev().for_each(|(爻, 变)| println!("{爻}    {变}"));
}