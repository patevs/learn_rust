
/*
    1_testing/src/main.rs
*/

const NAME: &str = "basic_tests";
const VERSION: f32 = 0.5;

// Basic Rectangle Structure
struct Brectangle {
    width: u32,
    height: u32
}

// Brectangle structure (object) implementation
impl Brectangle {
    fn is_square(&self) -> bool {
        &self.width == &self.height
    }
}

// main program function
fn main() {
    print_welcome();
}

// print welcome message
fn print_welcome() {
    println!("\nRunning program...");
    println!("\n Program info: ");
    println!("\tName:    {}", &NAME);
    println!("\tVersion: {}", &VERSION);
    println!("\n");
}

// returns an 8 bit unsigned integer, in this case 7
fn give_seven() -> u8 {
    7
}

// basic_tests project tests module
#[cfg(test)]
mod basic_tests {

    #[test]
    fn test_basic(){
        assert!(1 == 1); // Ok
    }

    #[test]
    #[should_panic]
    fn test_panic(){
        panic!("Test Panics!");
    }

    #[test]
    fn test_equal(){
        // check second parameter is equal
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_nt_eq(){
        // check second parameter is not equal
        assert_ne!(2, 1 + 2);
    }

    #[test]
    #[ignore]
    fn test_ignore(){
        println!("this test should be ignored");
    }

    #[test]
    fn test_ex_fn(){
        // check extrernal function give_seven returns 7
        assert_eq!(super::give_seven(), 7);
        assert_ne!(super::give_seven(), 6);
    }

    #[test]
    fn test_brect(){
        let brect = super::Brectangle{
            width: 50,
            height: 50
        };
        assert!(brect.is_square());
    }

    #[test]
    fn test_brect_sqr(){
        let brect = super::Brectangle{
            width: 50,
            height: 25
        };
        assert_eq!(brect.is_square(), false);
        assert_ne!(brect.is_square(), true);
    }


} // EO basic_tests module


// EOF
