extern crate gilrs;
extern crate cgmath;

use cgmath::Vector2;

use gilrs::{Gilrs, Button, Axis, Event};

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const ESCAPE: u32 = 1;
#[cfg(target_os = "macos")]
pub const ESCAPE: u32 = 53;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const A: u32 = 30;
#[cfg(target_os = "macos")]
pub const A: u32 = 0;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const B: u32 = 48;
#[cfg(target_os = "macos")]
pub const B: u32 = 11;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const C: u32 = 46;
#[cfg(target_os = "macos")]
pub const C: u32 = 8;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const D: u32 = 32;
#[cfg(target_os = "macos")]
pub const D: u32 = 2;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const E: u32 = 18;
#[cfg(target_os = "macos")]
pub const E: u32 = 14;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F: u32 = 33;
#[cfg(target_os = "macos")]
pub const F: u32 = 3;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const G: u32 = 34;
#[cfg(target_os = "macos")]
pub const G: u32 = 5;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const H: u32 = 35;
#[cfg(target_os = "macos")]
pub const H: u32 = 4;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const I: u32 = 23;
#[cfg(target_os = "macos")]
pub const I: u32 = 34;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const J: u32 = 36;
#[cfg(target_os = "macos")]
pub const J: u32 = 38;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const K: u32 = 37;
#[cfg(target_os = "macos")]
pub const K: u32 = 40;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const L: u32 = 38;
#[cfg(target_os = "macos")]
pub const L: u32 = 37;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const M: u32 = 50;
#[cfg(target_os = "macos")]
pub const M: u32 = 45;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const N: u32 = 49;
#[cfg(target_os = "macos")]
pub const N: u32 = 45;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const O: u32 = 24;
#[cfg(target_os = "macos")]
pub const O: u32 = 31;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const P: u32 = 25;
#[cfg(target_os = "macos")]
pub const P: u32 = 35;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const Q: u32 = 16;
#[cfg(target_os = "macos")]
pub const Q: u32 = 12;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const R: u32 = 19;
#[cfg(target_os = "macos")]
pub const R: u32 = 15;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const S: u32 = 31;
#[cfg(target_os = "macos")]
pub const S: u32 = 1;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const T: u32 = 20;
#[cfg(target_os = "macos")]
pub const T: u32 = 17;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const U: u32 = 22;
#[cfg(target_os = "macos")]
pub const U: u32 = 32;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const V: u32 = 47;
#[cfg(target_os = "macos")]
pub const V: u32 = 9;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const W: u32 = 17;
#[cfg(target_os = "macos")]
pub const W: u32 = 13;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const X: u32 = 45;
#[cfg(target_os = "macos")]
pub const X: u32 = 7;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const Y: u32 = 21;
#[cfg(target_os = "macos")]
pub const Y: u32 = 16;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const Z: u32 = 44;
#[cfg(target_os = "macos")]
pub const Z: u32 = 6;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const SPACE: u32 = 57;
#[cfg(target_os = "macos")]
pub const SPACE: u32 = 49;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F1: u32 = 59;
#[cfg(target_os = "macos")]
pub const F1: u32 = 120;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F2: u32 = 60;
#[cfg(target_os = "macos")]
pub const F2: u32 = 122;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F3: u32 = 61;
#[cfg(target_os = "macos")]
pub const F3: u32 = 99;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F4: u32 = 62;
#[cfg(target_os = "macos")]
pub const F4: u32 = 118;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F5: u32 = 63;
#[cfg(target_os = "macos")]
pub const F5: u32 = 96;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F6: u32 = 64;
#[cfg(target_os = "macos")]
pub const F6: u32 = 97;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F7: u32 = 65;
#[cfg(target_os = "macos")]
pub const F7: u32 = 98;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F8: u32 = 66;
#[cfg(target_os = "macos")]
pub const F8: u32 = 100;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F9: u32 = 67;
#[cfg(target_os = "macos")]
pub const F9: u32 = 101;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F10: u32 = 68;
#[cfg(target_os = "macos")]
pub const F10: u32 = 109;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F11: u32 = 87;
#[cfg(target_os = "macos")]
pub const F11: u32 = 0; // TO BE UPDATE

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const F12: u32 = 88;
#[cfg(target_os = "macos")]
pub const F12: u32 = 0; // TO BE UPDATE

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const TAB: u32 = 15;
#[cfg(target_os = "macos")]
pub const TAB: u32 = 48;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const ONE: u32 = 2;
#[cfg(target_os = "macos")]
pub const ONE: u32 = 18;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const TWO: u32 = 3;
#[cfg(target_os = "macos")]
pub const TWO: u32 = 19;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const THREE: u32 = 4;
#[cfg(target_os = "macos")]
pub const THREE: u32 = 20;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const FOUR: u32 = 5;
#[cfg(target_os = "macos")]
pub const FOUR: u32 = 21;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const FIVE: u32 = 6;
#[cfg(target_os = "macos")]
pub const FIVE: u32 = 23;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const SIX: u32 = 7;
#[cfg(target_os = "macos")]
pub const SIX: u32 = 22;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const SEVEN: u32 = 8;
#[cfg(target_os = "macos")]
pub const SEVEN: u32 = 26;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const EIGHT: u32 = 9;
#[cfg(target_os = "macos")]
pub const EIGHT: u32 = 28;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const NINE: u32 = 10;
#[cfg(target_os = "macos")]
pub const NINE: u32 = 25;

