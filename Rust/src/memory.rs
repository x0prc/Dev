enum Light {
    Bright,
    Dim,
}
//borrow / reference
fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("The light is bright."),
        Light::Dim => println!("The light is dim."),
    }
}
fn main(){
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}
