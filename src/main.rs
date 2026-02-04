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
    crossed_checkpoints: i64,
    fitness: f32,
}

fn mutate(
    checkpoint2: &Rect,
    checkpoint1: &Rect,
    finish: &Rect,
    agents: &mut Vec<Agent>,
    rng: &rand::RandGenerator,
) {
    //calculate the fitness
    for agent in agents.iter_mut() {
        if agent.crossed_checkpoints == 3 {
            agent.fitness = (agent.crossed_checkpoints * 5000) as f32;
        } else if agent.crossed_checkpoints == 2 {
            agent.fitness = (agent.crossed_checkpoints * 1500) as f32
                + (2000.0
                    - agent.pos.distance(Vec2::new(
                        finish.x + finish.w / 2.0,
                        finish.y + finish.h / 2.0,
                    )));
        } else if agent.crossed_checkpoints == 1 {
            agent.fitness = (agent.crossed_checkpoints * 850) as f32
                + (1000.0
                    - agent.pos.distance(Vec2::new(
                        checkpoint1.x + checkpoint1.w / 2.0,
                        checkpoint1.y + checkpoint1.h / 2.0,
                    )));
        } else if agent.crossed_checkpoints == 0 {
            agent.fitness = (agent.crossed_checkpoints * 700) as f32
                + (500.0
                    - agent.pos.distance(Vec2::new(
                        checkpoint2.x + checkpoint2.w / 2.0,
                        checkpoint2.y + checkpoint2.h / 2.0,
                    )));
        }
    }

    //sort the agents, select the best, and mutate
    agents.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    let mut best_dnas = Vec::new();
    for agent in agents.iter_mut().take(50) {
        best_dnas.push(agent.dna.clone());
    }
    for agent in agents.iter_mut().skip(50) {
        agent.dna = best_dnas[rng.gen_range(0, 50)].clone();
        for dna in agent.dna.iter_mut() {
            if rng.gen_range(0.0, 1.0) < 0.01 {
                let temp = Vec2::new(
                    rng.gen_range(0.0, 2.0 * PI).cos(),
                    rng.gen_range(0.0, 2.0 * PI).sin(),
                );
                *dna = temp;
            }
        }
    }

    //reset
    for agent in agents {
        agent.pos = Vec2::new(1300.0, 550.0);
        agent.vel = Vec2::new(0.0, 0.0);
        agent.acc = Vec2::new(0.0, 0.0);
        agent.is_dead = false;
        agent.crossed_checkpoints = 0;
        agent.fitness = 0.0;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    //Basic window config
    set_window_position(0, 0);
    set_window_size(1440, 720);

    let rect = Rect::new(40.0, 32.5, 1360.0, 640.0);
    let rect1 = Rect::new(40.0, 212.5, 1060.0, 15.0);
    let rect2 = Rect::new(340.0, 432.5, 1060.0, 15.0);
    let checkpoint1 = Rect::new(1100.0, 212.5, 285.0, 15.0);
    let checkpoint2 = Rect::new(55.0, 432.5, 285.0, 15.0);
    let finish = Rect::new(140.0, 47.5, 15.0, 165.0);

    let mut agents: Vec<Agent> = Vec::new();
    let rng = rand::RandGenerator::new();
    while agents.len() < 500 {
        let mut dna = Vec::new();

        while dna.len() < 1000 {
            let temp = Vec2::new(
                rng.gen_range(0.0, 2.0 * PI).cos(),
                rng.gen_range(0.0, 2.0 * PI).sin(),
            );
            dna.push(temp);
        }

        let agent = Agent {
            pos: Vec2::new(1300.0, 550.0),
            vel: Vec2::new(0.0, 0.0),
            acc: Vec2::new(0.0, 0.0),
            dna: dna,
            is_dead: false,
            crossed_checkpoints: 0,
            fitness: 0.0,
        };
        agents.push(agent);
    }

    let mut gen_step = 0;
    let mut generation = 1;
    let mut all_dead = false;
    loop {
        clear_background(WHITE);

        if gen_step >= 999 || all_dead {
            gen_step = 0;
            generation += 1;
            mutate(&checkpoint2, &checkpoint1, &finish, &mut agents, &rng);
        }

        //Draw the external rectangle
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 30.0, BLACK);

        //Draw the internal obstacles
        draw_rectangle(rect1.x, rect1.y, rect1.w, rect1.h, BLACK);
        draw_rectangle(rect2.x, rect2.y, rect2.w, rect2.h, BLACK);

        //Draw the checkpoints
        draw_rectangle(
            checkpoint1.x,
            checkpoint1.y,
            checkpoint1.w,
            checkpoint1.h,
            BLUE,
        );
        draw_rectangle(
            checkpoint2.x,
            checkpoint2.y,
            checkpoint2.w,
            checkpoint2.h,
            BLUE,
        );

        //Draw the finish line
        draw_rectangle(finish.x, finish.y, finish.w, finish.h, GREEN);

        all_dead = true;
        //Draw the element(s)
        for agent in &mut agents {
            if !agent.is_dead {
                all_dead = false;
                agent.acc += agent.dna[gen_step];
                agent.vel += agent.acc;
                agent.vel = agent.vel.clamp_length_max(5.0);
                agent.pos += agent.vel;
                agent.acc = Vec2::new(0.0, 0.0);
                if agent.crossed_checkpoints == 0
                    && (checkpoint2.contains(agent.pos + Vec2::new(0.0, 10.0)))
                {
                    agent.crossed_checkpoints = 1;
                } else if agent.crossed_checkpoints == 1
                    && (checkpoint1.contains(agent.pos + Vec2::new(0.0, 10.0)))
                {
                    agent.crossed_checkpoints = 2;
                } else if agent.crossed_checkpoints == 2
                    && (finish.contains(agent.pos + Vec2::new(-10.0, 0.0)))
                {
                    agent.crossed_checkpoints = 3;
                }
            }
            draw_circle(agent.pos.x, agent.pos.y, 10.0, RED);
            if !agent.is_dead
                && ((!rect.contains(agent.pos + Vec2::new(25.0, 0.0))
                    || !rect.contains(agent.pos + Vec2::new(-25.0, 0.0))
                    || !rect.contains(agent.pos + Vec2::new(0.0, 25.0))
                    || !rect.contains(agent.pos + Vec2::new(0.0, -25.0)))
                    || (rect1.contains(agent.pos + Vec2::new(-10.0, 0.0))
                        || rect1.contains(agent.pos + Vec2::new(0.0, 10.0))
                        || rect1.contains(agent.pos + Vec2::new(0.0, -10.0)))
                    || (rect2.contains(agent.pos + Vec2::new(10.0, 0.0))
                        || rect2.contains(agent.pos + Vec2::new(0.0, 10.0))
                        || rect2.contains(agent.pos + Vec2::new(0.0, -10.0))))
            {
                agent.is_dead = true;
            }
        }

        gen_step += 1;

        //Display stats
        draw_text(
            ("Generation ".to_owned() + &generation.to_string()).as_str(),
            50.0,
            25.0,
            40.0,
            BLACK,
        );

        draw_text(
            ("Step ".to_owned() + &gen_step.to_string()).as_str(),
            300.0,
            25.0,
            40.0,
            BLACK,
        );

        next_frame().await
    }
}
