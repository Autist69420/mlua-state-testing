pub mod lua;

pub struct State {
    pub lua: mlua::Lua,
}

impl State {
    pub fn new() -> Result<State, Box<dyn std::error::Error>> {
        let lua = mlua::Lua::new();

        Ok(State { lua })
    }

    pub fn setup_lua_env(state: &State) -> Result<(), Box<dyn std::error::Error>> {
        lua::setup_lua_env(state)?;

        Ok(())
    }
}