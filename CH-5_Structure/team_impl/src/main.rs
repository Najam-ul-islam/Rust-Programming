#[derive(Debug)]
struct Team{
country:String,
score:u16,
}
impl Team{
    fn high(&self,other:Team)->u16{
        if self.score > other.score{
            self.score
        } 
        else {
            other.score
        }
    }
    fn full(self)->Team{
        self
    }
    fn high_print(&self){
        println!("{}",self.score);
    }
}
fn main() {
    let team1 = Team{
        country:"Pakistan".to_string(),
        score:435,
    };
    let team2 = Team{
        country:"Sri lanka".to_string(),
        score:172,
    };
team1.high_print();
team2.high_print();
//team2.full();
println!("{:#?}",team1.full());
println!("{:#?}",team2.full());
//let hight_team = team1.high(team2);
//    println!("In Main:{}",hight_team);
}
