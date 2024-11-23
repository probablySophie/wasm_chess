
#[test]
fn test_colour_check()
{
    use crate::{game::Player, piece::get_colour};

	assert!(get_colour(' ').is_none());
	assert!(get_colour('♜') == Some(Player::White)); 
	assert!(get_colour('♞') == Some(Player::White)); 
	assert!(get_colour('♝') == Some(Player::White)); 
	assert!(get_colour('♛') == Some(Player::White)); 
	assert!(get_colour('♚') == Some(Player::White)); 
	assert!(get_colour('♟') == Some(Player::White));
	assert!(get_colour('♖') == Some(Player::Black)); 
	assert!(get_colour('♘') == Some(Player::Black)); 
	assert!(get_colour('♗') == Some(Player::Black)); 
	assert!(get_colour('♕') == Some(Player::Black)); 
	assert!(get_colour('♔') == Some(Player::Black)); 
	assert!(get_colour('♙') == Some(Player::Black));
}
