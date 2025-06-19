/*operators in Rust 
> < >= <= == != %*/

fn main() {
    /*let age: i8 = 17;
    if age >= 18{
    println!("you can in  {}", age);
    }else{
        println!("you dont in {}", age);
    }
    */
    // logical operators || - or, && - and 
    /*let num = 11 ;
    if num > 10 && num < 50 {
        println!("cool  ");
    }*/ /*else if num < 50  {
        println!("num < 50");
    }*/

    //let name: String = String::from("come");


    /*if name == "yevgen"{
    println!("hi yevgen ");
    }else if name == "jon" {
        println!("hi jon");
    }
    */
    /*if name != "genry"{
        println!("no mach no wait");
    }else if name == "genry" {
        println!("hi ser ");
    }
    */

    //-------------------------------------------------------
    let is_true: bool = true;
    let is_true: bool = false;
     let num = if is_true {
        1
     }else{
        0
     };
     println!("{}", num);

}
