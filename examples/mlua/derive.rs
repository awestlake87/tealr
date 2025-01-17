use tealr::{
    mlu::{
        mlua::{Lua, Result},
        TealData, TealDataMethods, UserData,
    },
    TypeName, TypeWalker,
};
//this example shows how the new traits allow you to generate the .d.tl file
//and shows how to use them to share data with lua
//It also shows how to generate the file
//NOTE: All it does it generate the contents of the file. Storing it is left to the user.

//First, create the struct you want to export to lua.
//instead of both deriving UserData and TypeName you can also
//derive TealDerive, which does both. However you will still need to import
//UserData and TypeName
//The clone is only needed because one of the example functions has it as a parameter
#[derive(Clone, UserData, TypeName)]
struct Example {}

//now, implement TealData. This tells rlua what methods are available and tealr what the types are
impl TealData for Example {
    //implement your methods/functions
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("example_method", |_, _, x: i8| Ok(x));
        methods.add_method_mut("example_method_mut", |_, _, x: (i8, String)| Ok(x.1));
        methods.add_function("example_function", |_, x: Vec<String>| Ok((x, 8)));
        methods.add_function_mut("example_function_mut", |_, x: (bool, Option<Example>)| {
            Ok(x)
        })
    }
}

fn main() -> Result<()> {
    //we collect the documentation of our API in a json file so `tealr_doc_gen` can generate
    //the online documentation
    let file_contents = TypeWalker::new()
        //tells it that we want to include the `Example` type
        //add more calls to process_type to generate more types in the same file
        .process_type::<Example>()
        //generate the file
        .to_json()
        .expect("serde_json failed to serialize our data");

    //normally you would now save the file somewhere.
    println!("{}\n ", file_contents);

    //how you pass this type to lua hasn't changed:
    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("test", Example {})?;
    let code = "
print(test:example_method(1))
print(test:example_method_mut(2,\"test\"))
print(test.example_function({}))
print(test.example_function_mut(true))
    ";
    lua.load(code).set_name("test?")?.eval()?;
    Ok(())
}
