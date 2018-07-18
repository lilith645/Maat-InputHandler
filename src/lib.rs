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
pub const F10: u32 = 68;
#[cfg(target_os = "macos")]
pub const F10: u32 = 109;

#[cfg(all(not(target_os = "android"), not(target_os = "macos")))]
pub const TAB: u32 = 15;
#[cfg(target_os = "macos")]
pub const TAB: u32 = 48;

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
  escape_released: bool,
  f10_released: bool,
  tab_released: bool,
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
      escape_released: true,
      f10_released: true,
      tab_released: true,
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
    }
  }
  
  pub fn key_released(&mut self, key: u32) -> bool {
    (self.was_key_released(key) || !self.is_key_pressed(key))
  }
  
  pub fn check_released(&mut self, released: bool, key: u32) -> bool {
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
    
    if !self.escape_released {
      if self.key_released(ESCAPE) {
        self.escape_released = true;
      }
    } else {
      if self.is_key_pressed(ESCAPE) {
        self.escape_released = false;
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

  fn was_key_released(&mut self, key: u32) -> bool {
    for element in self.released_this_render.iter() {
      if *element == key {
        return true;
      }
    }
    false
  }
  
  pub fn a_pressed(&mut self) -> bool {
    !self.a_released
  }
  
  pub fn b_pressed(&mut self) -> bool {
    !self.b_released
  }
  
  pub fn c_pressed(&mut self) -> bool {
    !self.c_released
  }
  
  pub fn d_pressed(&mut self) -> bool {
    !self.d_released
  }
  
  pub fn e_pressed(&mut self) -> bool {
    !self.e_released
  }
  
  pub fn f_pressed(&mut self) -> bool {
    !self.f_released
  }
  
  pub fn g_pressed(&mut self) -> bool {
    !self.g_released
  }
  
  pub fn h_pressed(&mut self) -> bool {
    !self.h_released
  }
  
  pub fn i_pressed(&mut self) -> bool {
    !self.i_released
  }
  
  pub fn j_pressed(&mut self) -> bool {
    !self.j_released
  }
  
  pub fn k_pressed(&mut self) -> bool {
    !self.k_released
  }
  
  pub fn l_pressed(&mut self) -> bool {
    !self.l_released
  }
  
  pub fn m_pressed(&mut self) -> bool {
    !self.m_released
  }
  
  pub fn n_pressed(&mut self) -> bool {
    !self.n_released
  }
  
  pub fn o_pressed(&mut self) -> bool {
    !self.o_released
  }
  
  pub fn p_pressed(&mut self) -> bool {
    !self.p_released
  }
  
  pub fn q_pressed(&mut self) -> bool {
    !self.q_released
  }
  
  pub fn r_pressed(&mut self) -> bool {
    !self.r_released
  }
  
  pub fn s_pressed(&mut self) -> bool {
    !self.s_released
  }
  
  pub fn t_pressed(&mut self) -> bool {
    !self.t_released
  }
  
  pub fn u_pressed(&mut self) -> bool {
    !self.u_released
  }
  
  pub fn v_pressed(&mut self) -> bool {
    !self.v_released
  }
  
  pub fn w_pressed(&mut self) -> bool {
    !self.w_released
  }
  
  pub fn x_pressed(&mut self) -> bool {
    !self.x_released
  }
  
  pub fn y_pressed(&mut self) -> bool {
    !self.y_released
  }
  
  pub fn z_pressed(&mut self) -> bool {
    !self.z_released
  }
  
  pub fn escape_pressed(&mut self) -> bool {
    !self.escape_released
  }
  
  pub fn f10_pressed(&mut self) -> bool {
    !self.f10_released
  }
  
  pub fn tab_pressed(&mut self) -> bool {
    !self.tab_released
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
