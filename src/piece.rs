use std::ops::BitXor;

use crate::board;
use crate::game::Player;
use crate::engine::js;

// TODO: Testing for move validation
//     * Can't test EVERY move case, but can at least test a valid and invalid test

#[path = "test/piece_test.rs"] mod test;

fn validate_rook(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize) -> bool
{
	// We can only move in X or Y so if we moved in both then return false
	// This also catches neither, but that's not a move so would be invalid anyway!
	if ! (piece_x.abs_diff(move_x) != 0).bitxor( piece_y.abs_diff(move_y) != 0 )
	{
		return false
	}
	// TODO: Check if the move is unblocked (but not the last square)

	// For each square between start & end - non-inclusive (in a straight line)

	let mut range_x = piece_x..piece_x+1;
	let mut range_y = piece_y..piece_y+1;

	match piece_x.cmp(&move_x)
	{
	    std::cmp::Ordering::Less => // Moving right
	    {
	    	range_x = (piece_x+1)..move_x;
	    },
	    std::cmp::Ordering::Greater => // Moving left
	    {
	    	range_x = (move_x+1)..piece_x;
	    },
	    std::cmp::Ordering::Equal => 
	    {
	    	match piece_y.cmp(&move_y)
	    	{
		        std::cmp::Ordering::Less => // Moving down
		        {
		        	range_y = (piece_y+1)..move_y;
		        },
		        std::cmp::Ordering::Greater => // Moving up
		        {
		        	range_y = (move_y+1)..piece_y;
		        },
		        std::cmp::Ordering::Equal =>
		        {
		        	return false
		        },
		    }
	    }
	}
	for x in range_x
	{
		for y in range_y.clone()
		{
			if board[y][x] != ' '
			{
				console_log!("Invalid move, square {} is occupied", board::get_square_name_from_usize(x, y));
				return false
			}
		}
	}	
	true
}
fn validate_bishop(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize) -> bool
{
	// We have to have moved the same amount on the X & Y axis
	if piece_x.abs_diff(move_x) != piece_y.abs_diff(move_y)
	{
		return false
	}
	// TODO: Check if the move is unblocked (but not the last square)
	false
}

/// Validate whether a move is possible using the board's current state, which square we're moving from, and which square we're moving to.  
/// This function needs the most recent move for validating en passant
pub fn validate_move(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize, most_recent_move: &str) -> bool
{
	// We've already confirmed that we're not actively moving ONTO a friendly piece
	// So now we just need to make sure we're moving in a legit way & not through a friend
	let piece = board[piece_y][piece_x];

	console_log!("Validating {piece} from {piece_x}{piece_y} to {move_x}{move_y}");
	match piece
	{
		'♖' | '♜' => // rook
		{
			// Unblocked lefty righty upy downy
			validate_rook(board, piece_x, piece_y, move_x, move_y)
		},
		'♞' | '♘' => // knight
		{
			// Make sure we're moving 2 on one axis, and one on the other
			(
				(piece_x.abs_diff(move_x) == 2 && piece_y.abs_diff(move_y) == 1)
				||
				(piece_x.abs_diff(move_x) == 1 && piece_y.abs_diff(move_y) == 2)
			)
			&& get_colour(piece) != get_colour(board[move_y][move_x])
		},
		'♗' | '♝' => // bishhop
		{
			// Unblocked X
			validate_bishop(board, piece_x, piece_y, move_x, move_y)
		},
		'♕' | '♛' => // queen
		{
			// Unblocked X+
			// rook + bishop
			validate_rook(board, piece_x, piece_y, move_x, move_y) 
			|| // or
			validate_bishop(board, piece_x, piece_y, move_x, move_y)
		},
		'♔' | '♚' => // king
		{
			piece_x.abs_diff(move_x) <= 1// if our change in X is 0 or 1
			&& // and
			piece_y.abs_diff(move_y) <= 1// our change in Y is 0 or 1
			// Then we've moved one space (and allowed diagonals)
		},
		'♙' | '♟' => // pawn
		{
			validate_pawn(board, piece_x, piece_y, move_x, move_y, most_recent_move)
		},
		_ => { false } // uhhhh what are you trying to move???
	}
}

pub fn get_colour(piece: char) -> Option<Player>
{
	let piece_num = u32::from(piece);
	if (9812..=9817).contains(&piece_num)
	{
		return Some(Player::Black)
	}
	// Else
	if (9818..=9823).contains(&piece_num)
	{
		return Some(Player::White)
	}
	// Else
	None
}


fn validate_pawn(board: [[char; 8]; 8], piece_x: usize, piece_y: usize, move_x: usize, move_y: usize, most_recent_move: &str) -> bool
{
	let piece = board[piece_y][piece_x];
	let taking = board[move_y][move_x];
	
	// If black is going up or white is going down
	if ( piece == '♙' && ( piece_y > move_y ) )
	|| ( piece == '♟' && ( piece_y < move_y ) )
	{
		return false // that's bad
	}

	/* ~ ~ ~ Checking regular movement ~ ~ ~ */
	
	// No lefty-righty please
	if piece_x.abs_diff(move_x) == 0
	// And where we're going MUST be free
	&& ( taking == ' ' )
	{
		match piece_y.abs_diff(move_y)
		{
			1 => 
			{	// If we're only moving 1 and the space is free, then we're happy :)
				return true 
			},
			2 => 
			{
				// Get the inbetween space's Y coord
				let inbetween_y = match get_colour(piece)
				{
					Some(Player::White) => {move_y + 1},
					Some(Player::Black) => {move_y - 1},
					None => {0} // ????
				};
				// If the inbetween space was free
				if board[inbetween_y][move_x] == ' '
				{
					return true
				}
				// Else it was blocked
				console_log!("Invalid move, square {} is occupied", board::get_square_name_from_usize(move_x, inbetween_y) );
				return false
			},
			_ =>
			{	// If we're trying to move more than 2 (or 0) then that's baaad
				return false 
			},
		}
	}

	/* ~ ~ ~ Checking taking opponents pieces ~ ~ ~ */

	// We have to be taking *something*
	if taking == ' '
	{
		return false
	}

	// A single move diagonal (into an opponent's piece)
	if (piece_x.abs_diff(move_x) == 1)
	&& (piece_y.abs_diff(move_y) == 1)
	&& (get_colour(piece) != get_colour(taking))
	{
		return true
	}

	// En passant

	// Was the most recent move a pawn?
	if ! most_recent_move.starts_with(['♙', '♟'])
	{
		return false // Because if not, no en passant thank you
	}

	// TODO: En passant!?!
	// TODO: Check if most_recent_move was a pawn moving 2 spaces
	
	// TODO: We're going to need to pass in the most recent move because en passant requires that the pawn being taken literally JUST did a two square move
	todo!()
}
