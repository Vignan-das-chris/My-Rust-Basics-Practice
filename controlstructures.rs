//Branching on condition

pub fn run(){

       /*let dead=false;
       let health=8;

       if dead{
        println!("GAme over");
        return;
       }
       if dead{
        println!("Game over");
        return;
       }
       else{
        println!("continue");
       }
       if health>=50 {
        println!("continue to score");
       }
       else if health>=20 {
            println!("stop battle gain ");
        }
        else{
            println!("just die");
        }
        */

        //Looping 

    let max_power=10;
    let mut power=1;

    while power<max_power{
        print!("{}",power);
        power+=1
    }

    loop{
        power += 1;
        print!("{}",power);
        if power==20{
            println!("thats enough");
            break;
          
        }
    }
    for n in 20..2{
        print!("{}",n);
    }
       
}