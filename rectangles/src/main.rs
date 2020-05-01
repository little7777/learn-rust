// 原始方法,width, height1没有相关性

fn main1() {
    let width1 = 30;
    let height1 =50;
    println!(
        "the area of the rectange is {} square pixels",
        area1(width1, height1)
    );
}

fn area1(width: u32, height: u32) ->u32 {
    width * height
}

// 使用tuple实现, 无法得知哪个值是长,哪个值是宽, 只能通过记住index位置

fn main2() {
    let rect1 = (30,20);
    println!(
        "the area of the rectange is {} ",
        area2(rect1)
    )
}

fn area2(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

// 使用struct, 赋予变量意义
#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectange {width: 10, height: 15};
    println!(
        "the area of rectange is {} square pixels.",
        area(&rect1)
    );
    println!(
        "rect1 is {:#?}", rect1
    );
}

fn area(rectange: &Rectange) -> u32 {
    rectange.width * rectange.height
}