#[cfg(all(not(target_os = "android"), not(target_os = "macos"), not(target_os = "windows")))]
pub const LEFT: u32 = 105;
#[cfg(target_os = "macos")]
pub const LEFT: u32 = 123;
#[cfg(target_os = "windows")]
pub const LEFT: u32 = 75;

#[cfg(all(not(target_os = "android"), not(target_os = "macos"), not(target_os = "windows")))]
pub const RIGHT: u32 = 106;
#[cfg(target_os = "macos")]
pub const RIGHT: u32 = 124;
#[cfg(target_os = "windows")]
pub const RIGHT: u32 = 77;

#[cfg(all(not(target_os = "android"), not(target_os = "macos"), not(target_os = "windows")))]
pub const UP: u32 = 103;
#[cfg(target_os = "macos")]
pub const UP: u32 = 125;
#[cfg(target_os = "windows")]
pub const UP: u32 = 72;

#[cfg(all(not(target_os = "android"), not(target_os = "macos"), not(target_os = "windows")))]
pub const DOWN: u32 = 108;
#[cfg(target_os = "macos")]
pub const DOWN: u32 = 125;
#[cfg(target_os = "windows")]
pub const DOWN: u32 = 72;


//macos
//escape                37
//a                     0
//b                     11
//c                     8
//d                     2
//e                     14
//f                     3
//g                     5
//h                     4
//i                     34
//j                     38
//k                     40
//l                     37
//m                     46
//n                     45
//o                     31
//p                     35
//q                     12
//r                     15
//s                     1
//t                     17
//u                     32
//v                     9
//w                     13
//x                     7
//y                     16
//z                     6
//F1                    122
//F2                    120
//F3                    99
//F4                    118
//F5                    96
//F6                    97
//F7                    98
//F8                    100
//F9                    101
//F10                   109
//tild                  50
//tab                   48
//caps lock             None
//left shift            56
//right shift           60
//left ctrl             55
//right ctrl            54
//left alt/option key   58
//right alt/option key  61
//space bar             49
//enter                 36
//comma                 43
//dot                   47
//foward slash          44
//semi colon            41
//quotation marks       39
//square bracket open   33
//square bracket close  30
//back slash            42
//1                     18
//2                     19
//3                     20
//4                     21
//5                     23
//6                     22
//7                     26
//8                     28
//9                     25
//0                     29
//minus sign            27
//plus sign             24
//Back space            51
//arrow up              126
//arrow left            123
//arrow right           124
//arrow down            125

//Linux and most of windows
//escape                1
//a                     30
//b                     48
//c                     46
//d                     32
//e                     18
//f                     33
//g                     34
//h                     35
//i                     23
//j                     36
//k                     37
//l                     38
//m                     50
//n                     49
//o                     24
//p                     25
//q                     16
//r                     19
//s                     31
//t                     20
//u                     22
//v                     47
//w                     17
//x                     45
//y                     21
//z                     44
//F1                    59
//F2                    60
//F3                    61
//F4                    62
//F5                    63
//F6                    64
//F7                    65
//F8                    66
//F9                    67
//F10                   68
//F11                   87
//F12                   88
//tild                  41
//tab                   15
//caps lock             58
//left shift            42
//right shift           54
//left ctrl             29
//right ctrl            97
//left alt/option key   56
//right alt/option key  100
//space bar             57
//enter                 28
//comma                 51
//dot                   52
//foward slash          53
//semi colon            39
//quotation marks       40
//square bracket open   26
//square bracket close  27
//back slash            43
//1                     2
//2                     3
//3                     4
//4                     5
//5                     6
//6                     7
//7                     8
//8                     9
//9                     10
//0                     11
//minus sign            12
//plus sign             13
//Back space            14
//arrow up              103
//arrow left            105
//arrow right           106
//arrow down            108

