use anyhow::Error;
use text_io::try_read;

#[derive(Debug)]
struct Frame {
    start: usize,
    end: usize,
    score: Option<usize>,
}

#[derive(Debug)]
struct Game {
    rolls: Vec<usize>,
    frames: Vec<Frame>,
    score: usize,
}

fn main() {
    lets_go_bowling();
}

fn lets_go_bowling() -> Game {
    let mut game: Game = Game {
        rolls: vec![],
        frames: vec![],
        score: 0,
    };

    for frame_ix in 0..10 {
        // game_loop: init variables for each frame
        let mut frame_sum = 0;
        let roll_start = game.rolls.len();
        // for roll_ix in 0..2
        loop {
            if frame_ix < 9 && frame_sum >= 10 || game.rolls.len() >= roll_start + 2 {
                break;
            }
            if frame_ix == 9 { /* TODO */ }

            // frame loop: get roll input
            print!(
                "Frame={}, Roll={}, Enter knocked down pins: ",
                frame_ix + 1,
                game.rolls.len() - roll_start + 1
            );
            let roll_rd: Result<usize, _> = try_read!();
            match roll_rd {
                Ok(roll) => {
                    if roll + frame_sum > 10 {
                        println!(
                            "Error: invalid roll, please enter a value between 0 and {}",
                            10 - frame_sum
                        );
                        continue;
                    }
                    // frame_loop: add roll to state variables
                    frame_sum += roll;
                    game.rolls.push(roll);
                }
                _ => {
                    println!(
                        "Error: invalid roll, please enter a value between 0 and {}",
                        10 - frame_sum
                    );
                    continue;
                }
            }
        }
        let roll_end = game.rolls.len();

        let score = if frame_sum < 10 {
            Some(frame_sum)
        } else {
            None
        };
        game.frames.push(Frame {
            start: roll_start,
            end: roll_end,
            score,
        });

        update_scores(&mut game);
        display_game(&game);
    } // end game_loop

    game // return game variable
}

fn update_scores(g: &mut Game) {
    for ix in 0..g.frames.len() {
        // check frame for None score
        let f: &mut Frame = g.frames.get_mut(ix).unwrap();
        if let Some(_val) = f.score {
            continue;
        }

        // for sanity, let's assert strike/spare
        assert!(g.rolls[f.start..f.end].iter().sum::<usize>() == 10);

        // handle updateable spare case
        if 2==f.end-f.start && g.rolls.len() >= f.end+1 {
            f.score = Some(10 + g.rolls[f.end+0]);
        }
        // handle updateable strike case
        else if 1==f.end-f.start && g.rolls.len() >= f.end+2 {
            f.score = Some(10 + g.rolls[f.end+0] + g.rolls[f.end+1]);
        }
        println!("TEST: frame = {f:?}");
    }

    // update game score
    let mut gscore = 0;
    for ix in 0..g.frames.len() {
        if let Some(fscore) = g.frames[ix].score {
            gscore += fscore;
        }
    }
    g.score = gscore;
}

fn display_game(g: &Game) {
    println!("");
    for (fix, f) in g.frames.iter().enumerate() {
        println!("{} : {f:?}", fix + 1);
    }
    println!("Rolls = {:?}", g.rolls);
    println!("Score = {}", g.score);
    println!("");
    // println!("g = {g:?}");
}
