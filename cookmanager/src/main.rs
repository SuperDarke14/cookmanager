/*
Goals--
initialize to a file, cuz easy
List of ingredients
Quantity of ingredients
Recipe
Recipe checks ingredients
ask if want to auto-update ingredients
if yes then remove ingredients from stored List

*/
#[derive(Debug)]
struct ingredient_list<'a> {
    name: &'a str,
    quantity: u8,
}

struct recipe<'a> {
    name: &'a str,
    serves_number: u8,
    ingredients: Vec<ingredient_list<'a>>//this may be problematic
}

fn main() {
    println!("{:?}",
        test_total(
            &mut test_ingredient(),
            &test_single_recipe())
        .unwrap());
    println!("{:?}",
        test_bigger(
            &mut multi_ingredient_gen(),
            &big_recipe_gen())
        .unwrap()
    );
    // println!("{:?}",
    //     evil_test(
    //         &mut empty_inglist_gen(),
    //         &big_recipe_gen())
    //     .unwrap()
    // );
}

// fn initialize_ingredients(){
//     //TODO read ingredients from a file
// }



fn test_ingredient() -> ingredient_list<'static> {
    let mut testIngred = ingredient_list{name : "Flour", quantity: 1};
    return testIngred
}

fn test_single_recipe() -> recipe<'static> {
    let testedRec = recipe{name: "test recipe", serves_number: 4, ingredients: vec!(ingredient_list{name: "Flour", quantity: 1})};
    return testedRec
}

fn multi_ingredient_gen() -> Vec<ingredient_list<'static>> {
    let mut big_ingred_list = vec!(
        ingredient_list {name: "Flour", quantity: 1},
        ingredient_list {name: "Sugar", quantity: 10},
        ingredient_list {name: "Egg", quantity: 3}
    );
    return big_ingred_list

}
fn empty_inglist_gen() -> Vec<ingredient_list<'static>> {
    let mut big_ingred_list = vec!(
        ingredient_list {name: "Flour", quantity: 0},
        ingredient_list {name: "Weeds", quantity: 10},
        ingredient_list {name: "Egg", quantity: 0}
    );
    return big_ingred_list
}

fn big_recipe_gen() -> recipe<'static> {
    let testedRec = recipe{
        name: "Multi-ingredient recipe",
        serves_number: 4,
        ingredients: vec!(
            ingredient_list {name: "Flour", quantity: 1},
            ingredient_list {name: "Sugar", quantity: 5},
            ingredient_list {name: "Egg", quantity: 2}
        )
    };
    return testedRec
}

fn test_bigger<'a>(il: &mut Vec<ingredient_list<'a>>, rec: &'a recipe<'a>) -> Result<String, (&'a str, &'a str)> {

    let mut ing_names = Vec::new();
    for ing_item in &mut *il {
        ing_names.push(ing_item.name.to_string());
    }
    for item in &rec.ingredients{
        if !ing_names.iter().any(|ing_item| ing_item == item.name) {
                    return Err(("{:?} not in ingredient", item.name))
                    //this whole goofus copies the item names over to
                    //a second list, and then checks if every name in Recipe
                    //is in the second list
                    //I got this from gemini so it doesn't make
                    //that much sense to me but knowing there's a
                    //.any() for iterators made my life easier
        }
    }

    for item in &rec.ingredients {
        for ing_item in &mut *il {
            if ing_item.name == item.name {
                if ing_item.quantity >= item.quantity {
                    ing_item.quantity -= item.quantity
                } else {
                    return Err(("Insufficient Quantity of {:?}", ing_item.name))
                }
            }


        }
    };
    return Ok("Multi-ingredient test Successful".to_string())
}

fn evil_test<'a>(il: &mut Vec<ingredient_list<'a>>, rec: &'a recipe<'a>) -> Result<String, (&'a str, &'a str)> {
        //this checks for correct fail-states
        let mut ing_names = Vec::new();
        for ing_item in &mut *il {
            ing_names.push(ing_item.name.to_string());
        }
        for item in &rec.ingredients{
            if ing_names.iter().any(|ing_item| ing_item == item.name) {
                        println!("Successful failure for not having {:?}", item);
                        //this whole goofus copies the item names over to
                        //a second list, and then checks if every name in Recipe
                        //is in the second list
            }
        }

        for item in &rec.ingredients {
            for ing_item in &mut *il {
                if ing_item.name == item.name {
                    if ing_item.quantity >= item.quantity {
                        return Err(("Failure on incorrect quantity", ing_item.name))
                    } else {
                        println!("Successful failure for incorrect quantity of {:?}", item)
                    }
                }


            }
        };
        return Ok("Multi-ingredient fail-test Successful".to_string())

}

fn test_total(IL: &mut ingredient_list, REC: &recipe) -> Result<String, ()> {
    //I will skip prompting for testing purposes
    //first check if ingredients contains the right item
    //if so, then subtract from ingredient_list
    //then print ingredients + success message
    //a basic test. If broken, then something's gone wrong
    //with the structs

    for item in &REC.ingredients {
        if item.name == IL.name {
            IL.quantity -= item.quantity;
        }
    }
    if IL.quantity == 0 {
        Ok("Single-ingredient test sussyful".to_string())
    } else {
        return Err(())
    }

}
