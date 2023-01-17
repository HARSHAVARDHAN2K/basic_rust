enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center
}
pub fn main() {
     let player_direction = Direction::Center;
     match player_direction {
        Direction::Up => println!("it's up"),
        Direction::Down => println!("it's Down"),
        Direction::Left => println!("it's Left"),
        Direction::Right => println!("it's Right"),
        _=> println!("invalid")

}

}

