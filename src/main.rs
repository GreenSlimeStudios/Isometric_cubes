
use macroquad::prelude::*;

const STRANGE_THINGS:bool = false;
const WAVE:bool = true;
const ORIGINAL_ANIMATION:bool = true;

#[macroquad::main("Texture")]
async fn main() {
    //Here declare the x axis and the y axis of the isometric plane
    let xax = 26;
    let yax = 26;

    let mut cubes:Vec<Cube> = Vec::new();
    for i in 0..xax{
        for j in 0..yax{
            cubes.push(Cube::new(i as f32, j as f32));

        }
    }
    let texture: Texture2D = load_texture("assets/cube.png").await.unwrap();
    let mut frame: i128 = 0;
    loop {
        frame += 1;
        clear_background(BLACK);

        for i in 0..xax{
            for j in (0..yax).rev(){
                let x = (cubes[(i*yax+j) as usize].x as f32 * 0.5 * 32.0 + cubes[(i*yax+j) as usize].y as f32 * 0.5 * 32.0) + 0.2 * screen_width();
                let y = (cubes[(i*yax+j) as usize].x as f32 * 0.25 * 32.0 + cubes[(i*yax+j) as usize].y as f32 * -0.25 * 32.0) + 0.5 * screen_height();
                // let y = j as f32 * 32.0;
                draw_texture(
                    texture,
                    x,
                    y,
                    WHITE,
                );
                if WAVE{
                    for k in 0..yax{
                        if i == k{   
                            if frame > k*1{
                                up_down(&mut cubes[(i*yax+j) as usize]);
                            }
                        }
                    }
                }
                //===================================================
                if STRANGE_THINGS {
                    if i % 5 == 0 || j % 3 == 0{   
                        up_down(&mut cubes[(i*yax+j) as usize]);
                    }
                    else if frame > 15 {
                        up_down(&mut cubes[(i*yax+j) as usize]);
                    }
                }
                fn up_down(cube:&mut Cube){
                    if cube.cnt <= 0 {
                        cube.is_up = true;
                    }
                    if cube.cnt > 15 {
                        cube.is_up = false;
                    }
                    if cube.is_up{
                        cube.updatez();
                        cube.cnt += 1
                    }
                    else {
                        cube.updatezm();
                        cube.cnt -= 1;
                    }
                }
                //=================================================

                if ORIGINAL_ANIMATION{
                    let mut cube:&mut Cube = &mut cubes[(i*yax+j) as usize];
                    if frame >= 480 && frame < 520{
                        
                        if i % 5 == 0 || j % 5 ==0{
                            cube.updatex();
                        }
                        else {
                            cube.updatey();
                        }
                    }
                    if frame >= 520 && frame < 560{
                        
                        if i % 5 == 0 || j % 5 ==0{
                            cube.updatez();
                        }
                        else {
                            cube.updatezm();
                        }
                    }
                    if frame >= 560 && frame < 580{
                        
                        if i % 5 == 0 || j % 5 ==0{
                            cube.updatexm();
                        }
                    else {
                        cube.updateym();
                    }
                }
                if frame >= 580 && frame < 600{
                    
                    if i % 5 == 0 || j % 5 ==0{
                        cube.updatezm();
                    }
                    else {
                        cube.updatez();
                    }
                }
                if frame >= 620 && frame < 640{
                    
                    if i % 5 == 0 || j % 5 ==0{
                        cube.updatezm();
                    }
                    else {
                        cube.updatez();
                    }
                }
                if frame >= 660 && frame < 670{
                    
                    if i % 5 == 0 || j % 5 ==0{
                        cube.updatez();
                    }
                    else {
                        cube.updatezm();
                    }
                }
                if frame >= 20 && frame < 80{
                    
                    if i % 2 == 0{
                        
                        cube.updatey();
                    }
                    else {
                        cube.updatex();
                    }
                }
                if frame >= 80 && frame < 140{
                    
                    if j % 2 == 0{
                        
                        cube.updatex();
                    }
                    else {
                        cube.updatey();
                    }
                }
                if frame >= 180 && frame < 240{
                    
                    if j % 2 == 0{
                        
                        cube.updatexm();
                    }
                    else {
                        cube.updateym();
                    }
                }
                if frame >= 180 && frame < 240{
                    
                    if i % 2 == 0{
                        
                        cube.updatexm();
                    }
                    else {
                        cube.updateym();
                    }
                }
                if frame >= 260 && frame < 380{
                    
                    if i % 2 == 0{
                        
                        cube.updatex();
                    }
                    else {
                        cube.updatey();
                    }
                }
                if frame >= 420 && frame < 450{
                    
                    if j % 2 == 0{
                        
                        cube.updatexm();
                    }
                    else {
                        cube.updateym();
                    }
                }
                if frame >= 450 && frame < 480{
                    
                    if j % 2 != 0{
                        
                        cube.updatexm();
                    }
                    else {
                        cube.updateym();
                    }
                }
            }
                
            }
        
        }
        next_frame().await
    }
}
struct Cube{
    x:f32,
    y:f32,
    cnt:i32,
    is_up:bool,
    // id:i32,
}
impl Cube {
    fn new(x:f32,y:f32) -> Self {
        Self{x:x,y:y,cnt:0,is_up:true}
    }
    fn updatey(&mut self){
        // self.x += 0.1;
        self.y += 0.1;
    }
    fn updatex(&mut self){
        // self.x += 0.1;
        self.x += 0.1;
    }
    fn updateym(&mut self){
        // self.x += 0.1;
        self.y -= 0.1;
    }
    fn updatexm(&mut self){
        // self.x += 0.1;
        self.x -= 0.1;
    }
    fn updatez(&mut self){
        self.x -= 0.1;
        self.y += 0.1;
        // self.cnt += 1;
    }
    fn updatezm(&mut self){
        self.x += 0.1;
        self.y -= 0.1;
        // self.cnt -= 1;

    }
}
