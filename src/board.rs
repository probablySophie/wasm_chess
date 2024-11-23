
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn get_square_name(x: i32, y: i32) -> String
{
	let letter = ('A'..='H').nth(x as usize).unwrap();
	let num = 8-y;
	letter.to_string() + &num.to_string()
}

pub fn unselect_all(chessboard: &web_sys::Element)
{
	let selected = chessboard.get_elements_by_class_name("selected");

	// For each element with the class selected
	for i in 0..selected.length()
	{
		// Make sure we actually have a valid element...
		let Some(current) = selected.get_with_index(i)
		else { continue };

		// ... and remove the class selected
		current.set_class_name( &current.class_name().replace("selected", "") );
	}
}
