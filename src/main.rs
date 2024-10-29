fn main() {
    let mut acc1=BankAccount{
        account_number:21,
        holder_name:"ravi".to_string(),
        balance:200000.0,
    };
    let mut acc2=BankAccount{
        account_number:23,
        holder_name:"Pravin".to_string(),
        balance:250000.0,
    };
acc1.deposit(50000.0);
acc2.withdraw(50000.0);
println!("the bank balance of account1 after operation  is {}",acc1.balance());
println!("the bank balance of account2 after operation is {}",acc2.balance());


}
trait Account{
    fn deposit( &mut self,bal:f64);
    fn withdraw( &mut self,bal:f64);
    fn balance(&mut self)->f64;
}

struct BankAccount{
    account_number:u8,
    holder_name:String,
    balance:f64,
}

impl Account for BankAccount{
    fn deposit(&mut self,bal:f64){
    self.balance+=bal
    }
    fn withdraw(&mut self,bal:f64){
    self.balance-=bal
        }
    fn balance (&mut self)->f64{
     self.balance
            }
}