use std::time::SystemTime;
use std::time::Duration;
use std::thread::sleep;
use std::io::stdin;
use std::thread::spawn;
use std::io::Stdin;
use std::sync::Mutex;
use std::sync::Arc;

const COAT_SIZE: i32=64;

struct Game{
pub ball: f64,
pub speed: f64,
}

impl Game{
    pub fn new()->Self{
        Self{ball:0.0, speed: 0.01}
    }

    pub fn update(&mut self,is_swing: &Mutex<bool>)->bool{
        self.ball+=self.speed;
        let mut is_swing=is_swing.lock().unwrap();
        if *is_swing{
            self.speed*=-1.0;
        }
        *is_swing=false;

        if 0.0<=self.ball&&self.ball<=1.0{
            return true;
        }
        false
    }
}

fn draw(ball: f64){
    let ball:i32=(COAT_SIZE as f64*ball).round() as i32;
    let mut buf=String::from(" ");
    buf+="|";
    for i in 0..COAT_SIZE{
        buf+=if i==ball{"o"}else{" "};
    }
    buf+="|";
    println!("\x1B[1;1H{}", buf);//上書き
}
fn game_loop(game:&mut Game, is_swing: &Mutex<bool>){
let mut time=SystemTime::now();
loop{
    if!game.update(is_swing){
        break;
    }
    draw(game.ball);
    time+=Duration::from_nanos(16_666_667);
    if let Ok(dur)=time.duration_since(SystemTime::now()){
        sleep(dur);
    }

}
println!("Game Over");
}

fn sub_main(is_swing:&Mutex<bool>)->!{
    let input: Stdin = stdin();
    let mut buf=String::new(); 
    loop {
        input.read_line(&mut buf).unwrap();
        *is_swing.lock().unwrap()=true;

        buf.clear();
    }
}


fn main() {
    println!("\x1B[2J");
    let is_swing:Arc<Mutex<bool>>=Default::default();
    {
        let is_swing:Arc<Mutex<bool>>=is_swing.clone();
        spawn(move||sub_main(& is_swing));
    }

    let mut game=Game::new();
    game_loop(&mut game,& is_swing);

}