//Windows
//left ctrl             29
//right ctrl            29
//left alt/option key   56
//right alt/option key  56
//arrow up              72
//arrow left            75
//arrow right           77
//arrow down            80

// Simple offset_of macro akin to C++ offsetof

#[derive(Clone)]
pub struct MappedKeys {
  a_pressed: bool,
  b_pressed: bool,
  c_pressed: bool,
  d_pressed: bool,
  e_pressed: bool,
  f_pressed: bool,
  g_pressed: bool,
  h_pressed: bool,
  i_pressed: bool,
  j_pressed: bool,
  k_pressed: bool,
  l_pressed: bool,
  m_pressed: bool,
  n_pressed: bool,
  o_pressed: bool,
  p_pressed: bool,
  q_pressed: bool,
  r_pressed: bool,
  s_pressed: bool,
  t_pressed: bool,
  u_pressed: bool,
  v_pressed: bool,
  w_pressed: bool,
  x_pressed: bool,
  y_pressed: bool,
  z_pressed: bool,
  one_pressed: bool,
  two_pressed: bool,
  three_pressed: bool,
  four_pressed: bool,
  five_pressed: bool,
  six_pressed: bool,
  seven_pressed: bool,
  eight_pressed: bool,
  nine_pressed: bool,
  space_pressed: bool,
  escape_pressed: bool,
  f1_pressed: bool,
  f2_pressed: bool,
  f3_pressed: bool,
  f4_pressed: bool,
  f5_pressed: bool,
  f6_pressed: bool,
  f7_pressed: bool,
  f8_pressed: bool,
  f9_pressed: bool,
  f10_pressed: bool,
  f11_pressed: bool,
  f12_pressed: bool,
  tab_pressed: bool,
  left_pressed: bool,
  right_pressed: bool,
  up_pressed: bool,
  down_pressed: bool,
  a_held: bool,
  b_held: bool,
  c_held: bool,
  d_held: bool,
  e_held: bool,
  f_held: bool,
  g_held: bool,
  h_held: bool,
  i_held: bool,
  j_held: bool,
  k_held: bool,
  l_held: bool,
  m_held: bool,
  n_held: bool,
  o_held: bool,
  p_held: bool,
  q_held: bool,
  r_held: bool,
  s_held: bool,
  t_held: bool,
  u_held: bool,
  v_held: bool,
  w_held: bool,
  x_held: bool,
  y_held: bool,
  z_held: bool,
  one_held: bool,
  two_held: bool,
  three_held: bool,
  four_held: bool,
  five_held: bool,
  six_held: bool,
  seven_held: bool,
  eight_held: bool,
  nine_held: bool,
  space_held: bool,
  escape_held: bool,
  f1_held: bool,
  f2_held: bool,
  f3_held: bool,
  f4_held: bool,
  f5_held: bool,
  f6_held: bool,
  f7_held: bool,
  f8_held: bool,
  f9_held: bool,
  f10_held: bool,
  f11_held: bool,
  f12_held: bool,
  tab_held: bool,
  left_held: bool,
  right_held: bool,
  up_held: bool,
  down_held: bool,
  pub pressed_this_frame: Vec<String>,
  currently_pressed: Vec<u32>,
  released_this_render: Vec<u32>,
}

