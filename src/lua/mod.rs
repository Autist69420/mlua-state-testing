use mlua::prelude::*;

use crate::State;

pub fn setup_lua_env(state: &State) -> mlua::Result<()> {
    let lua = &state.lua;
    let globals = lua.globals();

    globals.set("print", lua.create_function(print)?)?;

    Ok(())
}

fn print(_: &Lua, args: String) -> mlua::Result<()> {
    println!("{}", args);

    Ok(())
}