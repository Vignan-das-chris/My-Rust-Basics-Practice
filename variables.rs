//comments type-1 /*comment type-2*/
pub fn run(){

static MAX_HEALTH: i32=100;
static GAME_NAME: &'static str="Valorant";


    println!("the name of game is {}",GAME_NAME);
    println!("the otherone is {}",MAX_HEALTH);
    println!("IN the game {1} you have {0}% health yes you  read it correct{1}points",GAME_NAME,MAX_HEALTH);
    println!("the answer of the value is{1} {1}{0}",2*2,2*3);

    //special ways for formatting 

    
    println!("the hexadecimal value of  max health is {:x}",MAX_HEALTH);
    println!("the octal value of  max health is {:o}",MAX_HEALTH);
    println!("the binary  value of  max health is {:b}",MAX_HEALTH);
    println!("the exponentail lower value of  max health is {:e}",MAX_HEALTH);
    println!("the hexadecimal value of  max health is {:E}",MAX_HEALTH);
    println!("the debigging thing of  max health is {:?}",MAX_HEALTH);

    //varibles binding to values

    let gene=5;

    let copy_gene=gene;
    let name="Hari das";

    println!("the value of energy{} of {}",gene,name);
    println!("the value of copy {}",copy_gene);
    

    //mutability and Immutability

     let mut dartsize=10;
     let n: i32=7;

     dartsize=20;
     println!("the dart size {}{}",dartsize,n);

    
     // scope of the variables

     let outside=100;
     {
        let inside=30;
        println!("inside the scope{}",inside);
        let outside=99;//shadowed by the first outer variable
        println!("outside in innerspace{}",outside);
     }
     println!("outside in outerspace{}",outside);
     
 
   //type checking and conversions.
   
   let he="Mr.fox";
   let she="Miss.rose";
   let they= he.to_string()+she;
   let them=format!("{}{}",she,he);
   println!("{}{}",they,them);

   let value1:i32=2;
   let value2: f32=3.14;
   let value3= value1+ value2 as i32;//explicit conversion
   println!("{}",value3);


//expressions
 let n1={
    let a=2;
    let b=3;
    a+b  //without semicolon
 };
 println!("{:?}",n1);

    let n2={
        let a=3;
        let b=3;
        a+b; // return value of this code block is supressed
    };
    println!("{:?}",n2);


    //memory allocation
  
   let health=35;
   let mut game="valorant";
   println!(" the address location is {:p}",&health);
   println!("the address location is {:p}",&game);

   let game2=&game;
   println!("the address of game2 is {}",game2);



}