impl MappedKeys {
  pub fn new() -> MappedKeys {
    MappedKeys {
      a_pressed: false,
      b_pressed: false,
      c_pressed: false,
      d_pressed: false,
      e_pressed: false,
      f_pressed: false,
      g_pressed: false,
      h_pressed: false,
      i_pressed: false,
      j_pressed: false,
      k_pressed: false,
      l_pressed: false,
      m_pressed: false,
      n_pressed: false,
      o_pressed: false,
      p_pressed: false,
      q_pressed: false,
      r_pressed: false,
      s_pressed: false,
      t_pressed: false,
      u_pressed: false,
      v_pressed: false,
      w_pressed: false,
      x_pressed: false,
      y_pressed: false,
      z_pressed: false,
      one_pressed: false,
      two_pressed: false,
      three_pressed: false,
      four_pressed: false,
      five_pressed: false,
      six_pressed: false,
      seven_pressed: false,
      eight_pressed: false,
      nine_pressed: false,
      space_pressed: false,
      escape_pressed: false,
      f1_pressed: false,
      f2_pressed: false,
      f3_pressed: false,
      f4_pressed: false,
      f5_pressed: false,
      f6_pressed: false,
      f7_pressed: false,
      f8_pressed: false,
      f9_pressed: false,
      f10_pressed: false,
      f11_pressed: false,
      f12_pressed: false,
      tab_pressed: false,
      left_pressed: false,
      right_pressed: false,
      up_pressed: false,
      down_pressed: false,
      a_held: false,
      b_held: false,
      c_held: false,
      d_held: false,
      e_held: false,
      f_held: false,
      g_held: false,
      h_held: false,
      i_held: false,
      j_held: false,
      k_held: false,
      l_held: false,
      m_held: false,
      n_held: false,
      o_held: false,
      p_held: false,
      q_held: false,
      r_held: false,
      s_held: false,
      t_held: false,
      u_held: false,
      v_held: false,
      w_held: false,
      x_held: false,
      y_held: false,
      z_held: false,
      one_held: false,
      two_held: false,
      three_held: false,
      four_held: false,
      five_held: false,
      six_held: false,
      seven_held: false,
      eight_held: false,
      nine_held: false,
      space_held: false,
      escape_held: false,
      f1_held: false,
      f2_held: false,
      f3_held: false,
      f4_held: false,
      f5_held: false,
      f6_held: false,
      f7_held: false,
      f8_held: false,
      f9_held: false,
      f10_held: false,
      f11_held: false,
      f12_held: false,
      tab_held: false,
      left_held: false,
      right_held: false,
      up_held: false,
      down_held: false,
      pressed_this_frame: Vec::new(),
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
    }
  }

  pub fn get_pressed_this_frame(&self) -> Vec<String> {
    self.pressed_this_frame.clone()
  }
  
  pub fn key_held(&self, key: u32) -> bool {
    self.is_key_pressed(key)
  }
  
  pub fn key_pressed(&self, key: u32) -> bool {
    self.was_key_pressed(key)
  }
  /*
  pub fn check_pressed(&self, released: bool, key: u32) -> bool {
    let mut key_pressed = released;
    if !key_pressed {
      if self.key_pressed(key) {
        key_pressed = true;
      }
    } else {
      if self.is_key_pressed(key) {
        key_pressed = false;
      }
    }
    key_pressed
  }*/
  
  #[cfg(target_os = "andoird")]
  pub fn update_keys(&mut self, pressed: Vec<u32>, released: Vec<u32>) {
    
  }
  
