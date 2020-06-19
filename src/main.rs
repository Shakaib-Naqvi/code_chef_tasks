extern crate text_io;
use text_io::read;

// ------------------------------------Task 1: Leap Year----------------------------------


fn main(){   
    println!("Task 1: Leap Year");
    loop{
    println!("Enter Year: ");
    let year:i32 = read!();
    if year % 400 == 0 || (year % 4 == 0) & (year % 100 != 0){
    println!("{} is a leap year.",year ) ;
    }
    else{
    println!("{} is not a leap year.",year ) ;
    }
    }
}


// ------------------------------------Task 3: Some Simple Math----------------------------------


// fn main(){ 
//     println!("Task 3: Some Simple Math");
//     let ludo_squares = 92; 
//     let mut coins:u128 = 1; 
//     let mut a:u128 = 1; 
//     println!("On 1 box the gold coin will: 1"); 
//     loop{ 
//         coins+=coins;
//          a+=1;
//         println!("On {} boxes the gold coins will: {}",a,coins);
//          if a == ludo_squares{
//               break; 
//                 } 
//         else{
//         }
//      }
//  }

// ------------------------------------Task 4: Beer bottles----------------------------------



// fn main(){     
//     println!("Task 4: Beer bottles");     
//     let mut bottles = 100;     
//     let mut bottles_1 = 99;      
//     println!("\n\t\t\t\tSong:\n");     
//     loop{         
//         bottles-=1;         
//         bottles_1-=1;         
//         println!("{} bottles of beer on the wall,{} bottles of beer.",bottles,bottles);         
//         println!("Take one down and pass it around, {} bottles of beer on the wall.\n",bottles_1);                   
//         if bottles_1 == 1{             
//             println!("1 bottle of beer on the wall,1 bottle of beer.");             
//             println!("Take it down and pass it around, no more bottles of beer on the wall.\n");             
//             println!("No more bottles of beer on the wall, no more bottles of beer.");             
//             println!("Go to the store and buy some more, 99 bottles of beer on the wall.\n");             
//         break;         
//         }     
//     } 
// }


// ------------------------------------Task 5: Fun with math and strings----------------------------------



// fn is_armstrong_number(num: u32) -> String {         
//     let len = ((num as f32).log(10.) as u32) + 1;         
//     let mut sum = 0;         
//     let mut i = num;         
//     while i != 0 {             
//         sum += (i % 10).pow(len);             
//         i /= 10;         
//     }         
//         if sum == num{             
//             println!("{} is an armstrong number",num);             
//             "".to_string()         
//         }         
//         else{             
//             println!("{} is not an armstrong number",num);             
//             "".to_string()         
//         }     
//         }     
// fn main(){         
//     println!("Task 5: Fun with math and strings");         
//     println!("Enter a number: ");         
//     let _numberr = is_armstrong_number(read!()); 
// }


// ------------------------------------Task 7: When to take Medicine----------------------------------



// fn main(){     
//     println!("Task 7: When to take Medicine");     
//     println!("Enter the Date: ");     
//     let date:i32 = read!();     
//     if  date >= 32 || date == 0{         
//         println!("Enter a valid date!!!");     
//     }     
//     else if date % 2 == 0{         
//         println!("Take A Medicine Today.");     
//     }     
//     else {         
//         println!("There is no time to take medicine.");     
//     } 
// }