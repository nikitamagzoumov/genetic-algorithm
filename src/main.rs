use std::f32::consts::PI;

use macroquad::{
    miniquad::window::{set_window_position, set_window_size},
    prelude::*,
};

struct Agent {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    dna: Vec<Vec2>,
    is_dead: bool,
}

fn mutate(agents: &mut Vec<Agent>) {
    //reset the positions
    //calculate the fitness
    //select the best
    //mutate
}

#[macroquad::main("MyGame")]
async fn main() {
    //Basic window config
    let x_pos = -1930i32 as u32;
    set_window_position(x_pos, 0);
    set_window_size(1920, 1050);

    let rect = Rect::new(40.0, 32.5, 1860.0, 960.0);
    let rect1 = Rect::new(40.0, 312.5, 1360.0, 15.0);
    let rect2 = Rect::new(500.0, 632.5, 1400.0, 15.0);

    let mut agents: Vec<Agent> = Vec::new();
    let rng = rand::RandGenerator::new();
    while agents.len() < 500 {
        let mut dna = Vec::new();

        while dna.len() < 500 {
            let temp = Vec2::new(
                rng.gen_range(0.0, 2.0 * PI).cos(),
                rng.gen_range(0.0, 2.0 * PI).sin(),
            );
            dna.push(temp);
        }

        let agent = Agent {
            pos: Vec2::new(1800.0, 820.0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            dna: dna,
            is_dead: false,
        };
        agents.push(agent);
    }

    let mut gen_step = 0;
    let mut generation = 1;
    loop {
        clear_background(WHITE);

        if gen_step >= 499 {
            gen_step = 0;
            generation += 1;
            mutate(&mut agents);
        }

        //Draw the external rectangle
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 30.0, BLACK);

        //Draw the internal obstacles
        draw_rectangle(rect1.x, rect1.y, rect1.w, rect1.h, BLACK);
        draw_rectangle(rect2.x, rect2.y, rect2.w, rect2.h, BLACK);

        //Draw the checkpoints
        draw_line(1400.0, 320.0, 1885.0, 320.0, 10.0, BLUE);
        draw_line(55.0, 640.0, 500.0, 640.0, 10.0, BLUE);

        //Draw the finish line
        draw_line(140.0, 47.5, 140.0, 312.5, 20.0, GREEN);
        //Draw the element(s)
        for agent in &mut agents {
            if !agent.is_dead {
                agent.acc += agent.dna[gen_step];
                agent.vel += agent.acc;
                agent.vel = agent.vel.clamp_length_max(5.0);
                agent.pos += agent.vel;
                agent.acc = Vec2::new(0.0, 0.0);
            }
            draw_circle(agent.pos.x, agent.pos.y, 10.0, RED);
            if !agent.is_dead
                && ((!rect.contains(agent.pos + Vec2::new(25.0, 0.0))
                    || !rect.contains(agent.pos + Vec2::new(-25.0, 0.0))
                    || !rect.contains(agent.pos + Vec2::new(0.0, 25.0))
                    || !rect.contains(agent.pos + Vec2::new(0.0, -25.0)))
                    || (rect1.contains(agent.pos + Vec2::new(10.0, 0.0))
                        || rect1.contains(agent.pos + Vec2::new(-10.0, 0.0))
                        || rect1.contains(agent.pos + Vec2::new(0.0, 10.0))
                        || rect1.contains(agent.pos + Vec2::new(0.0, -10.0)))
                    || (rect2.contains(agent.pos + Vec2::new(10.0, 0.0))
                        || rect2.contains(agent.pos + Vec2::new(-10.0, 0.0))
                        || rect2.contains(agent.pos + Vec2::new(0.0, 10.0))
                        || rect2.contains(agent.pos + Vec2::new(0.0, -10.0))))
            {
                agent.is_dead = true;
            }
        }

        gen_step += 1;

        //Display current generation's number
        draw_text(
            ("Generation ".to_owned() + &generation.to_string()).as_str(),
            20.0,
            25.0,
            40.0,
            BLACK,
        );

        next_frame().await
    }
}
