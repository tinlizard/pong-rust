use macroquad::{color, miniquad::window::set_window_size, prelude::*};

struct CollisionRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color_paddle: color::Color,
    offset: f32,
    rect: CollisionRect,
    score: i32,
}

struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    color: color::Color,
    offset_x: f32,
    offset_y: f32,
    rect: CollisionRect,
}

impl CollisionRect {
    fn intersects(&self, other: &CollisionRect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
}

impl Ball {
    fn move_ball(&mut self){
        self.y += self.offset_y;
        self.x -= self.offset_x;
        self.rect.y += self.offset_y;
        self.rect.x -= self.offset_x;
    }
    
    fn reverse_direction(&mut self){
        if self.y > 600.0 {
            self.offset_y = -self.offset_y;
        }
        if self.y < 0.0 {
            self.offset_y = -self.offset_y;
        }
    }
    fn check_offscreen(&mut self, paddle1: &mut Paddle, paddle2: &mut Paddle){
        if self.x < 0.0  {
            self.x = screen_width() / 2.0;
            self.y = screen_height() / 2.0;
            self.rect.x = screen_width() / 2.0;
            self.rect.y = screen_height() / 2.0;
            self.offset_y = 2.0;
            paddle2.score += 1;
        } 
        else if self.x > 800.0 {
            self.x = screen_width() / 2.0;
            self.y = screen_height() / 2.0;
            self.rect.x = screen_width() / 2.0;
            self.rect.y = screen_height() / 2.0;
            self.offset_y = 2.0;
            paddle1.score += 1;
        }
    }
    fn check_paddle_collision(&mut self, paddle_rect: &CollisionRect) {
        if self.rect.intersects(paddle_rect) {
            self.offset_x = -self.offset_x; // Reverse ball's horizontal direction
        }
    }
}

impl Paddle {
    fn move_left_rect(&mut self){
        if is_key_down(KeyCode::W){
            self.y -= self.offset; 
            self.rect.y -= self.offset;
        }
        else if is_key_down(KeyCode::S){
            self.y += self.offset;
            self.rect.y += self.offset;
        }
    }
    fn move_right_rect(&mut self){
        if is_key_down(KeyCode::Up){
            self.y -= self.offset; 
            self.rect.y -= self.offset;
        }
        else if is_key_down(KeyCode::Down){
            self.y += self.offset;
            self.rect.y += self.offset;
        }
    }
}


#[macroquad::main("Pong")]
async fn main() {
    set_window_size(800, 600);
    let mut paddle_left = Paddle {
        x: 0.0,
        y: 100.0,
        width: 20.0,
        height: 120.0,
        color_paddle: WHITE,
        offset: 2.0,
        rect: CollisionRect {x: 0.0, y: 100.0, width: 20.0, height:120.0},
        score: 0,
    };
    let mut ball = Ball {
        x: screen_width()/2.0,
        y: screen_height()/2.0,
        radius: 10.0,
        color: WHITE,
        offset_x: 2.0,
        offset_y: 2.0,
        rect: CollisionRect {x: screen_width()/2.0,y:screen_height()/2.0,width:10.0,height:10.0},
    };
    let mut paddle_right = Paddle {
        x: 780.0,
        y: 100.0,
        width: 20.0,
        height: 120.0,
        color_paddle: WHITE,
        offset: 2.0,
        rect: CollisionRect {x: 780.0, y: 100.0, width: 20.0, height:120.0},
        score: 0,
    };
    loop {
        clear_background(BLACK);
        draw_rectangle(paddle_left.x,paddle_left.y,paddle_left.width,paddle_left.height,paddle_left.color_paddle);
        draw_rectangle(paddle_right.x, paddle_right.y, paddle_right.width, paddle_right.height, paddle_right.color_paddle);
        paddle_left.move_left_rect();
        paddle_right.move_right_rect();
        draw_circle(ball.x,ball.y,ball.radius,ball.color);
        ball.move_ball();
        ball.check_paddle_collision(&paddle_left.rect);
        ball.check_paddle_collision(&paddle_right.rect);
        ball.reverse_direction();
        ball.check_offscreen(&mut paddle_left, &mut paddle_right);
        draw_text_ex(&paddle_left.score.to_string(),20.0,30.0,TextParams::default());
        draw_text_ex(&paddle_right.score.to_string(),780.0,20.0,TextParams::default());
        
        next_frame().await
    }
}