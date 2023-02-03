/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-03 09:23:35
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-03 17:33:41
 * @FilePath: /iterator_demo/src/main.rs
 * @Description:
 */
fn main() {
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));

}
