// pub struct AveragedCollection {
//     list: Vec<i32>,
//     average: f64,
// }

// impl AveragedCollection {
//     pub fn add(&mut self, value: i32) {
//         self.list.push(value);
//         self.update_average();
//     }

//     pub fn remove(&mut self) -> Option<i32> {
//         let result = self.list.pop();
//         match result {
//             Some(value) => {
//                 self.update_average();
//                 Some(value)
//             },
//             None => None,
//         }
//     }

//     pub fn average(&self) -> f64 {
//         self.average
//     }

//     fn update_average(&mut self) {
//         let total: i32 = self.list.iter().sum();
//         self.average = total as f64 / self.list.len() as f64;
//     }
// }


// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// // 泛型实现
// pub struct Screen1<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen1<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw()
//         }
//     }
// }
// // 泛型实现


// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         // 绘制一个按钮
//     }
// }



pub trait Draw {
    fn draw(&self);
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
}