use std::borrow::Cow;

/// A resource location.
///
/// See the [Minecraft wiki](https://minecraft.fandom.com/wiki/Resource_location) for details.
///
/// TODO: Maybe implement an own type for that to ensure the constraints mentioned in the docs.
pub type ResourceLocation = Cow<'static, str>;
