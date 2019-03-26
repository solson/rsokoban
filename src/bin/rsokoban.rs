extern crate clap;
extern crate rsokoban;
extern crate xmltree;

use clap::{Arg, App, AppSettings};
use rsokoban::{GameState, Action};
use rsokoban::board::{Board, Cell};
use std::fs::File;
use xmltree::Element;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("LEVEL_PACK_FILE")
             .required(true)
             .help("The level pack to load"))
        .arg(Arg::with_name("LEVEL_INDEX")
             .required(true)
             .help("The index of the level within the level pack to play"))
        .get_matches();

    let level_pack_file = matches.value_of_os("LEVEL_PACK_FILE").unwrap();
    let level_index = matches.value_of("LEVEL_INDEX").unwrap().parse::<usize>().unwrap();
    let level_pack = Element::parse(File::open(level_pack_file).unwrap()).unwrap();

    let levels: Vec<Board<Cell>> = level_pack
        .get_child("LevelCollection")
        .unwrap()
        .children
        .iter()
        .map(|level_elem| {
            assert_eq!(level_elem.name, "Level");
            let width  = level_elem.attributes.get("Width").unwrap().parse::<usize>().unwrap();
            let height = level_elem.attributes.get("Height").unwrap().parse::<usize>().unwrap();
            let mut board = Board::new(width, height);

            if level_elem.children.len() > height {
                panic!("level has more rows than specified by the Height attribute");
            }

            for (row, line_elem) in level_elem.children.iter().enumerate() {
                assert_eq!(line_elem.name, "L");
                let text = line_elem.text.as_ref().unwrap().as_bytes();

                if text.len() > width {
                    panic!("level has more rows than specified by the Height attribute");
                }

                for (col, &byte) in text.iter().enumerate() {
                    let cell = match byte {
                        b'#' => Cell::Wall,
                        b'@' => Cell::Player,
                        b'+' => Cell::PlayerOnGoal,
                        b'$' => Cell::Box,
                        b'*' => Cell::BoxOnGoal,
                        b'.' => Cell::Goal,
                        b' ' => Cell::Floor,
                        _    => panic!("unexpected character in level: {}", byte as char),
                    };
                    board[row][col] = cell;
                }
            }

            board
        })
        .collect();

    levels[level_index].dump_ascii();

    // let mut game = GameState::new();

    // loop {
    //     match game.handle_input() {
    //         Action::Stop => break,
    //         Action::None => {},
    //     }

    //     game.update();
    //     game.render();
    // }
}
