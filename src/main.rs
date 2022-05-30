
use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {
    let xax = 16;
    let yax =16;

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
                let x = (cubes[i*yax+j].x as f32 * 0.5 * 32.0 + cubes[i*yax+j].y as f32 * 0.5 * 32.0) + 0.2 * screen_width();
                let y = (cubes[i*yax+j].x as f32 * 0.25 * 32.0 + cubes[i*yax+j].y as f32 * -0.25 * 32.0) + 0.5 * screen_height();
                // let y = j as f32 * 32.0;
                draw_texture(
                    texture,
                    x,
                    y,
                    WHITE,
                );
                if frame >= 480 && frame < 520{

                    if i % 5 == 0 || j % 5 ==0{
                        cubes[i*yax+j].updatex();
                    }
                    else {
                        cubes[i*yax+j].updatey();
                    }
                }
                if frame >= 520 && frame < 560{

                    if i % 5 == 0 || j % 5 ==0{
                        cubes[i*yax+j].updatez();
                    }
                    else {
                        cubes[i*yax+j].updatezm();
                    }
                }
                if frame >= 560 && frame < 580{

                    if i % 5 == 0 || j % 5 ==0{
                        cubes[i*yax+j].updatexm();
                    }
                    else {
                        cubes[i*yax+j].updateym();
                    }
                }
                if frame >= 580 && frame < 600{

                    if i % 5 == 0 || j % 5 ==0{
                        cubes[i*yax+j].updatezm();
                    }
                    else {
                        cubes[i*yax+j].updatez();
                    }
                }
                if frame >= 620 && frame < 640{

                    if i % 5 == 0 || j % 5 ==0{
                        cubes[i*yax+j].updatezm();
                    }
                    else {
                        cubes[i*yax+j].updatez();
                    }
                }
                if frame >= 660 && frame < 670{

                    if i % 5 == 0 || j % 5 ==0{
                        cubes[i*yax+j].updatez();
                    }
                    else {
                        cubes[i*yax+j].updatezm();
                    }
                }
                if frame >= 20 && frame < 80{

                    if i % 2 == 0{
                        
                        cubes[i*yax+j].updatey();
                    }
                    else {
                        cubes[i*yax+j].updatex();
                    }
                }
                if frame >= 80 && frame < 140{

                    if j % 2 == 0{
                        
                        cubes[i*yax+j].updatex();
                    }
                    else {
                        cubes[i*yax+j].updatey();
                    }
                }
                if frame >= 180 && frame < 240{

                    if j % 2 == 0{
                        
                        cubes[i*yax+j].updatexm();
                    }
                    else {
                        cubes[i*yax+j].updateym();
                    }
                }
                if frame >= 180 && frame < 240{

                    if i % 2 == 0{
                        
                        cubes[i*yax+j].updatexm();
                    }
                    else {
                        cubes[i*yax+j].updateym();
                    }
                }
                if frame >= 260 && frame < 380{

                    if i % 2 == 0{
                        
                        cubes[i*yax+j].updatex();
                    }
                    else {
                        cubes[i*yax+j].updatey();
                    }
                }
                if frame >= 420 && frame < 450{

                    if j % 2 == 0{
                        
                        cubes[i*yax+j].updatexm();
                    }
                    else {
                        cubes[i*yax+j].updateym();
                    }
                }
                if frame >= 450 && frame < 480{

                    if j % 2 != 0{
                        
                        cubes[i*yax+j].updatexm();
                    }
                    else {
                        cubes[i*yax+j].updateym();
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
    
}
impl Cube {
    fn new(x:f32,y:f32) -> Self {
        Self{x:x,y:y}
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
    }
    fn updatezm(&mut self){
        self.x += 0.1;
        self.y -= 0.1;
    }
}
