pub trait Draw {
  fn draw(&self);
}

/* pub struct Screen {
  // dyn: This allows polymorphism, 
  // which means that we can have a Screen instance that holds a Vec 
  // that contains a Box that holds a type that implements the Draw trait.
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
} */

// Generic Implementation of Screen
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}

impl<T> Screen<T>
where
  T: Draw,
{
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Drawing a button with width: {}, height: {}, and label: {}", self.width, self.height, self.label);
  }
}

pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("Drawing a select box with width: {}, height: {}, and options: {:?}", self.width, self.height, self.options);
  }
}

fn main() {
/*   let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run(); */
}