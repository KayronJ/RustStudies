
#[derive(Debug)]
enum Profession{
   Analyst,
   Devoloper,
   CyberSecurity,
}

struct Employer{
   name: String,
   salary: u32,
   profession: Profession,
}
fn main(){

   let employer1 = Employer{
      name: "Fabio".to_string(),
      salary: 6000,
      profession: Profession::CyberSecurity,
   };

      println!("Employer Name: {:?}", employer1.name);
      println!("Employer Salary: {:?}", employer1.salary);
      println!("Employer Profession: {:?}", employer1.profession);
      
}