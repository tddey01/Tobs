
//1、rust语言将错误分为两个类别：可恢复错误和不可恢复错误
//（1）可恢复错误通常代表向用户报告错误和重试操作是合理的情况，例如未找到文件。rust中使用Result<T,E>来实现。
//（2）不可恢复错误是bug的同义词，如尝试访问超过数组结尾的位置。rust中通过panic！来实现。
// 2 panic!

// 3  使用BAXKTRACE=1
// 4 Result<T,E>
// 5 简写

fn main() {

    panic!("crash here");
}
