// // fn factorial(n: i32, result: i32) -> i32{

// //     if n == 0 {
// //        return result;
// //     };

// //     let res: i32 = n*result;

// //     factorial(n-1, res)
// // }

// // // formula (n!/n-1!)^n


fn concat(arr: Vec<String>) -> i32 {
    return arr.join("").parse::<i32>().unwrap();
 } 

fn permute(n: i32, arr:Vec<i32>, mut perm: Vec<i32>, base: [i32;5], iter: i32) -> Vec<i32> {

        if iter == 1 {
            return perm;
        }
    
        let mut new_arr: Vec<i32> = Vec::new();
    
        for i in &arr {

            if *i != n {
                
                let num: i32 = concat(vec![n.to_string(), i.to_string()]);

                if num % 3 == 0 {
                    println!("✅ {} ✅", num);
                }
                else{
                    println!("❌ {} ❌", num);
                }

                if !perm.contains(&num) {
                    perm.push(num);
                }

            }
    
            for j in &base {

                if i != j && *j != n && *i != n {

                    let string_num: String = j.to_string();

                    if i.to_string().contains(&string_num) == false {

                        new_arr.push(concat(vec![i.to_string(), j.to_string()]));

                    }
            
                }
            }
        }
        
        //println!("{:?}", new_arr);
        return permute(n, new_arr, perm, base, iter-1);
}
 
fn main(){

    let arr: [i32;5] = [4,7,6,3,1];
    let mut permutations: Vec<i32> = arr.clone().to_vec();


    for i in arr{
        if i % 3 == 0 {
            println!("✅ {} ✅", i);
        }
        else{
            println!("❌ {} ❌", i);
        }
        let mut p: Vec<i32> = permute(i, arr.clone().to_vec(), Vec::new(), arr.clone(), arr.len().try_into().unwrap());
        permutations.append(&mut p);
    }

    println!("{:?} | length: {}", permutations, permutations.len());

}
 


