pub fn run(){
/*
     let message1="hello";
     let message2="welcome";
     greeting1(message1);
     greeting2(message1,message2);

     fn greeting1(name: &str){
        println!("{}",name);
     }
     fn greeting2(name1:&str,name2:&str){
        greeting1(name1);
        greeting1(name2);

     }
     
     let power=increment_power(4);
     println!("{}",power);
     
     let x=abs(0);
     println!("{}",x);
     */
     println!("the cubed value is {}",cube(3));
}
/*fn increment_power(power: i32)->i32{

    println!("my power is going to increase");
    power+1
}*/

/*fn abs(x:i32)->i32{
    if x>0{
        x
    }
    else{
        x
    }
}*/

fn cube(x:i32)->i32{
    x*x*x
}
