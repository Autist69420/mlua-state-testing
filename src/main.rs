use mlua_state_testing::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = State::new()?;
    State::setup_lua_env(&state)?;

    state.lua.load("print(1)").exec()?;

    Ok(())
}