  #[cfg(all(not(target_os = "android")))]
  pub fn update_keys(&mut self, pressed: Vec<u32>, released: Vec<u32>) {
    self.pressed_this_frame.clear();
    self.currently_pressed = pressed;
    self.released_this_render = released;
    
    let a = self.a_pressed;
    let b = self.b_pressed;
    let c = self.c_pressed;
    let d = self.d_pressed;
    let e = self.e_pressed;
    let f = self.f_pressed;
    let g = self.g_pressed;
    let h = self.h_pressed;
    let i = self.i_pressed;
    let j = self.j_pressed;
    let k = self.k_pressed;
    let l = self.l_pressed;
    let m = self.m_pressed;
    let n = self.n_pressed;
    let o = self.o_pressed;
    let p = self.p_pressed;
    let q = self.q_pressed;
    let r = self.r_pressed;
    let s = self.s_pressed;
    let t = self.t_pressed;
    let u = self.u_pressed;
    let v = self.v_pressed;
    let w = self.w_pressed;
    let x = self.x_pressed;
    let y = self.y_pressed;
    let z = self.z_pressed;
    let left = self.left_pressed;
    let right = self.right_pressed;
    let up = self.up_pressed;
    let down = self.down_pressed;
    let one = self.one_pressed;
    let two = self.two_pressed;
    let three = self.three_pressed;
    let four = self.four_pressed;
    let five = self.five_pressed;
    let six = self.six_pressed;
    let seven = self.seven_pressed;
    let eight = self.eight_pressed;
    let nine = self.nine_pressed;
    
    self.a_held = self.key_held(A);
    self.b_held = self.key_held(B);
    self.c_held = self.key_held(C);
    self.d_held = self.key_held(D);
    self.e_held = self.key_held(E);
    self.f_held = self.key_held(F);
    self.g_held = self.key_held(G);
    self.h_held = self.key_held(H);
    self.i_held = self.key_held(I);
    self.j_held = self.key_held(J);
    self.k_held = self.key_held(K);
    self.l_held = self.key_held(L);
    self.m_held = self.key_held(M);
    self.n_held = self.key_held(N);
    self.o_held = self.key_held(O);
    self.p_held = self.key_held(P);
    self.q_held = self.key_held(Q);
    self.r_held = self.key_held(R);
    self.s_held = self.key_held(S);
    self.t_held = self.key_held(T);
    self.u_held = self.key_held(U);
    self.v_held = self.key_held(V);
    self.w_held = self.key_held(W);
    self.x_held = self.key_held(X);
    self.y_held = self.key_held(Y);
    self.z_held = self.key_held(Z);
    self.left_held = self.key_held(LEFT);
    self.right_held = self.key_held(RIGHT);
    self.up_held = self.key_held(UP);
    self.down_held = self.key_held(DOWN);
    self.one_held = self.key_held(ONE);
    self.two_held = self.key_held(TWO);
    self.three_held = self.key_held(THREE);
    self.four_held = self.key_held(FOUR);
    self.five_held = self.key_held(FIVE);
    self.six_held = self.key_held(SIX);
    self.seven_held = self.key_held(SEVEN);
    self.eight_held = self.key_held(EIGHT);
    self.nine_held = self.key_held(NINE);
    
    self.a_pressed = if a { false } else { self.key_pressed(A) };
    self.b_pressed = if b { false } else { self.key_pressed(B) };
    self.c_pressed = if c { false } else { self.key_pressed(C) };
    self.d_pressed = if d { false } else { self.key_pressed(D) };
    self.e_pressed = if e { false } else { self.key_pressed(E) };
    self.f_pressed = if f { false } else { self.key_pressed(F) };
    self.g_pressed = if g { false } else { self.key_pressed(G) };
    self.h_pressed = if h { false } else { self.key_pressed(H) };
    self.i_pressed = if i { false } else { self.key_pressed(I) };
    self.j_pressed = if j { false } else { self.key_pressed(J) };
    self.k_pressed = if k { false } else { self.key_pressed(K) };
    self.l_pressed = if l { false } else { self.key_pressed(L) };
    self.m_pressed = if m { false } else { self.key_pressed(M) };
    self.n_pressed = if n { false } else { self.key_pressed(N) };//if n { false } else { self.key_pressed(N) };
    self.o_pressed = if o { false } else { self.key_pressed(O) };
    self.p_pressed = if p { false } else { self.key_pressed(P) };
    self.q_pressed = if q { false } else { self.key_pressed(Q) };
    self.r_pressed = if r { false } else { self.key_pressed(R) };
    self.s_pressed = if s { false } else { self.key_pressed(S) };
    self.t_pressed = if t { false } else { self.key_pressed(T) };
    self.u_pressed = if u { false } else { self.key_pressed(U) };
    self.v_pressed = if v { false } else { self.key_pressed(V) };
    self.w_pressed = if w { false } else { self.key_pressed(W) };
    self.x_pressed = if x { false } else { self.key_pressed(X) };
    self.y_pressed = if y { false } else { self.key_pressed(Y) };
    self.z_pressed = if z { false } else { self.key_pressed(Z) };
    self.left_pressed = if left { false } else { self.key_pressed(LEFT) };
    self.right_pressed = if right { false } else { self.key_pressed(RIGHT) };
    self.up_pressed = if up { false } else { self.key_pressed(UP) };
    self.down_pressed = if down { false } else { self.key_pressed(DOWN) };
    self.one_pressed = if one { false } else { self.key_pressed(ONE) };
    self.two_pressed = if two { false } else { self.key_pressed(TWO) };
    self.three_pressed = if three { false } else { self.key_pressed(THREE) };
    self.four_pressed = if four { false } else { self.key_pressed(FOUR) };
    self.five_pressed = if five { false } else { self.key_pressed(FIVE) };
    self.six_pressed = if six { false } else { self.key_pressed(SIX) };
    self.seven_pressed = if seven { false } else { self.key_pressed(SEVEN) };
    self.eight_pressed = if eight { false } else { self.key_pressed(EIGHT) };
    self.nine_pressed = if nine { false } else { self.key_pressed(NINE) };
    
    if self.a_pressed {
      self.a_held = false;
    }
    if self.b_pressed {
      self.b_held = false;
    }
    if self.c_pressed {
      self.c_held = false;
    }
    if self.d_pressed {
      self.d_held = false;
    }
    if self.e_pressed {
      self.e_held = false;
    }
    if self.f_pressed {
      self.f_held = false;
    }
    if self.g_pressed {
      self.g_held = false;
    }
    if self.h_pressed {
      self.h_held = false;
    }
    if self.i_pressed {
      self.i_held = false;
    }
    if self.j_pressed {
      self.j_held = false;
    }
    if self.k_pressed {
      self.k_held = false;
    }
    if self.l_pressed {
      self.l_held = false;
    }
    if self.m_pressed {
      self.m_held = false;
    }
    if self.n_pressed {
      self.n_held = false;
    }
    if self.o_pressed {
      self.o_held = false;
    }
    if self.p_pressed {
      self.p_held = false;
    }
    if self.q_pressed {
      self.q_held = false;
    }
    if self.r_pressed {
      self.r_held = false;
    }
    if self.s_pressed {
      self.s_held = false;
    }
    if self.t_pressed {
      self.t_held = false;
    }
    if self.u_pressed {
      self.u_held = false;
    }
    if self.v_pressed {
      self.v_held = false;
    }
    if self.w_pressed {
      self.w_held = false;
    }
    if self.x_pressed {
      self.x_held = false;
    }
    if self.y_pressed {
      self.y_held = false;
    }
    if self.z_pressed {
      self.z_held = false;
    }
    if self.left_pressed {
      self.left_held = false;
    }
    if self.right_pressed {
      self.right_held = false;
    }
    if self.up_pressed {
      self.up_held = false;
    }
    if self.down_pressed {
      self.down_held = false;
    }
  }
  
