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
pub const TWo: u32 = 19;

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

#[derive(Clone)]
pub struct MappedKeys {
  a_released: bool,
  b_released: bool,
  c_released: bool,
  d_released: bool,
  e_released: bool,
  f_released: bool,
  g_released: bool,
  h_released: bool,
  i_released: bool,
  j_released: bool,
  k_released: bool,
  l_released: bool,
  m_released: bool,
  n_released: bool,
  o_released: bool,
  p_released: bool,
  q_released: bool,
  r_released: bool,
  s_released: bool,
  t_released: bool,
  u_released: bool,
  v_released: bool,
  w_released: bool,
  x_released: bool,
  y_released: bool,
  z_released: bool,
  one_released: bool,
  two_released: bool,
  three_released: bool,
  four_released: bool,
  five_released: bool,
  six_released: bool,
  seven_released: bool,
  eight_released: bool,
  nine_released: bool,
  space_released: bool,
  escape_released: bool,
  f1_released: bool,
  f2_released: bool,
  f3_released: bool,
  f4_released: bool,
  f5_released: bool,
  f6_released: bool,
  f7_released: bool,
  f8_released: bool,
  f9_released: bool,
  f10_released: bool,
  f11_released: bool,
  f12_released: bool,
  tab_released: bool,
  pub pressed_this_frame: Vec<String>,
  currently_pressed: Vec<u32>,
  released_this_render: Vec<u32>,
}

impl MappedKeys {
  pub fn new() -> MappedKeys {
    MappedKeys {
      a_released: true,
      b_released: true,
      c_released: true,
      d_released: true,
      e_released: true,
      f_released: true,
      g_released: true,
      h_released: true,
      i_released: true,
      j_released: true,
      k_released: true,
      l_released: true,
      m_released: true,
      n_released: true,
      o_released: true,
      p_released: true,
      q_released: true,
      r_released: true,
      s_released: true,
      t_released: true,
      u_released: true,
      v_released: true,
      w_released: true,
      x_released: true,
      y_released: true,
      z_released: true,
      one_released: true,
      two_released: true,
      three_released: true,
      four_released: true,
      five_released: true,
      six_released: true,
      seven_released: true,
      eight_released: true,
      nine_released: true,
      space_released: true,
      escape_released: true,
      f1_released: true,
      f2_released: true,
      f3_released: true,
      f4_released: true,
      f5_released: true,
      f6_released: true,
      f7_released: true,
      f8_released: true,
      f9_released: true,
      f10_released: true,
      f11_released: true,
      f12_released: true,
      tab_released: true,
      pressed_this_frame: Vec::new(),
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
    }
  }
  
  pub fn get_pressed_this_frame(&self) -> Vec<String> {
    self.pressed_this_frame.clone()
  }
  
  pub fn key_released(&self, key: u32) -> bool {
    (self.was_key_released(key) || !self.is_key_pressed(key))
  }
  
  pub fn check_released(&self, released: bool, key: u32) -> bool {
    let mut key_released = released;
    if !key_released {
      if self.key_released(key) {
        key_released = true;
      }
    } else {
      if self.is_key_pressed(key) {
        key_released = false;
      }
    }
    key_released
  }
  
