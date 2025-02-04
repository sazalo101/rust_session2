struct User{
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}
fn main (){
    let user1 =User {
        email: String::from("sam@gmail.com"),
        username: String::from("sazal"),
        active: true,
        sign_in_count: 1,
    };
    println!("the {}is for {} and is online :{},and has signed in{}",user1.email,user1.username,user1.active,user1.sign_in_count);
}