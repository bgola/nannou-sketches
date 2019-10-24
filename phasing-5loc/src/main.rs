extern crate nannou;
extern crate nannou_osc;

use nannou::prelude::*;
use nannou_osc as osc;
use nannou_osc::Type::Float as F;

fn main() {
  nannou::app(m).run(); // sketch(view);
}

struct M {
  s: osc::Sender<osc::Connected>,
}

fn m(a: &App) -> M {
  a.new_window().view(v).build().unwrap();
  a.main_window().set_inner_size_points(1920.0*0.333, 1080.0*0.333);
  let s = osc::sender().unwrap().connect("127.0.0.1:57120").unwrap();
  M { s }
}

fn v(a: &App, m: &M, f: &Frame) {
  // setup variables
  let d=a.draw();let r=a.window_rect();let t=a.time*7.;
  //(a.mouse.y/r.h()+0.5)*9.;

  // actual code
  d.background().rgb(0.,0.,0.);for i in 0..(r.w()/50.) as i32+1{let x=(i*(50. as
  i32)) as f32;let s=(t+t*(i as f32)*0.002).sin();if s>0.92{d.rect().x_y(x-r.w()
  /2., 0.).w_h(50., r.h());if s>0.991{m.s.send(("/",vec![F(x/r.w())])).ok();}}};

  // nannou stuff
  d.to_frame(a, &f).unwrap();
}
