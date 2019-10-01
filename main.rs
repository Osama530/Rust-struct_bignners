//make program to calculate area of a rectangle using struct
#[derive(Debug)]
// struct Rectangle{
//     width : u64,
//     hight : u64,
// }

// fn area(call_values:&Rectangle)->u64 {
//     call_values.width*call_values.hight
// }

// fn main (){
//     let area_rect = Rectangle {width:50,hight:10};
//     println!("area rectangel is {}",area(&area_rect));
// }

//by using impl (implimentation block) or method
struct Rectangle {
    hight : u32,
    width : u32,
}

impl Rectangle {
    fn area(&self)->u32 {
        self.hight *self.width
    }
}

fn main () {
    let rect_1 = Rectangle{width:50, hight:10};
    let result = rect_1.area();
    println!("area of rect_1 is : {}",result);

    let rect_2 = Rectangle{width:30, hight:10};
    let resul = rect_2.area();
    println!("area of rect_2 is : {}",resul);
}