
/*CYCLES 
loop - just in rust cycle unended cycle
while - 

*/


fn main() {
//---------LOOP CYCLE---------------------------------------------------------
/* 
            let mut num = 0;
    loop {
       
       // println!("LOOP............"); //FOR EXIT Ctrl+C
       // if -like requirements that say stop for loop when reqs correct
      
        println!("{}", num);

           // num = num +1;
            num +=1; // +=1,2,3,4,5,.......
 if num == 100 {
        break;
       }
    
    }

*/

//-------WHILE CYCLE-----------------------------------------------
//let mut num =0;

/*while num <=500 {
    num +=1; // if num +=1; paste before println! - we hawe +1 to our num in while cycles
    println!("{}", num);
   // 
}*/

//------------the % operations with numbers
/*while num <= 100 {
    num +=1; // if num +=1; paste before println! - we hawe +1 to our num in while cycles
    println!("{}", num);
   }
 */

   // example -------%-------------------
   /*  let n1 = 18 % 6;
    let n2 = 18 % 4;
    println!("{},{}", n1, n2);
       */

      /*  let mut num =0;
       while num <=100 {
           if num % 30 == 0{
            println!("{}", num);
           }
           num +=1;
       }
    */

 // this case worked without "if" just when type "num = 0" 
 //because them num of type and sum num of iterate are summ 
   /*  let mut num = 0;
    while num <=100 {
        println!("{}", num);
        num += 2
    }
    */

    //-------FOR CYCLE------------------------------

     for i in 0..101 { // here is >=0 and > 100 
        if i % 2 ==0{
                println!("{}", i);
        }
         
     }


}
