#[derive(Debug, PartialEq, Eq)]
pub struct One {
    first_layer : Option<Two>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Two {
    second_layer: Option<Three>
}
#[derive(Debug, PartialEq, Eq)]
pub struct Three {
    third_layer: Option<Four>
}
#[derive(Debug, PartialEq, Eq)]
pub struct Four {
    fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.as_ref().and_then(|two| two.second_layer.as_ref().and_then(|three| three.third_layer.as_ref().and_then(|four| four.fourth_layer)))
    }
}



// fn main() {
//     let a = One {
//         first_layer : Some(Two {
//             second_layer: Some(Three {
//                 third_layer: Some(Four {
//                     fourth_layer: Some(1000)
//                 })
//             })
//         })
//     };

//     // output: 1000
//     println!("{:?}", match a.get_fourth_layer() {
//         Some(e) => e,
//         None => 0
//     })
// }
