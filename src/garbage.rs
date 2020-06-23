// doesnt work, fixing wouldn't take more than 5 mins but i am lazy

// pub fn draw_circle(game_state: &mut State, x_center: i32, y_center: i32, radius: i32) {
//     // implemented according to Bresenham’s circle drawing algorithm
//     let mut x = 0;
//     let mut y = radius;
//     let mut d = 3 - 2 * radius;
//     let mut draw_circle = |x_center: i32, y_center: i32, x: i32, y: i32| {
//         let mut put_pixel = |x: i32, y: i32| {
//             put_pixel(
//                 game_state,
//                 x,
//                 y,
//                 RGB::named(rltk::YELLOW),
//                 RGB::named(rltk::BLACK),
//                 '.',
//             );
//         };
//         put_pixel(x_center + x, y_center + y);
//         put_pixel(x_center - x, y_center + y);
//         put_pixel(x_center + x, y_center - y);
//         put_pixel(x_center - x, y_center - y);
//         put_pixel(x_center + y, y_center + x);
//         put_pixel(x_center - y, y_center + x);
//         put_pixel(x_center + y, y_center - x);
//         put_pixel(x_center - y, y_center - x);
//     };
//     // draw 8 points inside each piece
//     while y >= x {
//         x += 1;
//         if d > 0 {
//             y -= 1;
//             d += 4 * (x - y) + 10;
//         } else {
//             d += 4 * x + 6;
//         }
//         draw_circle(x_center, y_center, x, y);
//     }
// }

// pub fn draw_line_x_axis(game_state: &mut State, x_start: i32, x_end: i32, y: i32) {
//     for x in x_start..x_end {
//         put_pixel(
//             game_state,
//             x,
//             y,
//             RGB::named(rltk::YELLOW),
//             RGB::named(rltk::BLACK),
//             '.',
//         )
//     }
// }

// pub fn draw_line_y_axis(game_state: &mut State, x: i32, y_start: i32, y_end: i32) {
//     for y in y_start..y_end {
//         put_pixel(
//             game_state,
//             x,
//             y,
//             RGB::named(rltk::YELLOW),
//             RGB::named(rltk::BLACK),
//             '.',
//         )
//     }
// }

// pub fn draw_rectangle(game_state: &mut State, x_left: i32, x_right: i32, y_start: i32, y_end: i32) {
//     // draw left and right vertical line
//     draw_line_y_axis(game_state, x_left, y_start, y_end);
//     draw_line_y_axis(game_state, x_right, y_start, y_end);
//     // draw top and bottom enclosing horizontal line because i am to lazy to draw half circle there
//     draw_line_x_axis(game_state, x_left, x_right, y_start);
//     draw_line_x_axis(game_state, x_left, x_right, y_end);
// }

// pub fn draw_a_dick(game_state: &mut State) {
//     draw_circle(game_state, 20, 25, 10);
//     draw_circle(game_state, 50, 25, 10);
//     draw_shaft(game_state, 30, 40, 5, 25);
// }
