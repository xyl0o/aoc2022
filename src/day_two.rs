
pub fn day_two(input: String) {

    let guide : Vec<Game> = input.lines().map(parse_line).collect();
    let points = strategy_guide(&guide);

    println!("Total score of guide: {:?}", points);
}

#[derive(Debug,PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug,PartialEq)]
struct Game {
    opponent: Play,
    player: Play,
}

fn parse_line(line: &str) -> Game {

    let opponent_char = line.chars().nth(0).expect("Invalid game");
    let opponent = match opponent_char {
        'A' => Play::Rock,
        'B' => Play::Paper,
        'C' => Play::Scissors,
        _ => panic!("Invalid char in game"),
    };

    let player_char = line.chars().nth(2).expect("Invalid game");
    let player = match player_char {
        'X' => Play::Rock,
        'Y' => Play::Paper,
        'Z' => Play::Scissors,
        _ => panic!("Invalid char in game"),
    };

    Game {
        opponent,
        player,
    }
}

fn points(game: &Game) -> u32 {
    let shape_points = match game.player {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };

    let game_points = match game {
        Game{opponent: Play::Rock, player: Play::Rock} => 3,
        Game{opponent: Play::Paper, player: Play::Rock} => 0,
        Game{opponent: Play::Scissors, player: Play::Rock} => 6,

        Game{opponent: Play::Rock, player: Play::Paper} => 6,
        Game{opponent: Play::Paper, player: Play::Paper} => 3,
        Game{opponent: Play::Scissors, player: Play::Paper} => 0,

        Game{opponent: Play::Rock, player: Play::Scissors} => 0,
        Game{opponent: Play::Paper, player: Play::Scissors} => 6,
        Game{opponent: Play::Scissors, player: Play::Scissors} => 3,
    };

    shape_points + game_points
}

fn strategy_guide(games: &[Game]) -> u32 {
    games.iter().map(|g| points(g) ).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = parse_line("A X");
        assert_eq!(result, Game{opponent: Play::Rock, player: Play::Rock});

        let result = parse_line("B X");
        assert_eq!(result, Game{opponent: Play::Paper, player: Play::Rock});

        let result = parse_line("C Y");
        assert_eq!(result, Game{opponent: Play::Scissors, player: Play::Paper});

        let result = parse_line("C Z");
        assert_eq!(result, Game{opponent: Play::Scissors, player: Play::Scissors});
    }

    #[test]
    fn test_points() {
        let result = points(&parse_line("A Y"));
        assert_eq!(result, 8);

        let result = points(&parse_line("B X"));
        assert_eq!(result, 1);

        let result = points(&parse_line("C Z"));
        assert_eq!(result, 6);
    }

    #[test]
    fn test_strategy_guide() {
        let guide = [
            parse_line("A Y"),
            parse_line("B X"),
            parse_line("C Z"),
        ];
        let result = strategy_guide(&guide);
        assert_eq!(result, 15);
    }
}
