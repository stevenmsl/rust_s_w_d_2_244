use rust_s_w_d_2_244::Solution;
fn main() {
    let sn = Solution::from(&Solution::test_fixrure());

    let s = sn.shortest(&String::from("coding"), &String::from("makes"));

    //let s = sn.shortest(&String::from("practice"), &String::from("coding"));

    println!("{:?}", s);
}
