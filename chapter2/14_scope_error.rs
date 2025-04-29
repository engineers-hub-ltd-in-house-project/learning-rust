fn main() {
    let a = 1;
    {
        let b = a;
        let c = a + b;
        {
            let d = a + b + c;
        }
        let e = a + b + c + d;
    }
    let f = a + b + c + d + e;
}
