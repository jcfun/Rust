/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-08 16:16:33
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-08 16:17:33
 * @FilePath: /add/add-two/src/lib.rs
 * @Description: 
 */
use rand;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