  fn is_key_pressed(&self, key: u32) -> bool {
    for element in self.currently_pressed.iter() {
      if *element == key {
        return true;
      }
    }
    false
  }

  fn was_key_pressed(&self, key: u32) -> bool {
    for element in self.released_this_render.iter() {
      if *element == key {
        return true;
      }
    }
    false
  }
  
  pub fn a_pressed(&self) -> bool {
    self.a_pressed
  }
  
  pub fn b_pressed(&self) -> bool {
    self.b_pressed
  }
  
  pub fn c_pressed(&self) -> bool {
    self.c_pressed
  }
  
  pub fn d_pressed(&self) -> bool {
    self.d_pressed
  }
  
  pub fn e_pressed(&self) -> bool {
    self.e_pressed
  }
  
  pub fn f_pressed(&self) -> bool {
    self.f_pressed
  }
  
  pub fn g_pressed(&self) -> bool {
    self.g_pressed
  }
  
  pub fn h_pressed(&self) -> bool {
    self.h_pressed
  }
  
  pub fn i_pressed(&self) -> bool {
    self.i_pressed
  }
  
  pub fn j_pressed(&self) -> bool {
    self.j_pressed
  }
  
  pub fn k_pressed(&self) -> bool {
    self.k_pressed
  }
  
  pub fn l_pressed(&self) -> bool {
    self.l_pressed
  }
  
  pub fn m_pressed(&self) -> bool {
    self.m_pressed
  }
  
  pub fn n_pressed(&self) -> bool {
    self.n_pressed
  }
  
  pub fn o_pressed(&self) -> bool {
    self.o_pressed
  }
  
  pub fn p_pressed(&self) -> bool {
    self.p_pressed
  }
  
  pub fn q_pressed(&self) -> bool {
    self.q_pressed
  }
  
  pub fn r_pressed(&self) -> bool {
    self.r_pressed
  }
  
  pub fn s_pressed(&self) -> bool {
    self.s_pressed
  }
  
  pub fn t_pressed(&self) -> bool {
    self.t_pressed
  }
  
  pub fn u_pressed(&self) -> bool {
    self.u_pressed
  }
  
  pub fn v_pressed(&self) -> bool {
    self.v_pressed
  }
  
  pub fn w_pressed(&self) -> bool {
    self.w_pressed
  }
  
  pub fn x_pressed(&self) -> bool {
    self.x_pressed
  }
  
  pub fn y_pressed(&self) -> bool {
    self.y_pressed
  }
  
  pub fn z_pressed(&self) -> bool {
    self.z_pressed
  }
  
  pub fn one_pressed(&self) -> bool {
    self.one_pressed
  }
  
