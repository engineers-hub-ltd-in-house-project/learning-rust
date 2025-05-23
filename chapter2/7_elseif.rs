/*
 * else if文による条件分岐
 *
 * この例では、Rustでの複数条件を持つ分岐（else if）の使い方を示しています。
 * else if文を使うと、最初の条件が偽の場合に別の条件をチェックできます。
 *
 * else if文の特徴：
 * 1. 最初のif条件が偽の場合に次の条件を評価する
 * 2. 条件は上から順番に評価され、最初に真となった条件のブロックのみが実行される
 * 3. どの条件も真にならない場合はelse節（存在する場合）が実行される
 *
 * この例では、数値が5、4、3、2で割り切れるかどうかを順番にチェックし、
 * 最初に条件を満たした場合にメッセージを表示します。
 */

fn main() {
    let num = 123;
    if num % 5 == 0 {
        println!("{}は、5で割れます。", num);
    } else if num % 4 == 0 {
        println!("{}は、4で割れます。", num);
    } else if num % 3 == 0 {
        println!("{}は、３で割れます。", num);
    } else if num % 2 == 0 {
        println!("{}は、２で割れます。", num);
    } else {
        println!("{}は、うまく割れませんでした。", num);
    }
}
