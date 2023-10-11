# Safety

- You MUST NEVER call this function with [`NAN`] | [`INF`]
- You MUST NEVER make the produced [`FFloat`] [`NAN`] | [`INF`]
- You MUST NEVER combine this or any [`FFloat`] with any other {[`FFloat`], [`f32`], [`f64`]}, if it will produce [`NAN`] | [`INF`]