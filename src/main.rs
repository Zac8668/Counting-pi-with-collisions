use macroquad::prelude::*;

struct Block {
    color: Color,
    mass: f64,
    vel: f64,
    size: i32,
    x: f64,
}

impl Block {
    fn update_block(&mut self) {
        self.x += self.vel * get_frame_time() as f64 * 1.00;
    }
}

fn draw_block(block: &Block) {
    draw_rectangle(
        block.x as f32,
        screen_height() * 0.8,
        block.size as f32,
        block.size as f32,
        block.color,
    );
}

fn check_collision(block1: &Block, block2: &Block) -> bool {
    block1.x + block1.size as f64 >= block2.x
}

fn collision(block1: &Block, block2: &Block) -> (f64, f64) {
    let vel1 = block1.vel;
    let vel2 = block2.vel;

    let new_vel1 = (block1.mass - block2.mass) / (block1.mass + block2.mass) * vel1
        + (2.00 * block2.mass) / (block1.mass + block2.mass) * vel2;

    let new_vel2 = (2.00 * block1.mass) / (block1.mass + block2.mass) * vel1
        + (block2.mass - block1.mass) / (block1.mass + block2.mass) * vel2;

    (new_vel1, new_vel2)
}

#[macroquad::main("CalculatingPi")]
async fn main() {
    let digits: i32 = 3;
    let acc: i32 = 50;
    let mut collisions: i32 = 0;
    let mut done: bool = false;

    let mut block1 = Block {
        color: RED,
        mass: 1.00,
        vel: 0.00,
        size: 50,
        x: 100.00,
    };

    let mut block2 = Block {
        color: BLUE,
        mass: 100_i64.pow((digits - 1) as u32) as f64,
        vel: -20.00,
        size: 50,
        x: 400.00,
    };

    loop {
        clear_background(DARKGRAY);
        for _n in 0..acc {
            block1.update_block();
            block2.update_block();

            if check_collision(&block1, &block2) {
                let new_vel = collision(&block1, &block2);
                block1.vel = new_vel.0;
                block2.vel = new_vel.1;
                collisions += 1;
            };

            if block1.x <= 20.00 {
                block1.vel *= -1.00;
                collisions += 1;
            };
        }

        if block2.x < block1.size as f64 + 20.00 {
            block2.x = block1.size as f64 + 20.00;
        }

        if block1.vel > 0.00 && block2.vel > 0.00 && block2.vel > block1.vel {
            done = true;
        }
        draw_block(&block1);
        draw_block(&block2);
        draw_text(&("Collisions: ".to_string() + &collisions.to_string()), 10.0, 50.0, 60.0, GRAY);
        draw_text(&("Fps: ".to_string() + &get_fps().to_string()), 10.0, 100.0, 60.0, GRAY);
        if done {
            draw_text("Done!", 10.0, 150.0, 60.0, GRAY);
        }

        next_frame().await
    }
}
