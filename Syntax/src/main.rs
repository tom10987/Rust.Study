
fn main()
{
    {
        let x = 1;
        println!( "x = {}", x );
    }

    {
        let x: i32 = 2;
        println!( "x = {}", x );
    }

    {
        let x: i32;
        x = 100;
        println!( "x = {}", x );
    }

    {
        let mut x = 100;
        x = 200;
        println!( "x = {}", x );
    }
}

