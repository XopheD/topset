use topset::*;

pub fn main()
{
    let mut top = TopSet::<f32,_>::new(5, f32::lt);
    // top.extend(vec![81.5, 4.5, 4., 1., 45., 22., 11.]);
    //  top.extend(vec![81.5, 4.5, 4., 1., 45., 22., 11.]);
    vec![81.5, 4.5, 4., 1., 45., 22., 11.,93.].into_iter().for_each(|u| { dbg!(&top); dbg!(&u); dbg!(top.insert(u)); dbg!(&top); });
    vec![81.5, 4.5, 4., 1., 45., 22., 11.].into_iter().for_each(|u| { dbg!(&top); dbg!(&u); dbg!(top.insert(u));dbg!(&top); });
    assert_eq![ top.pop(), Some(4.5) ];
    assert_eq![ top.pop(), Some(4.) ];
    assert_eq![ top.pop(), Some(4.) ];
    assert_eq![ top.pop(), Some(1.) ];
    assert_eq![ top.pop(), Some(1.) ];
    assert_eq![ top.pop(), None ];
}