  pub fn two_pressed(&self) -> bool {
    self.two_pressed
  }
  
  pub fn three_pressed(&self) -> bool {
    self.three_pressed
  }
  
  pub fn four_pressed(&self) -> bool {
    self.four_pressed
  }
  
  pub fn five_pressed(&self) -> bool {
    self.five_pressed
  }
  
  pub fn six_pressed(&self) -> bool {
    self.six_pressed
  }
  
  pub fn seven_pressed(&self) -> bool {
    self.seven_pressed
  }
  
  pub fn eight_pressed(&self) -> bool {
    self.eight_pressed
  }
  
  pub fn nine_pressed(&self) -> bool {
    self.nine_pressed
  }
  
  pub fn space_pressed(&self) -> bool {
    self.space_pressed
  }
  
  pub fn escape_pressed(&self) -> bool {
    self.escape_pressed
  }

  pub fn f1_pressed(&self) -> bool {
    self.f1_pressed
  }
  
  pub fn f2_pressed(&self) -> bool {
    self.f2_pressed
  }
  
  pub fn f3_pressed(&self) -> bool {
    self.f3_pressed
  }
  
  pub fn f4_pressed(&self) -> bool {
    self.f4_pressed
  }
  
  pub fn f5_pressed(&self) -> bool {
    self.f5_pressed
  }
  
  pub fn f6_pressed(&self) -> bool {
    self.f6_pressed
  }
  
  pub fn f7_pressed(&self) -> bool {
    self.f7_pressed
  }
  
  pub fn f8_pressed(&self) -> bool {
    self.f8_pressed
  }
  
  pub fn f9_pressed(&self) -> bool {
    self.f9_pressed
  }
  
  pub fn f10_pressed(&self) -> bool {
    self.f10_pressed
  }
  
  pub fn f11_pressed(&self) -> bool {
    self.f11_pressed
  }
  
  pub fn f12_pressed(&self) -> bool {
    self.f12_pressed
  }
  
  pub fn tab_pressed(&self) -> bool {
    self.tab_pressed
  }
  
  pub fn left_pressed(&self) -> bool {
    self.left_pressed
  }
  
  pub fn right_pressed(&self) -> bool {
    self.right_pressed
  }
  
  pub fn up_pressed(&self) -> bool {
    self.up_pressed
  }
  
  pub fn down_pressed(&self) -> bool {
    self.down_pressed
  }
  
  pub fn a_held(&self) -> bool {
    self.a_held
  }
  
  pub fn b_held(&self) -> bool {
    self.b_held
  }
  
  pub fn c_held(&self) -> bool {
    self.c_held
  }
  
  pub fn d_held(&self) -> bool {
    self.d_held
  }
  
  pub fn e_held(&self) -> bool {
    self.e_held
  }
  
  pub fn f_held(&self) -> bool {
    self.f_held
  }
  
  pub fn g_held(&self) -> bool {
    self.g_held
  }
  
  pub fn h_held(&self) -> bool {
    self.h_held
  }
  
  pub fn i_held(&self) -> bool {
    self.i_held
  }
  
  pub fn j_held(&self) -> bool {
    self.j_held
  }
  
  pub fn k_held(&self) -> bool {
    self.k_held
  }
  
  pub fn l_held(&self) -> bool {
    self.l_held
  }
  
  pub fn m_held(&self) -> bool {
    self.m_held
  }
  
  pub fn n_held(&self) -> bool {
    self.n_held
  }
  
  pub fn o_held(&self) -> bool {
    self.o_held
  }
  
  pub fn p_held(&self) -> bool {
    self.p_held
  }
  
  pub fn q_held(&self) -> bool {
    self.q_held
  }
  
  pub fn r_held(&self) -> bool {
    self.r_held
  }
  
  pub fn s_held(&self) -> bool {
    self.s_held
  }
  
  pub fn t_held(&self) -> bool {
    self.t_held
  }
  
  pub fn u_held(&self) -> bool {
    self.u_held
  }
  
  pub fn v_held(&self) -> bool {
    self.v_held
  }
  
  pub fn w_held(&self) -> bool {
    self.w_held
  }
  
  pub fn x_held(&self) -> bool {
    self.x_held
  }
  
  pub fn y_held(&self) -> bool {
    self.y_held
  }
  
  pub fn z_held(&self) -> bool {
    self.z_held
  }
  
  pub fn one_held(&self) -> bool {
    self.one_held
  }
  
