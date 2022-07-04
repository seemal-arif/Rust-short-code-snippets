use std::io::stdin;


pub fn fibonacci(x:i32)->(i32,i32){
    if x==1 {         // if 1th term
        return (x,0);
    }
    else if x==2 {   // if 2nd term
        return (x,1); 
    }
    else if x<0 {
        return (x,-1);
    }
    else{ //
        let mut a:i32=0;
        let mut b:i32=0;
        let mut c:i32=1;
        let mut count:i32=x ;
       while count>2 {
        a=b;
        b=c;
        count=count-1;
        c=a+b;
       }
       return (x,c);
    }
}
pub fn main(){
    let mut s = String::new();
    println!("Enter number :  ");
    stdin().read_line(&mut s).unwrap();
    let mut number=0;
    match s.trim_end().parse::<i32>() {
        Ok(i) => number= i,
        Err(_) => number=-1,
    }
    let (x,y):(i32,i32)=fibonacci(number);
   println!("Fibonacci of {}th term is {} ",x,y);

}