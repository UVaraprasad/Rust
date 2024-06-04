fn main() {
     let s=String::from("prasad123akil678pavan111");
     let k=reve(&s);
     println!("{}",k);
}

fn reve(s:&String)->String{
    let mut rmsg=String::new();
    let mut v=Vec::new();
    let mut r=Vec::new();
    let mut flag=0;
    for i in s.chars(){
        if i.is_alphabetic(){
             v.push(i);
             flag=1; 
        }
        else if flag!=0 {
            v.reverse();  
            r.append(&mut v);
            v.clear(); 
            flag=0; 
        }
    }
    v.reverse();
    r.append(&mut v);
    v.clear();
    println!("{:?}",r);
   
     let mut indl=0;
    for c in s.chars(){
        if c.is_alphabetic(){
            rmsg.push(r[indl]);
            indl+=1;
        }
        else {
            rmsg.push(c);
        } 
    }
    return rmsg;

}

/*  Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message
use std::io;
fn main() {
    println!("Try programiz.pro");
    let mut s=String::new();
    let _=io::stdin().read_line(&mut s);
  // let c= s.pop();
  // print!("{:?}",c);
   //let c= s.pop();
   //print!("{:?}",c);
    println!("{}",s.len());
    println!("{}",s);
    
}*/