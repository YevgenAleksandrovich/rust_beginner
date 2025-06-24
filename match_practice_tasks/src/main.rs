//practice for work with - match
/* 
fn main() {
    
    //the primitive number -----------------------
    
    /*let x =5;
    match x {
        2 => println!(" one"),
        4 => println!("four"),
        _ => println!("none other"),
    }
    */

    //the String --------------------------------

        let lang = "rust";
        match lang {
            "rust" => println!("🦀"),
            "python" => println!("🐍"),
            _ => println!("🤷"),
        }

        

//



let mut krok = 9;

match krok {
    5 => println!("five"),
    7 => println!("seven "),
    0..= 15 => {
        println!("good, very hot");
    },
    _ => println!("try more - no one"),
}

//match with score -------------------------------

    let score = 85;
        match score {
            90..=100 => println!("score is 79 "),
            79..=89 => println!("yes score inside here distance "),
             _ => println!("none of above "),
        }
    


}

*/


//return num-------------------------------------
/* 
fn grade(score: u8) -> &'static str{
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        _ => "F",
    }
} 

fn main (){

        println!("Grade: {}", grade(88));

}
        */






       /*  fn grade (score: u8) -> &'static str {
            
            match score {
                50..=100 => "n",
                60..=120 => "f",
                70..=140 => "g",
                80..=160 => "z",
                _ => "lol",
            }

        } 

        fn main(){
            println!("grade : {}", grade(101) );
        }
*/



/*fn level_of_score(score: u8 ) -> &'static  str{
    match score {
        20..= 30 => "a",
        30..= 40 => "n",
        40..= 50 => "b",
        50..= 60 => "f",
         _   => "none",
  }
}
fn main(){
    println!("our level for score : {}", level_of_score(48));
}
*/

//----option---------------------------
/*fn main(){
    let maybe_num    = Some(87);
    match maybe_num{
        Some(n) => println!("nuber : {}", n),
        None => println!("no number"),
    }
} 
 */

 //----------Result-------------------------



fn divide(x: i32, y: i32 ) -> Result<i32, String> {

        if y == 0{
            Err("cannot divide by zero ".to_string())
        }else {
            Ok(x / y)
        }
}


fn main(){
    match divide(16,2){
        Ok(result) => println!("result : {}", result),
        Err(e) => println!("error : {}", e),
    } 

}