  pub fn two_held(&self) -> bool {
    self.two_held
  }
  
  pub fn three_held(&self) -> bool {
    self.three_held
  }
  
  pub fn four_held(&self) -> bool {
    self.four_held
  }
  
  pub fn five_held(&self) -> bool {
    self.five_held
  }
  
  pub fn six_held(&self) -> bool {
    self.six_held
  }
  
  pub fn seven_held(&self) -> bool {
    self.seven_held
  }
  
  pub fn eight_held(&self) -> bool {
    self.eight_held
  }
  
  pub fn nine_held(&self) -> bool {
    self.nine_held
  }
  
  pub fn space_held(&self) -> bool {
    self.space_held
  }
  
  pub fn escape_held(&self) -> bool {
    self.escape_held
  }

  pub fn f1_held(&self) -> bool {
    self.f1_held
  }
  
  pub fn f2_held(&self) -> bool {
    self.f2_held
  }
  
  pub fn f3_held(&self) -> bool {
    self.f3_held
  }
  
  pub fn f4_held(&self) -> bool {
    self.f4_held
  }
  
  pub fn f5_held(&self) -> bool {
    self.f5_held
  }
  
  pub fn f6_held(&self) -> bool {
    self.f6_held
  }
  
  pub fn f7_held(&self) -> bool {
    self.f7_held
  }
  
  pub fn f8_held(&self) -> bool {
    self.f8_held
  }
  
  pub fn f9_held(&self) -> bool {
    self.f9_held
  }
  
  pub fn f10_held(&self) -> bool {
    self.f10_held
  }
  
  pub fn f11_held(&self) -> bool {
    self.f11_held
  }
  
  pub fn f12_held(&self) -> bool {
    self.f12_held
  }
  
  pub fn tab_held(&self) -> bool {
    self.tab_held
  }
  
  pub fn left_held(&self) -> bool {
    self.left_held
  }
  
  pub fn right_held(&self) -> bool {
    self.right_held
  }
  
  pub fn up_held(&self) -> bool {
    self.up_held
  }
  
  pub fn down_held(&self) -> bool {
    self.down_held
  }
}

pub struct Controller {
  gilrs: Gilrs,
}

impl Controller {
  pub fn new() -> Controller {
    let mut controller = Controller {
      gilrs: Gilrs::new().unwrap(),
    };
    controller.list_controllers();
    
    controller
  }
  
  pub fn list_controllers(&mut self) {
    for (id, gamepad) in self.gilrs.gamepads() {
      println!("Gamepad {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
    }
  }
  
  pub fn test_loop(&mut self) {
    loop {
      // Examine new events
      while let Some(Event { id, event, time }) = self.gilrs.next_event() {
        println!("{:?} New event from {}: {:?}", time, id, event);
      }

      // You can also use cached gamepad state
      if self.gilrs[0].is_pressed(Button::South) {
        println!("Button South is pressed (XBox - A, PS - X)");
      }
    }
  }
  
  pub fn update(&mut self) {
    self.gilrs.next_event();
  }
  
  pub fn start_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::Start)
  }
  
  pub fn select_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::Select)
  }
  
  pub fn mode_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::Mode)
  }
  
  pub fn a_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::South)
  }
  
  pub fn y_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::North)
  }
  
  pub fn b_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::East)
  }
  
  pub fn x_button_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::West)
  }
  
  pub fn dpad_down_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::DPadDown)
  }
  
  pub fn left_bumper_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::LeftTrigger)
  }
  
  pub fn left_trigger_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::LeftTrigger2)
  }
  
  pub fn right_bumper_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::RightTrigger)
  }
  
  pub fn right_trigger_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::RightTrigger2)
  }
  
  pub fn dpad_up_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::DPadUp)
  }
  
  pub fn dpad_left_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::DPadLeft)
  }
  
  pub fn dpad_right_pressed(&self) -> bool {
    self.gilrs[0].is_pressed(Button::DPadRight)
  }
  
  pub fn left_stick_position(&self) -> Vector2<f32> {
    let x = self.gilrs[0].value(Axis::LeftStickX);
    let y = self.gilrs[0].value(Axis::LeftStickY);
    
    Vector2::new(x, y)
  }
  
  pub fn right_stick_position(&self) -> Vector2<f32> {
    let x = self.gilrs[0].value(Axis::RightStickX);
    let y = self.gilrs[0].value(Axis::RightStickY);
    
    Vector2::new(x, y)
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
