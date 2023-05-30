use std::{io, collections::HashMap};

fn main() {
    println!("Hi today i have for you some special task, first i ask u what text u would like to print out!");
    let mut word_from_user = String::new();
    io::stdin()
        .read_line(&mut word_from_user)
        .expect("Failed to read line");
    
     let mut alphabet_map: HashMap<char, &str> = HashMap::new();
     alphabet_map.insert('A', A);
     alphabet_map.insert('B', B);
     alphabet_map.insert('C', C);
     alphabet_map.insert('D', D);
     alphabet_map.insert('E', E);
     alphabet_map.insert('F', F);
     alphabet_map.insert('G',G);
     alphabet_map.insert('H', H);
     alphabet_map.insert('I', I);
     alphabet_map.insert('J', J);
     alphabet_map.insert('K', K);
     alphabet_map.insert('L', L);
     alphabet_map.insert('M', M);
     alphabet_map.insert('N', N);
     alphabet_map.insert('O', O);
     alphabet_map.insert('P', P);
     alphabet_map.insert('Q', Q);
     alphabet_map.insert('R', R);
     alphabet_map.insert('S', S);
     alphabet_map.insert('T', T);
     alphabet_map.insert('U', U);
     alphabet_map.insert('V', V);
     alphabet_map.insert('X', X);
     alphabet_map.insert('Y', Y);
     alphabet_map.insert('Z',Z);


     let word_for_print = word_from_user
     .chars()
     .take(word_from_user.len() - 1)
     .filter_map(|c| alphabet_map.get(&c.to_ascii_uppercase()).cloned())
     .collect::<Vec<&str>>()
     .join("\n");


    println!("This is your beloved word\n{}", word_for_print);
}



const A: &str = "
  ##
 ####   
##  ##  
######  
##  ##  
##  ##  
##  ##";

const B: &str = "
### ##   
 ##  ##  
 ##  ##  
 ## ##   
 ##  ##  
 ##  ##  
### ##";

const C: &str = "
 ## ##   
##   ##  
##       
##       
##       
##   ##  
 ## ##";

const D: &str = "
### ##   
 ##  ##  
 ##  ##  
 ##  ##  
 ##  ##  
 ##  ##  
### ##";

const E: &str = "
### ###  
 ##  ##  
 ##      
 ## ##   
 ##      
 ##  ##  
### ###";

const F: &str = "
### ###  
 ##  ##  
 ##      
 ## ##   
 ##      
 ##      
####";

const G: &str = "
 ## ##   
##   ##  
##       
##  ###  
##   ##  
##   ##  
 ## ##";

 const H: &str = "
 ##  ##  
 ##  ##  
 ##  ##  
 ######  
 ##  ##  
 ##  ##  
 ##  ##";

 const I: &str = "
####   
 ##    
 ##    
 ##    
 ##    
 ##    
####";

const J: &str = "
   ####  
    ##   
    ##   
    ##   
##  ##   
##  ##   
 ## #";

const K: &str = "
##  ###  
##  ##   
## ##    
## ##    
## ###   
##  ##   
##  ###";

const L: &str = "
####     
 ##      
 ##      
 ##      
 ##      
 ##  ##  
### ###";

const M: &str = "
##   ## 
### ### 
####### 
## # ## 
##   ## 
##   ## 
##   ##";

const N: &str = "
##  ##  
### ##  
######  
######  
## ###  
##  ##  
##  ##";

const O: &str = "
 ## ##   
##   ##  
##   ##  
##   ##  
##   ##  
##   ##  
 ## ##";

const P: &str = "
### ##   
 ##  ##  
 ##  ##  
 ##  ##  
 ## ##   
 ##      
####";

const Q: &str = "
 ####   
##  ##  
##  ##  
##  ##  
##  ##  
 ####   
   ###";

const R: &str = "
#####   
##  ##  
##  ##  
#####   
####    
## ##   
##  ##";

const S: &str = "
  ####   
 ##  ##  
 ##      
  ####   
     ##  
 ##  ##  
  ####";

const T: &str = "
######  
  ##    
  ##    
  ##    
  ##    
  ##    
  ##";

const U: &str = "
##  ##  
##  ##  
##  ##  
##  ##  
##  ##  
##  ##  
 #### ";

const V: &str = "
##  ##  
##  ##  
##  ##  
##  ##  
##  ##  
 ####   
  ##";

const X: &str = "
##  ##  
##  ##  
 ####   
  ##    
 ####   
##  ##  
##  ##";

const Y: &str = "
##  ##  
##  ##  
##  ##  
 ####   
  ##    
  ##    
  ##";

const Z: &str = "
######  
    ##  
   ##   
  ##    
 ##     
##      
######";

