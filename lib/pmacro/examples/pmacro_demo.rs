use pmacro::SlintFromConvert;

mod slint {
    pub type VecModel<T> = Vec<T>;
    pub type ModelRc<T> = std::sync::Arc<VecModel<T>>;
}

use slint::ModelRc;

#[derive(Debug, Clone, Default, SlintFromConvert)]
#[from("UIFoo")]
#[vec_ui("other1")]
pub struct Foo {
    pub name: String,
    pub age: u32,

    #[vec(from = "other_name")]
    pub other: Vec<i32>,

    #[vec(from = "bar")]
    pub foo: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UIFoo {
    pub name: String,
    pub age: u32,
    pub other_name: ModelRc<i32>,
    pub other1: ModelRc<i32>,
    pub bar: ModelRc<String>,
}

fn main() {
    let f1 = Foo {
        name: "name1".to_string(),
        age: 1,
        other: vec![1, 2, 3],
        foo: vec!["foo".to_string(), "bar".to_string()],
    };

    let ui_f1: UIFoo = f1.into();
    println!("{ui_f1:?}");

    let f2: Foo = ui_f1.into();
    println!("{f2:?}");
}

// `SlintFromConvert` macro expand
// impl From<Foo> for UIFoo {
//     fn from(entry: Foo) -> Self {
//         Self {
//             name: entry.name.into(),
//             age: entry.age.into(),
//             other_name: slint::ModelRc::new(
//                 entry
//                     .other
//                     .into_iter()
//                     .map(|item| item.into())
//                     .collect::<slint::VecModel<_>>(),
//             ),
//             bar: slint::ModelRc::new(
//                 entry
//                     .foo
//                     .into_iter()
//                     .map(|item| item.into())
//                     .collect::<slint::VecModel<_>>(),
//             ),
//             other1: slint::ModelRc::new(slint::VecModel::default()),
//             ..Default::default()
//         }
//     }
// }
// impl From<UIFoo> for Foo {
//     fn from(entry: UIFoo) -> Self {
//         Self {
//             name: entry.name.into(),
//             age: entry.age.into(),
//             other: entry
//                 .other_name
//                 .iter()
//                 .map(|item| item.clone().into())
//                 .collect::<Vec<_>>(),
//             foo: entry.bar.iter().map(|item| item.clone().into()).collect::<Vec<_>>(),
//         }
//     }
// }
