// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use serde_json::Result;
use tauri::api::file;
use std::fs;

//Matt, please fill these in when you put the program together
const RECIPE_FOLDER_PATH: String = " ".to_string();
const RECIPE_LIST_PATH: String = " ".to_string();
const MEALPLAN_LIST_PATH: String = " ".to_string();
const INGREDIENT_LIST_PATH: String = " ".to_string();

#[derive(Serialize, Deserialize)]
struct Recipe {
    title: String,
    in_mp: bool,
    servings: f64,
    ingredients_list: Vec<(f64, Ingredient)>,
    directions_list: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Ingredient {
    name: String,
    unit_of_measurement: String
}

#[tauri::command]
fn load_recipe(title: String) -> Recipe{
    let path = format!("{}{}", title, ".json");
    let raw_data = fs::read_to_string(&path).expect("Invalid Title");
    serde_json::from_str(&raw_data).expect("Recipe file unreadable: incorrectly formatted")
}

#[tauri::command]
fn all_recipes() -> Vec<(String, bool)>{
    let path = RECIPE_LIST_PATH;
    let raw_data = fs::read_to_string(&path).expect("Invalid Path");
    serde_json::from_str(&raw_data).expect("Recipe list file unreadable: incorrectly formatted")
}

#[tauri::command]
fn meal_plan() -> Vec<String>{
    let path = MEALPLAN_LIST_PATH;
    let raw_data = fs::read_to_string(&path).expect("Invalid Path");
    serde_json::from_str(&raw_data).expect("Meal plan list file unreadable: incorrectly formatted")
}

#[tauri::command]
fn load_ingredients() -> Vec<Ingredient>{
    let path = INGREDIENT_LIST_PATH;
    let raw_data = fs::read_to_string(&path).expect("Invalid Path");
    serde_json::from_str(&raw_data).expect("Ingredient list file unreadable: incorrectly formatted")
}

#[tauri::command]
fn new_recipe(title: String){
    let new_rec = Recipe {
        title,
        in_mp: false,
        servings: 0.0,
        ingredients_list: Vec::new(),
        directions_list: Vec::new()
    };
    save_recipe(new_rec);
    let mut recipe_list = all_recipes();
    recipe_list.push((new_rec.title, new_rec.in_mp));
    let output_data = serde_json::to_string(&recipe_list).expect("Serialization Failed");
    fs::write(RECIPE_LIST_PATH, output_data);
}

#[tauri::command]
fn save_recipe(recipe: Recipe){
    let path = format!("{}{}{}", RECIPE_FOLDER_PATH, recipe.title, ".json");
    let output_data = serde_json::to_string(&recipe).expect("Serialization Failed");
    fs::write(&path, output_data);
}

#[tauri::command]
fn save_ingredient(ingredient: Ingredient){
    let mut ingredient_list: Vec<Ingredient> = load_ingredients();
    ingredient_list.push(ingredient);
    let output_data = serde_json::to_string(&ingredient_list).expect("Serialization Failed");
    fs::write(INGREDIENT_LIST_PATH, output_data);
}

#[tauri::command]
fn delete_ingredient(ingredient: Ingredient){
    let ingredient_list: Vec<Ingredient> = load_ingredients();
    let mut new_ing_list = Vec::new();
    for ing in ingredient_list{
        if ing.name != ingredient.name {
            new_ing_list.push(ing);
        }
    }
    let output_data = serde_json::to_string(&new_ing_list).expect("Serialization Failed");
    fs::write(INGREDIENT_LIST_PATH, output_data);
}

#[tauri::command]
fn rem_from_mp(title: String){
    let recipe = load_recipe(title);
    if recipe.in_mp {
        let mp_list = meal_plan();
        let mut new_mp_list = Vec::new();
        for rec in mp_list{
            if rec != title{
                new_mp_list.push(rec);
            }
        }
        let output_data = serde_json::to_string(&new_mp_list).expect("Serialization Failed");
        fs::write(MEALPLAN_LIST_PATH, output_data);
        let mut recipe_list = all_recipes();
        for mut rec in recipe_list{
            if rec.0 == recipe.title{
                rec.1 = false;
            }
        }
        let output_data = serde_json::to_string(&recipe_list).expect("Serialization Failed");
        fs::write(RECIPE_LIST_PATH, output_data);
    } 
}

#[tauri::command]
fn add_to_mp(title: String){
    let recipe = load_recipe(title);
    if !recipe.in_mp {
        let mut mp_list = meal_plan();
        mp_list.push(recipe.title);
        let output_data = serde_json::to_string(&mp_list).expect("Serialization Failed");
        fs::write(MEALPLAN_LIST_PATH, output_data);
        let mut recipe_list = all_recipes();
        for mut rec in recipe_list{
            if rec.0 == recipe.title{
                rec.1 = true;
            }
        }
        let output_data = serde_json::to_string(&recipe_list).expect("Serialization Failed");
        fs::write(RECIPE_LIST_PATH, output_data);
    }
}

#[tauri::command]
fn shopping_list(serving_amounts: Vec<f64>) -> Vec<(f64, Ingredient)>{
    let mp = meal_plan();
    let mut shopping_list: Vec<(f64, Ingredient)> = Vec::new();
    for count in 0..mp.len(){
        let recipe = load_recipe(mp[count]);
        for (measure, ingredient) in recipe.ingredients_list{
            let mut new_ingredient = true;
            for mut item in shopping_list{
                if item.1.name == ingredient.name{
                    item.0 += measure * serving_amounts[count] / recipe.servings;
                    new_ingredient = false;
                }
            }
            if(new_ingredient){
                shopping_list.push((measure * serving_amounts[count] / recipe.servings, ingredient));
            }
        }
    }
    shopping_list
}

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![all_recipes, load_recipe, meal_plan, 
        load_ingredients, new_recipe, save_recipe, save_ingredient, delete_ingredient, 
        shopping_list, add_to_mp, rem_from_mp, my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
