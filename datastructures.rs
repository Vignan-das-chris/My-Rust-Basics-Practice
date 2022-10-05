use std::io;

// STRINGS -----1

//literal strings
pub fn run(){
    
let magician1="david";
let greeting="hello";

let magician2: &'static str="blaine";//string is staticly allocated
println!("magicain {} greets magician{} with a {}",magician1,magician2,greeting);
 // strings
let mut str1=String::new();//an empty string is created

let mut str2=String::with_capacity(25);

let mut str3=magician1.to_string();//add string to string

let s11=&str1;//a string slice of str3
let s12=&str3;
println!("{:?} this is slice of {:?}",s11,str3);
if s12 == magician1{
    println!("we got the same magician");

}

let c1='c';
str1.push(c1);//pushing a character to the string
println!("{}",str1);

str1.push_str("level one is finished");//pushing a string to a string 
println!("{}",str1);

//to print all characters in a string

for n in magician1.chars(){
    println!("{}  ",n);
}
// to separate strings by its white spaces

for v in str1.split(""){
    print!(" {} * ",v);
}

let str4=str1.replace("level","floor");//to replace the string with anothor string
print!("{} {}",str4,str1);

fn how_long(s: &str)->usize{s.len()}//the length of the string
print!("{}",how_long("hell"));




//ARRAYS

let bts:[&str;7]=["taheyung","jimin","junkook","J-hope","jin","namjoon","suga"];
println!("{:?}",bts);
print!("{}",bts.len());//length of the array

print!("{} last elemt ",bts.iter().last().unwrap());//return the last element

//if we want to print succesive array
for ix in 0..bts.len(){
    print!(" {} {} ",ix,bts[ix]);
}
//to print the next item
for a in bts.iter(){
    println!("the next bts member is {}",a);
}

  
 //VECTORS
 let mut numbers:Vec<i32> = Vec::new();

 let mut magic_numbers=vec![3,4,5,6,7];

 //SLICES
 let slc=&magic_numbers[1..4];
 for b in slc.iter(){
 print!("{}",b);}

 
 //TUPLES

 let thor=("thor",true,3500u32);
 let (god,strength)=increase_power(thor.0,thor.2);
 println!("the god{} has power{}",god,strength);


fn increase_power(name:&str,power:u32,)->(&str,u32){
    if power>1000{
        return (name,power * 3);
    }
    else{
        return(name,power*2);
    }
}


//STRUCTS
struct Player{
    nname:&'static str,
    health:i32,
    level:u8,
}
let mut p11=Player{
    nname:"ronaldo",health:100 ,level:10
};
println!("{}{}{}",p11.nname,p11.level,p11.health);

//getting data from console

let mut buf=String::new();
io::stdin().read_line(&mut buf).ok().expect("failed");
print!("{}",buf);

}