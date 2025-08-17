fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("{:?}",vec);
   let ans = even_filter(&vec);
     println!("{:?}",ans);
    
    
}

fn even_filter(vec: &Vec<i32>)->Vec<i32>{
    let mut new_vec = Vec::new();
    for val in vec {
        if val %   2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}