  pub fn update_keys(&mut self, pressed: Vec<u32>, released: Vec<u32>) {
    self.pressed_this_frame.clear();
    self.currently_pressed = pressed;
    self.released_this_render = released;
    
    let a = self.a_released;
    let b = self.b_released;
    let c = self.c_released;
    let d = self.d_released;
    let e = self.e_released;
    let f = self.f_released;
    let g = self.g_released;
    let h = self.h_released;
    let i = self.i_released;
    let j = self.j_released;
    let k = self.k_released;
    let l = self.l_released;
    let m = self.m_released;
    let n = self.n_released;
    let o = self.o_released;
    let p = self.p_released;
    let q = self.q_released;
    let r = self.r_released;
    let s = self.s_released;
    let t = self.t_released;
    let u = self.u_released;
    let v = self.v_released;
    let w = self.w_released;
    let x = self.x_released;
    let y = self.y_released;
    let z = self.z_released;
    
    self.a_released = self.check_released(a, A);
    self.b_released = self.check_released(b, B);
    self.c_released = self.check_released(c, C);
    self.d_released = self.check_released(d, D);
    self.e_released = self.check_released(e, E);
    self.f_released = self.check_released(f, F);
    self.g_released = self.check_released(g, G);
    self.h_released = self.check_released(h, H);
    self.i_released = self.check_released(i, I);
    self.j_released = self.check_released(j, J);
    self.k_released = self.check_released(k, K);
    self.l_released = self.check_released(l, L);
    self.m_released = self.check_released(m, M);
    self.n_released = self.check_released(n, N);
    self.o_released = self.check_released(o, O);
    self.p_released = self.check_released(p, P);
    self.q_released = self.check_released(q, Q);
    self.r_released = self.check_released(r, R);
    self.s_released = self.check_released(s, S);
    self.t_released = self.check_released(t, T);
    self.u_released = self.check_released(u, U);
    self.v_released = self.check_released(v, V);
    self.w_released = self.check_released(w, W);
    self.x_released = self.check_released(x, X);
    self.y_released = self.check_released(y, Y);
    self.z_released = self.check_released(z, Z);
    
    if !self.one_released {
      if self.key_released(ONE) {
        self.one_released = true;
      }
    } else {
      if self.is_key_pressed(ONE) {
        self.one_released = false;
      }
    }
    
    if !self.two_released {
      if self.key_released(TWO) {
        self.two_released = true;
      }
    } else {
      if self.is_key_pressed(TWO) {
        self.two_released = false;
      }
    }
    
    if !self.three_released {
      if self.key_released(THREE) {
        self.three_released = true;
      }
    } else {
      if self.is_key_pressed(THREE) {
        self.three_released = false;
      }
    }
    
    if !self.four_released {
      if self.key_released(FOUR) {
        self.four_released = true;
      }
    } else {
      if self.is_key_pressed(FOUR) {
        self.four_released = false;
      }
    }
    
    if !self.five_released {
      if self.key_released(FIVE) {
        self.five_released = true;
      }
    } else {
      if self.is_key_pressed(FIVE) {
        self.five_released = false;
      }
    }
    
    if !self.six_released {
      if self.key_released(SIX) {
        self.six_released = true;
      }
    } else {
      if self.is_key_pressed(SIX) {
        self.six_released = false;
      }
    }
    
    if !self.seven_released {
      if self.key_released(SEVEN) {
        self.seven_released = true;
      }
    } else {
      if self.is_key_pressed(SEVEN) {
        self.seven_released = false;
      }
    }
    
    if !self.eight_released {
      if self.key_released(EIGHT) {
        self.eight_released = true;
      }
    } else {
      if self.is_key_pressed(EIGHT) {
        self.eight_released = false;
      }
    }
    
    if !self.nine_released {
      if self.key_released(NINE) {
        self.nine_released = true;
      }
    } else {
      if self.is_key_pressed(NINE) {
        self.nine_released = false;
      }
    }
    
    if !self.space_released {
      if self.key_released(SPACE) {
        self.space_released = true;
      }
    } else {
      if self.is_key_pressed(SPACE) {
        self.space_released = false;
      }
    }
    
    if !self.escape_released {
      if self.key_released(ESCAPE) {
        self.escape_released = true;
      }
    } else {
      if self.is_key_pressed(ESCAPE) {
        self.escape_released = false;
      }
    }
    
    if !self.f1_released {
      if self.key_released(F1) {
        self.f1_released = true;
      }
    } else {
      if self.is_key_pressed(F1) {
        self.f1_released = false;
      }
    }
    
    if !self.f2_released {
      if self.key_released(F2) {
        self.f2_released = true;
      }
    } else {
      if self.is_key_pressed(F2) {
        self.f2_released = false;
      }
    }
    
    if !self.f3_released {
      if self.key_released(F3) {
        self.f3_released = true;
      }
    } else {
      if self.is_key_pressed(F3) {
        self.f3_released = false;
      }
    }
    
    if !self.f4_released {
      if self.key_released(F4) {
        self.f4_released = true;
      }
    } else {
      if self.is_key_pressed(F4) {
        self.f4_released = false;
      }
    }
    
    if !self.f5_released {
      if self.key_released(F5) {
        self.f5_released = true;
      }
    } else {
      if self.is_key_pressed(F5) {
        self.f5_released = false;
      }
    }
    
    if !self.f6_released {
      if self.key_released(F6) {
        self.f6_released = true;
      }
    } else {
      if self.is_key_pressed(F6) {
        self.f6_released = false;
      }
    }
    
    if !self.f7_released {
      if self.key_released(F7) {
        self.f7_released = true;
      }
    } else {
      if self.is_key_pressed(F7) {
        self.f7_released = false;
      }
    }
    
    if !self.f8_released {
      if self.key_released(F8) {
        self.f8_released = true;
      }
    } else {
      if self.is_key_pressed(F8) {
        self.f8_released = false;
      }
    }
    
    if !self.f9_released {
      if self.key_released(F9) {
        self.f9_released = true;
      }
    } else {
      if self.is_key_pressed(F9) {
        self.f9_released = false;
      }
    }
    
    if !self.f10_released {
      if self.key_released(F10) {
        self.f10_released = true;
      }
    } else {
      if self.is_key_pressed(F10) {
        self.f10_released = false;
      }
    }
    
    if !self.f11_released {
      if self.key_released(F11) {
        self.f11_released = true;
      }
    } else {
      if self.is_key_pressed(F11) {
        self.f12_released = false;
      }
    }
    
    if !self.f12_released {
      if self.key_released(F12) {
        self.f12_released = true;
      }
    } else {
      if self.is_key_pressed(F12) {
        self.f12_released = false;
      }
    }
    
    if !self.tab_released {
      if self.key_released(TAB) {
        self.tab_released = true;
      }
    } else {
      if self.is_key_pressed(TAB) {
        self.tab_released = false;
      }
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

  fn was_key_released(&self, key: u32) -> bool {
    for element in self.released_this_render.iter() {
      if *element == key {
        return true;
      }
    }
    false
  }
  
  pub fn a_pressed(&self) -> bool {
    !self.a_released
  }
  
  pub fn b_pressed(&self) -> bool {
    !self.b_released
  }
  
  pub fn c_pressed(&self) -> bool {
    !self.c_released
  }
  
  pub fn d_pressed(&self) -> bool {
    !self.d_released
  }
  
  pub fn e_pressed(&self) -> bool {
    !self.e_released
  }
  
  pub fn f_pressed(&self) -> bool {
    !self.f_released
  }
  
  pub fn g_pressed(&self) -> bool {
    !self.g_released
  }
  
  pub fn h_pressed(&self) -> bool {
    !self.h_released
  }
  
  pub fn i_pressed(&self) -> bool {
    !self.i_released
  }
  
  pub fn j_pressed(&self) -> bool {
    !self.j_released
  }
  
  pub fn k_pressed(&self) -> bool {
    !self.k_released
  }
  
  pub fn l_pressed(&self) -> bool {
    !self.l_released
  }
  
  pub fn m_pressed(&self) -> bool {
    !self.m_released
  }
  
  pub fn n_pressed(&self) -> bool {
    !self.n_released
  }
  
  pub fn o_pressed(&self) -> bool {
    !self.o_released
  }
  
  pub fn p_pressed(&self) -> bool {
    !self.p_released
  }
  
  pub fn q_pressed(&self) -> bool {
    !self.q_released
  }
  
  pub fn r_pressed(&self) -> bool {
    !self.r_released
  }
  
  pub fn s_pressed(&self) -> bool {
    !self.s_released
  }
  
  pub fn t_pressed(&self) -> bool {
    !self.t_released
  }
  
  pub fn u_pressed(&self) -> bool {
    !self.u_released
  }
  
  pub fn v_pressed(&self) -> bool {
    !self.v_released
  }
  
  pub fn w_pressed(&self) -> bool {
    !self.w_released
  }
  
  pub fn x_pressed(&self) -> bool {
    !self.x_released
  }
  
  pub fn y_pressed(&self) -> bool {
    !self.y_released
  }
  
  pub fn z_pressed(&self) -> bool {
    !self.z_released
  }
  
  pub fn one_pressed(&self) -> bool {
    !self.one_released
  }
  
  pub fn two_pressed(&self) -> bool {
    !self.two_released
  }
  
  pub fn three_pressed(&self) -> bool {
    !self.three_released
  }
  
  pub fn four_pressed(&self) -> bool {
    !self.four_released
  }
  
  pub fn five_pressed(&self) -> bool {
    !self.five_released
  }
  
  pub fn six_pressed(&self) -> bool {
    !self.six_released
  }
  
  pub fn seven_pressed(&self) -> bool {
    !self.seven_released
  }
  
  pub fn eight_pressed(&self) -> bool {
    !self.eight_released
  }
  
  pub fn nine_pressed(&self) -> bool {
    !self.nine_released
  }
  
  pub fn space_pressed(&self) -> bool {
    !self.space_released
  }
  
  pub fn escape_pressed(&self) -> bool {
    !self.escape_released
  }

  pub fn f1_pressed(&self) -> bool {
    !self.f1_released
  }
  
  pub fn f2_pressed(&self) -> bool {
    !self.f2_released
  }
  
  pub fn f3_pressed(&self) -> bool {
    !self.f3_released
  }
  
  pub fn f4_pressed(&self) -> bool {
    !self.f4_released
  }
  
  pub fn f5_pressed(&self) -> bool {
    !self.f5_released
  }
  
  pub fn f6_pressed(&self) -> bool {
    !self.f6_released
  }
  
  pub fn f7_pressed(&self) -> bool {
    !self.f7_released
  }
  
  pub fn f8_pressed(&self) -> bool {
    !self.f8_released
  }
  
  pub fn f9_pressed(&self) -> bool {
    !self.f9_released
  }
  
  pub fn f10_pressed(&self) -> bool {
    !self.f10_released
  }
  
  pub fn f11_pressed(&self) -> bool {
    !self.f11_released
  }
  
  pub fn f12_pressed(&self) -> bool {
    !self.f12_released
  }
  
  pub fn tab_pressed(&self) -> bool {
    !self.tab_released
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
